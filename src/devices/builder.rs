use std::{
  collections::{BTreeMap, BTreeSet},
  fs::OpenOptions,
  io,
  os::raw::c_int,
  os::unix::fs::OpenOptionsExt,
  os::unix::io::AsRawFd,
};

use super::{
  InputDevice, Kind,
  raw::{ioctl_none, ioctl_val, ioctl_write},
};
use crate::{
  input::{Bus, InputAbsinfo, InputId},
  input_event_codes::{
    AbsoluteAxis, EventTypes, InputProp, Key, Led, Misc, RelativeAxis, Sound, SwitchEvent,
  },
  uinput::{
    UI_ABS_SETUP, UI_DEV_CREATE, UI_DEV_SETUP, UI_SET_ABSBIT, UI_SET_EVBIT, UI_SET_FFBIT,
    UI_SET_KEYBIT, UI_SET_LEDBIT, UI_SET_MSCBIT, UI_SET_PROPBIT, UI_SET_RELBIT, UI_SET_SNDBIT,
    UI_SET_SWBIT, UinputAbsSetup, UinputSetup,
  },
};

/// Builds a virtual device to be created through `/dev/uinput`.
///
/// Chain in the identity (`.bus()`/`.vendor()`/`.product()`/`.version()`, or
/// `.id()` to set all four at once), enable whichever event types the device
/// should support (`.event_type()`), add the individual codes within each
/// type (`.key()`, `.rel_axis()`, `.abs_axis()`, ...), then call
/// [`build`](Self::build) to open `/dev/uinput` and create the device.
#[derive(Debug, Clone)]
pub struct DeviceBuilder {
  name: String,
  id: InputId,
  ff_effects_max: u32,
  event_types: BTreeSet<u16>,
  /// `(event type code, event code)`, for every type except `EV_ABS` (which
  /// carries axis info alongside the code, see `abs`).
  codes: BTreeSet<(u16, u32)>,
  abs: BTreeMap<u32, InputAbsinfo>,
  props: BTreeSet<u32>,
}

impl DeviceBuilder {
  /// Starts building a device with the given name and a placeholder
  /// (`Bus::Virtual`, vendor `0x0001`, product `0x0001`, version `1`) identity.
  pub fn new(name: impl Into<String>) -> Self {
    DeviceBuilder {
      name: name.into(),
      id: InputId {
        bustype: Bus::Virtual.code() as u16,
        vendor: 0x0001,
        product: 0x0001,
        version: 1,
      },
      ff_effects_max: 0,
      event_types: BTreeSet::new(),
      codes: BTreeSet::new(),
      abs: BTreeMap::new(),
      props: BTreeSet::new(),
    }
  }

  /// Sets the full [`InputId`] at once - the convenient way to carry an
  /// identity read off another device over to this one (see
  /// [`InputDevice::clone_device`](super::InputDevice::clone_device)).
  pub fn id(mut self, id: InputId) -> Self {
    self.id = id;
    self
  }

  pub fn bus(mut self, bus: Bus) -> Self {
    self.id.bustype = bus.code() as u16;
    self
  }

  pub fn vendor(mut self, vendor: u16) -> Self {
    self.id.vendor = vendor;
    self
  }

  pub fn product(mut self, product: u16) -> Self {
    self.id.product = product;
    self
  }

  pub fn version(mut self, version: u16) -> Self {
    self.id.version = version;
    self
  }

  /// Limits the number of force-feedback effects the device can hold.
  pub fn ff_effects_max(mut self, max: u32) -> Self {
    self.ff_effects_max = max;
    self
  }

  /// Declares that the device emits events of this type (`UI_SET_EVBIT`).
  /// Adding a code with `.key()`/`.rel_axis()`/etc. below already implies
  /// its type, so you only need this directly for a type with no codes of
  /// its own to add.
  pub fn event_type(mut self, event_type: EventTypes) -> Self {
    self.event_types.insert(event_type.code() as u16);
    self
  }

  /// Escape hatch for a code with no named enum in
  /// [`input_event_codes`](crate::input_event_codes) (vendor-specific codes,
  /// or copying a bitmap read straight off another device). Implies
  /// `.event_type(event_type)`. For `EV_ABS` use [`abs_code`](Self::abs_code)
  /// instead, since absolute axes also carry range/fuzz/flat/resolution.
  pub fn code(mut self, event_type: EventTypes, code: u32) -> Self {
    self.event_types.insert(event_type.code() as u16);
    self.codes.insert((event_type.code() as u16, code));
    self
  }

  /// Escape hatch equivalent of [`abs_axis`](Self::abs_axis) for a raw code.
  pub fn abs_code(mut self, code: u32, info: InputAbsinfo) -> Self {
    self.event_types.insert(EventTypes::Abs.code() as u16);
    self.abs.insert(code, info);
    self
  }

  pub fn key(self, key: Key) -> Self {
    self.code(EventTypes::Key, key.code())
  }

  /// Adds several keys at once, e.g. `.keys(Key::iter())` for a full
  /// keyboard.
  pub fn keys(mut self, keys: impl IntoIterator<Item = Key>) -> Self {
    for key in keys {
      self = self.key(key);
    }
    self
  }

  pub fn rel_axis(self, axis: RelativeAxis) -> Self {
    self.code(EventTypes::Rel, axis.code())
  }

  /// Adds an absolute axis with its range/fuzz/flat/resolution.
  pub fn abs_axis(self, axis: AbsoluteAxis, info: InputAbsinfo) -> Self {
    self.abs_code(axis.code(), info)
  }

  pub fn led(self, led: Led) -> Self {
    self.code(EventTypes::Led, led.code())
  }

  pub fn sound(self, sound: Sound) -> Self {
    self.code(EventTypes::Snd, sound.code())
  }

  pub fn misc(self, misc: Misc) -> Self {
    self.code(EventTypes::Msc, misc.code())
  }

  pub fn switch(self, switch: SwitchEvent) -> Self {
    self.code(EventTypes::Sw, switch.code())
  }

  /// Sets a device property/quirk (`INPUT_PROP_*`, e.g. `InputProp::Pointer`
  /// for a device that should be treated as a mouse-like pointer).
  pub fn prop(mut self, prop: InputProp) -> Self {
    self.props.insert(prop.code());
    self
  }

  /// The `UI_SET_*BIT` ioctl that enables individual codes for a given
  /// event type, if that type has one (`EV_SYN` and `EV_REP` don't).
  fn setbit_request(event_type: u16) -> Option<crate::ioctl::Request> {
    let ev = event_type as u32;
    Some(match ev {
      _ if ev == EventTypes::Key.code() => UI_SET_KEYBIT,
      _ if ev == EventTypes::Rel.code() => UI_SET_RELBIT,
      _ if ev == EventTypes::Abs.code() => UI_SET_ABSBIT,
      _ if ev == EventTypes::Msc.code() => UI_SET_MSCBIT,
      _ if ev == EventTypes::Led.code() => UI_SET_LEDBIT,
      _ if ev == EventTypes::Snd.code() => UI_SET_SNDBIT,
      _ if ev == EventTypes::Sw.code() => UI_SET_SWBIT,
      _ if ev == EventTypes::Ff.code() => UI_SET_FFBIT,
      _ => return None,
    })
  }

  /// Opens `/dev/uinput`, applies everything configured above, and creates
  /// the device. On success the returned [`InputDevice`] owns the device:
  /// dropping it issues `UI_DEV_DESTROY`.
  ///
  /// The kernel (and anything listening for new devices, e.g. libinput)
  /// needs a moment to notice the device after this returns; sending events
  /// immediately can be dropped on the floor, so give it a brief sleep
  /// instead of sending events right away.
  pub fn build(self) -> io::Result<InputDevice> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .custom_flags(libc::O_NONBLOCK)
      .open("/dev/uinput")?;
    let fd = file.as_raw_fd();

    for &event_type in &self.event_types {
      ioctl_val(fd, UI_SET_EVBIT, event_type as c_int)?;
    }

    for &(event_type, code) in &self.codes {
      if let Some(request) = Self::setbit_request(event_type) {
        ioctl_val(fd, request, code as c_int)?;
      }
    }

    for &prop in &self.props {
      ioctl_val(fd, UI_SET_PROPBIT, prop as c_int)?;
    }

    // UI_ABS_SETUP enables the axis's bit itself; no separate UI_SET_ABSBIT
    // call is needed once this runs.
    for (&code, info) in &self.abs {
      let setup = UinputAbsSetup {
        code: code as u16,
        absinfo: *info,
      };
      ioctl_write(fd, UI_ABS_SETUP, &setup)?;
    }

    let mut setup = UinputSetup {
      id: self.id,
      ff_effects_max: self.ff_effects_max,
      ..Default::default()
    };
    setup.set_name(&self.name);
    ioctl_write(fd, UI_DEV_SETUP, &setup)?;
    ioctl_none(fd, UI_DEV_CREATE)?;

    Ok(InputDevice {
      file,
      kind: Kind::Virtual,
    })
  }
}
