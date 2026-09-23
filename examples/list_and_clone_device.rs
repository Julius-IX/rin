//! Lists the devices under /dev/input, grabs the first one it can open,
//! and creates a virtual clone of it. Grab the real device so its events stop reaching anyone else, clone it,
//! then (not shown here) forward transformed events to the clone.
//! Needs read/write access to /dev/input/eventN and /dev/uinput.

use std::io::Result;
use std::{thread, time::Duration};

use rin::devices::InputDevice;

fn main() -> Result<()> {
  let devices = InputDevice::query_devices()?;
  if devices.is_empty() {
    eprintln!("no devices found under /dev/input (or none were readable)");
    return Ok(());
  }

  println!("found {} device(s):", devices.len());
  for info in &devices {
    println!("  {}  {:?}  {}", info.path.display(), info.id, info.name);
  }

  let chosen = &devices[0];
  println!("\ngrabbing and cloning: {}", chosen.name);

  let original = InputDevice::open(&chosen.path)?;

  let clone_name = format!("{} (rin clone)", chosen.name);
  let _clone = original.clone_device(&clone_name)?;
  println!("created virtual clone: {clone_name}");

  original.grab()?; // events now reach only this fd, not the rest of the desktop

  thread::sleep(Duration::from_secs(3)); // hold the grab briefly to show it has been grabbed

  original.ungrab()?;
  Ok(()) // dropping `_clone` issues UI_DEV_DESTROY; `original` just closes
}
