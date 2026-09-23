use std::{fs, io, path::PathBuf};

use super::InputDevice;
use crate::input::InputId;

/// A summary of a device found under `/dev/input`, returned by
/// [`InputDevice::query_devices`].
#[derive(Debug, Clone)]
pub struct DeviceInfo {
  pub path: PathBuf,
  pub name: String,
  pub id: InputId,
}

/// Lists the `eventN` devices under `/dev/input`, opening each briefly to
/// read its name and [`InputId`]. Devices that can't be opened (typically a
/// permissions issue - see [`crate::permissions`]) are skipped rather than
/// failing the whole scan.
pub(super) fn query_devices() -> io::Result<Vec<DeviceInfo>> {
  let mut devices = Vec::new();

  for entry in fs::read_dir("/dev/input")? {
    let path = entry?.path();
    let is_event_device = path
      .file_name()
      .and_then(|name| name.to_str())
      .is_some_and(|name| name.starts_with("event"));
    if !is_event_device {
      continue;
    }

    let Ok(device) = InputDevice::open(&path) else {
      continue;
    };
    devices.push(DeviceInfo {
      name: device.name().unwrap_or_default(),
      id: device.id().unwrap_or_default(),
      path,
    });
  }

  devices.sort_by(|a, b| a.path.cmp(&b.path));
  Ok(devices)
}
