//! Creates a virtual mouse through /dev/uinput, nudges the cursor
//! diagonally, then clicks the left button.
//! Needs write access to /dev/uinput (root, or a udev rule / `input` group).

use std::io::Result;
use std::{thread, time::Duration};

use rin::devices::{InputDevice, KeyState};
use rin::input_event_codes::{EventTypes, Key, RelativeAxis};

fn main() -> Result<()> {
  let mut mouse = InputDevice::builder("rust virtual mouse")
    .rel_axis(RelativeAxis::X)
    .rel_axis(RelativeAxis::Y)
    .key(Key::BtnLeft)
    .key(Key::BtnRight)
    .build()?;

  thread::sleep(Duration::from_secs(2)); // let userspace notice the new device

  // Move 20px right, 10px down as a single report: queue both axes, then
  // sync once so they land together instead of as two separate motions.
  mouse.queue_event(EventTypes::Rel, RelativeAxis::X.code(), 20)?;
  mouse.queue_event(EventTypes::Rel, RelativeAxis::Y.code(), 10)?;
  mouse.sync()?;

  thread::sleep(Duration::from_millis(200));

  // A left click: down then up, exactly like press_key but for a button.
  mouse.send_key(Key::BtnLeft, KeyState::Down)?;
  thread::sleep(Duration::from_millis(50));
  mouse.send_key(Key::BtnLeft, KeyState::Up)?;

  thread::sleep(Duration::from_secs(1));
  Ok(())
}
