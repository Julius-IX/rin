use std::mem::zeroed;
use std::os::raw::c_char;

use super::{ABS_CNT, UINPUT_MAX_NAME_SIZE};
use crate::input::{FfEffect, InputAbsinfo, InputId};

/// Copies `name` into a fixed-size, NUL-terminated C buffer, truncating it
/// (on a byte boundary) if it doesn't fit.
///
/// Not part of the C header; a convenience for filling in `name` fields.
pub fn copy_name(dst: &mut [c_char; UINPUT_MAX_NAME_SIZE], name: &str) {
  dst.fill(0);
  for (d, s) in dst.iter_mut().take(UINPUT_MAX_NAME_SIZE - 1).zip(name.bytes()) {
    *d = s as c_char;
  }
}

/// `struct uinput_ff_upload`
///
/// See [`UI_BEGIN_FF_UPLOAD`](super::UI_BEGIN_FF_UPLOAD).
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct UinputFfUpload {
  pub request_id: u32,
  pub retval: i32,
  pub effect: FfEffect,
  pub old: FfEffect,
}

/// `struct uinput_ff_erase`
///
/// See [`UI_BEGIN_FF_ERASE`](super::UI_BEGIN_FF_ERASE).
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct UinputFfErase {
  pub request_id: u32,
  pub retval: i32,
  pub effect_id: u32,
}

/// `struct uinput_setup` - argument of [`UI_DEV_SETUP`](super::UI_DEV_SETUP)
///
/// This sets parameters for the input device to be created. It supersedes the
/// old [`UinputUserDev`] method, which wrote this data via `write()`. To
/// actually set the absolute axes `UI_ABS_SETUP` should be used.
///
/// This ioctl can be called multiple times and will overwrite previous values.
/// If this ioctl fails with `-EINVAL`, it is recommended to use the old
/// [`UinputUserDev`] method via `write()` as a fallback, in case you run on an
/// old kernel that does not support this ioctl.
///
/// This ioctl may fail with `-EINVAL` if it is not supported or if you passed
/// incorrect values, `-ENOMEM` if the kernel runs out of memory or `-EFAULT` if the
/// passed [`UinputSetup`] object cannot be read/written.
/// If this call fails, partial data may have already been applied to the
/// internal device.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UinputSetup {
  /// See the description of [`InputId`]. This field is copied unchanged into
  /// the new device.
  pub id: InputId,
  /// This is used unchanged as name for the new device. See [`copy_name`].
  pub name: [c_char; UINPUT_MAX_NAME_SIZE],
  /// This limits the maximum numbers of force-feedback effects. See the
  /// module docs of the C header for a description of FF with uinput.
  pub ff_effects_max: u32,
}

impl Default for UinputSetup {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

impl UinputSetup {
  /// Sets [`name`](Self::name) from a string (see [`copy_name`]).
  pub fn set_name(&mut self, name: &str) {
    copy_name(&mut self.name, name);
  }
}

/// `struct uinput_abs_setup` - argument of [`UI_ABS_SETUP`](super::UI_ABS_SETUP)
///
/// This sets one absolute axis information for the input device to be
/// created. It supersedes the old [`UinputUserDev`] method, which wrote
/// part of this data and the content of `UI_DEV_SETUP` via `write()`.
///
/// This ioctl can be called multiple times and will overwrite previous values.
/// If this ioctl fails with `-EINVAL`, it is recommended to use the old
/// [`UinputUserDev`] method via `write()` as a fallback, in case you run on an
/// old kernel that does not support this ioctl.
///
/// This ioctl may fail with `-EINVAL` if it is not supported or if you passed
/// incorrect values, `-ENOMEM` if the kernel runs out of memory or `-EFAULT` if the
/// passed [`UinputAbsSetup`] object cannot be read/written.
/// If this call fails, partial data may have already been applied to the
/// internal device.
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct UinputAbsSetup {
  /// The corresponding input code associated with this axis (`ABS_X`,
  /// `ABS_Y`, etc...), e.g. `AbsoluteAxis::X.code() as u16`.
  pub code: u16,
  // __u16 filler; (implicit padding from #[repr(C)])
  /// See [`InputAbsinfo`] for a description of this field. This field is
  /// copied unchanged into the kernel for the specified axis. If the axis is
  /// not enabled via `UI_SET_ABSBIT`, this ioctl will enable it.
  pub absinfo: InputAbsinfo,
}

/// `struct uinput_user_dev` the legacy way of setting up a device, by
/// `write()`ing this struct to the uinput file descriptor.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UinputUserDev {
  pub name: [c_char; UINPUT_MAX_NAME_SIZE],
  pub id: InputId,
  pub ff_effects_max: u32,
  pub absmax: [i32; ABS_CNT],
  pub absmin: [i32; ABS_CNT],
  pub absfuzz: [i32; ABS_CNT],
  pub absflat: [i32; ABS_CNT],
}

impl Default for UinputUserDev {
  fn default() -> Self {
    unsafe { zeroed() }
  }
}

impl UinputUserDev {
  /// Sets [`name`](Self::name) from a string (see [`copy_name`]).
  pub fn set_name(&mut self, name: &str) {
    copy_name(&mut self.name, name);
  }
}
