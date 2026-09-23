//! Creates a virtual keyboard through /dev/uinput and types the letter "a".
//! Needs write access to /dev/uinput (root, or a udev rule / `input` group).

use std::fs::OpenOptions;
use std::io::{Error, Result, Write};
use std::os::fd::AsRawFd;
use std::os::raw::c_int;
use std::os::unix::fs::OpenOptionsExt;
use std::{slice, thread, time::Duration};

use rin::input::{Bus, InputEvent};
use rin::input_event_codes::{EventTypes, Key, Syn};
use rin::uinput::*;

fn ioctl_int(fd: c_int, request: rin::ioctl::Request, arg: c_int) -> Result<()> {
  match unsafe { libc::ioctl(fd, request, arg) } {
    -1 => Err(Error::last_os_error()),
    _ => Ok(()),
  }
}

fn send(uinput: &mut std::fs::File, ev: InputEvent) -> Result<()> {
  let bytes = unsafe {
    slice::from_raw_parts(
      &ev as *const _ as *const u8,
      std::mem::size_of::<InputEvent>(),
    )
  };
  uinput.write_all(bytes)
}

fn main() -> Result<()> {
  let mut uinput = OpenOptions::new()
    .write(true)
    .custom_flags(libc::O_NONBLOCK)
    .open("/dev/uinput")?;
  let fd = uinput.as_raw_fd();

  // Declare which events this device can produce.
  ioctl_int(fd, UI_SET_EVBIT, EventTypes::Key.code() as c_int)?;
  ioctl_int(fd, UI_SET_KEYBIT, Key::A.code() as c_int)?;

  // Describe and create it.
  let mut setup = UinputSetup::default();
  setup.id.bustype = Bus::Usb.code() as u16;
  setup.id.vendor = 0x1234;
  setup.id.product = 0x5678;
  setup.set_name("rust virtual keyboard");
  if unsafe { libc::ioctl(fd, UI_DEV_SETUP, &setup) } == -1 {
    return Err(Error::last_os_error());
  }
  if unsafe { libc::ioctl(fd, UI_DEV_CREATE) } == -1 {
    return Err(Error::last_os_error());
  }
  thread::sleep(Duration::from_secs(5)); // let userspace notice the new device

  for value in [1, 0] {
    send(
      &mut uinput,
      InputEvent::new(EventTypes::Key, Key::A.code(), value),
    )?;
    send(
      &mut uinput,
      InputEvent::new(EventTypes::Syn, Syn::Report.code(), 0),
    )?;
  }

  thread::sleep(Duration::from_secs(1));
  if unsafe { libc::ioctl(fd, UI_DEV_DESTROY) } == -1 {
    return Err(Error::last_os_error());
  }
  Ok(())
}
