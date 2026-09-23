//! Creates a virtual keyboard through /dev/uinput and types the letter "a".
//! Needs write access to /dev/uinput (root, or a udev rule / `input` group).
//!
//! The same device as `virtual_keyboard.rs`, but built with
//! `rin::devices` instead of talking to the ioctls directly
//! See how the abstraction simplifies usage.

use std::io::Result;
use std::{thread, time::Duration};

use rin::devices::InputDevice;
use rin::input_event_codes::Key;

fn main() -> Result<()> {
  let mut keyboard = InputDevice::builder("rust virtual keyboard")
    .key(Key::A)
    .build()?;

  thread::sleep(Duration::from_secs(5)); // let userspace notice the new device

  keyboard.press_key(Key::A)?; // sends value 1, then value 0, each synced

  thread::sleep(Duration::from_secs(1));
  Ok(()) // dropping `keyboard` issues UI_DEV_DESTROY
}
