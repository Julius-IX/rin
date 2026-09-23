//! A friendlier layer over [`uinput`](crate::uinput) and the [`EVIOC*` ioctls](crate::input::ioctls)
//! for the common case: open or build a device, optionally grab it, and send or read events on it.
//!
//! [`InputDevice`] wraps either kind of device the raw modules deal with:
//! - a **physical** device opened from `/dev/input/eventN` ([`open`](InputDevice::open)),
//!   which you can [`grab`](InputDevice::grab) and inspect, or
//! - a **virtual** device created through `/dev/uinput` ([`InputDevice::builder`]),
//!   which you drive by sending events and which is torn down
//!   (`UI_DEV_DESTROY`) when dropped.
//!
//! ```no_run
//! use rin::device::InputDevice;
//! use rin::input_event_codes::Key;
//!
//! # fn main() -> std::io::Result<()> {
//! let mut keyboard = InputDevice::builder("rust virtual keyboard")
//!   .key(Key::A)
//!   .build()?;
//!
//! std::thread::sleep(std::time::Duration::from_millis(200)); // let userspace notice it
//! keyboard.press_key(Key::A)?; // sends value 1, then value 0
//! # Ok(())
//! # }
//! ```

mod builder;
mod info;
mod raw;

pub use builder::DeviceBuilder;
pub use info::DeviceInfo;

use std::{
  fs::OpenOptions,
  io::{self, Write},
  mem,
  os::unix::fs::OpenOptionsExt,
  os::unix::io::{AsRawFd, RawFd},
  path::Path,
  slice,
};

use raw::{ioctl_none, ioctl_read, ioctl_read_buf, ioctl_val};

use crate::{
  input::ioctls::{EVIOCGID, EVIOCGRAB, eviocgabs, eviocgbit, eviocgname, eviocgprop},
  input::{InputAbsinfo, InputEvent, InputId},
  input_event_codes::{
    AbsoluteAxis, EventTypes, InputProp, Key, Led, Misc, RelativeAxis, Sound, SwitchEvent, Syn,
  },
  uinput::UI_DEV_DESTROY,
};

/// The state of a key or button, for [`InputDevice::send_key`].
///
/// This is just `value` for an `EV_KEY` event, named the way the kernel
/// describes it: `0` releases, `1` presses, `2` is an autorepeat.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyState {
  Up,
  Down,
  Repeat,
}

impl KeyState {
  pub fn value(&self) -> i32 {
    match self {
      KeyState::Up => 0,
      KeyState::Down => 1,
      KeyState::Repeat => 2,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Kind {
  /// Opened from `/dev/input/eventN`.
  Physical,
  /// Created through `/dev/uinput`; destroyed on drop.
  Virtual,
}

/// A physical or virtual input device. See the [module docs](self) for the
/// difference and how each kind is typically obtained.
#[derive(Debug)]
pub struct InputDevice {
  file: std::fs::File,
  kind: Kind,
}

impl InputDevice {
  /// Opens an existing device node, e.g. `/dev/input/event3`. Needs
  /// read/write access to the node (see [`crate::permissions`]).
  pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .custom_flags(libc::O_NONBLOCK)
      .open(path)?;
    Ok(InputDevice {
      file,
      kind: Kind::Physical,
    })
  }

  /// Starts a [`DeviceBuilder`] for a new virtual device with the given
  /// name. Call [`.build()`](DeviceBuilder::build) on it once it is fully configured
  pub fn builder(name: impl Into<String>) -> DeviceBuilder {
    DeviceBuilder::new(name)
  }

  /// Lists the devices currently available under `/dev/input`.
  pub fn query_devices() -> io::Result<Vec<DeviceInfo>> {
    info::query_devices()
  }

  fn fd(&self) -> RawFd {
    self.file.as_raw_fd()
  }

  /// Reads the device's name (`EVIOCGNAME`).
  pub fn name(&self) -> io::Result<String> {
    let mut buf = [0u8; 256];
    ioctl_read_buf(self.fd(), eviocgname(buf.len()), &mut buf)?;
    let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    Ok(String::from_utf8_lossy(&buf[..len]).into_owned())
  }

  /// Reads the device's bus type/vendor/product/version (`EVIOCGID`).
  pub fn id(&self) -> io::Result<InputId> {
    let mut id = InputId::default();
    ioctl_read(self.fd(), EVIOCGID, &mut id)?;
    Ok(id)
  }

  /// Grabs the device (`EVIOCGRAB`, arg `1`): while grabbed, only this file
  /// descriptor receives its events - they stop reaching every other reader
  /// (including the rest of the desktop). Only meaningful on a physical
  /// device have been [`open`](Self::open)ed.
  pub fn grab(&self) -> io::Result<()> {
    ioctl_val(self.fd(), EVIOCGRAB, 1)
  }

  /// Releases a grab taken with [`grab`](Self::grab).
  pub fn ungrab(&self) -> io::Result<()> {
    ioctl_val(self.fd(), EVIOCGRAB, 0)
  }

  /// Reads this device's capabilities (supported event types, codes within
  /// each, absolute-axis ranges, and properties) and creates a new virtual
  /// device through `/dev/uinput` that advertises the same ones, under
  /// `name`, with this device's [`InputId`].
  pub fn clone_device(&self, name: impl Into<String>) -> io::Result<InputDevice> {
    let fd = self.fd();
    let mut builder = DeviceBuilder::new(name).id(self.id()?);

    // ev = 0 is the kernel's special case for "which event types (EV_KEY,
    // EV_REL, ...) does this device support at all".
    let supported_types = read_bits(fd, 0, EventTypes::Cnt.code())?;
    for &event_type in &[
      EventTypes::Key,
      EventTypes::Rel,
      EventTypes::Abs,
      EventTypes::Msc,
      EventTypes::Led,
      EventTypes::Snd,
      EventTypes::Sw,
      EventTypes::Ff,
    ] {
      if !bit_set(&supported_types, event_type.code()) {
        continue;
      }
      builder = builder.event_type(event_type);

      if event_type == EventTypes::Abs {
        let abs_bits = read_bits(fd, event_type.code(), AbsoluteAxis::Cnt.code())?;
        for code in 0..AbsoluteAxis::Cnt.code() {
          if !bit_set(&abs_bits, code) {
            continue;
          }
          let mut info = InputAbsinfo::default();
          ioctl_read(fd, eviocgabs(code), &mut info)?;
          builder = builder.abs_code(code, info);
        }
        continue;
      }

      let cnt = match event_type {
        EventTypes::Key => Key::Cnt.code(),
        EventTypes::Rel => RelativeAxis::Cnt.code(),
        EventTypes::Msc => Misc::Cnt.code(),
        EventTypes::Led => Led::Cnt.code(),
        EventTypes::Snd => Sound::Cnt.code(),
        EventTypes::Sw => SwitchEvent::Cnt.code(),
        EventTypes::Ff => crate::input::FF_CNT,
        _ => continue,
      };
      let type_bits = read_bits(fd, event_type.code(), cnt)?;
      for code in 0..cnt {
        if bit_set(&type_bits, code) {
          builder = builder.code(event_type, code);
        }
      }
    }

    let mut prop_buf = vec![0u8; (InputProp::Cnt.code() as usize).div_ceil(8)];
    ioctl_read_buf(fd, eviocgprop(prop_buf.len()), &mut prop_buf)?;
    for code in 0..InputProp::Cnt.code() {
      if bit_set(&prop_buf, code) {
        if let Some(prop) = input_prop_from_code(code) {
          builder = builder.prop(prop);
        }
      }
    }

    builder.build()
  }

  fn write_raw(&mut self, event: InputEvent) -> io::Result<()> {
    let bytes = unsafe {
      slice::from_raw_parts(
        &event as *const InputEvent as *const u8,
        mem::size_of::<InputEvent>(),
      )
    };
    self.file.write_all(bytes)
  }

  /// Writes an `EV_SYN` / `SYN_REPORT`, marking the end of a batch of
  /// events for readers. [`send_event`](Self::send_event) already automatically does this.
  /// Use this together with [`queue_event`](Self::queue_event) when several events (e.g.
  /// `REL_X` and `REL_Y`) need to land as one atomic report.
  pub fn sync(&mut self) -> io::Result<()> {
    self.write_raw(InputEvent::new(EventTypes::Syn, Syn::Report.code(), 0))
  }

  /// Queues a single event without following it with `SYN_REPORT`. Call
  /// [`sync`](Self::sync) once all events that should land together are queued.
  pub fn queue_event(&mut self, event_type: EventTypes, code: u32, value: i32) -> io::Result<()> {
    self.write_raw(InputEvent::new(event_type, code, value))
  }

  /// Sends a single event with an explicit state/value (`0`, `1`, `2` for
  /// keys; a magnitude for absolute/relative axes; ...), followed by
  /// `SYN_REPORT` so it actually takes effect. For a full "press and
  /// release" key event use [`press_key`](Self::press_key) instead.
  pub fn send_event(&mut self, event_type: EventTypes, code: u32, value: i32) -> io::Result<()> {
    self.queue_event(event_type, code, value)?;
    self.sync()
  }

  /// Sends a key event with an explicit [`KeyState`] (down, up, or repeat).
  pub fn send_key(&mut self, key: Key, state: KeyState) -> io::Result<()> {
    self.send_event(EventTypes::Key, key.code(), state.value())
  }

  /// Convenience for a full keypress: sends the key down (`1`), then up
  /// (`0`), each followed by its own `SYN_REPORT`.
  /// For just one half of that (e.g. holding a modifier) use [`send_key`](Self::send_key).
  pub fn press_key(&mut self, key: Key) -> io::Result<()> {
    self.send_key(key, KeyState::Down)?;
    self.send_key(key, KeyState::Up)
  }
}

impl Drop for InputDevice {
  fn drop(&mut self) {
    if matches!(self.kind, Kind::Virtual) {
      let _ = ioctl_none(self.fd(), UI_DEV_DESTROY);
    }
  }
}

impl AsRawFd for InputDevice {
  fn as_raw_fd(&self) -> RawFd {
    self.fd()
  }
}

// bitmap helpers used by clone_device

fn read_bits(fd: RawFd, event_type: u32, cnt: u32) -> io::Result<Vec<u8>> {
  let mut buf = vec![0u8; (cnt as usize).div_ceil(8)];
  ioctl_read_buf(fd, eviocgbit(event_type, buf.len()), &mut buf)?;
  Ok(buf)
}

fn bit_set(buf: &[u8], code: u32) -> bool {
  let (byte, bit) = (code as usize / 8, code as usize % 8);
  buf.get(byte).is_some_and(|b| b & (1 << bit) != 0)
}

fn input_prop_from_code(code: u32) -> Option<InputProp> {
  Some(match code {
    0x00 => InputProp::Pointer,
    0x01 => InputProp::Direct,
    0x02 => InputProp::ButtonPad,
    0x03 => InputProp::SemiMt,
    0x04 => InputProp::TopButtonpad,
    0x05 => InputProp::PointingStick,
    0x06 => InputProp::Accelerometer,
    0x07 => InputProp::Pressurepad,
    _ => return None,
  })
}
