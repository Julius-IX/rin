//! The `UI_*` ioctls of `uinput.h`.

use std::os::raw::{c_char, c_int, c_uint};

use super::{UinputAbsSetup, UinputFfErase, UinputFfUpload, UinputSetup};
use crate::ioctl::{io, ioc, ior, iow, iowr, Request, IOC_READ};

pub const UINPUT_IOCTL_BASE: u8 = b'U';

pub const UI_DEV_CREATE: Request = io(UINPUT_IOCTL_BASE, 1);
pub const UI_DEV_DESTROY: Request = io(UINPUT_IOCTL_BASE, 2);

/// Set device parameters for setup. Takes a [`UinputSetup`].
pub const UI_DEV_SETUP: Request = iow::<UinputSetup>(UINPUT_IOCTL_BASE, 3);

/// Set absolute axis information for the device to setup. Takes a [`UinputAbsSetup`].
pub const UI_ABS_SETUP: Request = iow::<UinputAbsSetup>(UINPUT_IOCTL_BASE, 4);

pub const UI_SET_EVBIT: Request = iow::<c_int>(UINPUT_IOCTL_BASE, 100);
pub const UI_SET_KEYBIT: Request = iow::<c_int>(UINPUT_IOCTL_BASE, 101);
pub const UI_SET_RELBIT: Request = iow::<c_int>(UINPUT_IOCTL_BASE, 102);
pub const UI_SET_ABSBIT: Request = iow::<c_int>(UINPUT_IOCTL_BASE, 103);
pub const UI_SET_MSCBIT: Request = iow::<c_int>(UINPUT_IOCTL_BASE, 104);
pub const UI_SET_LEDBIT: Request = iow::<c_int>(UINPUT_IOCTL_BASE, 105);
pub const UI_SET_SNDBIT: Request = iow::<c_int>(UINPUT_IOCTL_BASE, 106);
pub const UI_SET_FFBIT: Request = iow::<c_int>(UINPUT_IOCTL_BASE, 107);
pub const UI_SET_PHYS: Request = iow::<*const c_char>(UINPUT_IOCTL_BASE, 108);
pub const UI_SET_SWBIT: Request = iow::<c_int>(UINPUT_IOCTL_BASE, 109);
pub const UI_SET_PROPBIT: Request = iow::<c_int>(UINPUT_IOCTL_BASE, 110);

pub const UI_BEGIN_FF_UPLOAD: Request = iowr::<UinputFfUpload>(UINPUT_IOCTL_BASE, 200);
pub const UI_END_FF_UPLOAD: Request = iow::<UinputFfUpload>(UINPUT_IOCTL_BASE, 201);
pub const UI_BEGIN_FF_ERASE: Request = iowr::<UinputFfErase>(UINPUT_IOCTL_BASE, 202);
pub const UI_END_FF_ERASE: Request = iow::<UinputFfErase>(UINPUT_IOCTL_BASE, 203);

/// `UI_GET_SYSNAME` - get the sysfs name of the created uinput device
///
/// Returns (into a buffer of `len` bytes) the sysfs name of the created
/// virtual input device. The complete sysfs path is then
/// `/sys/devices/virtual/input/--NAME--`. Usually, it is in the form "inputN".
pub const fn ui_get_sysname(len: usize) -> Request {
  ioc(IOC_READ, UINPUT_IOCTL_BASE, 44, len)
}

/// `UI_GET_VERSION` - Return version of uinput protocol
///
/// This writes uinput protocol version implemented by the kernel into
/// the integer pointed to by the ioctl argument. The protocol version
/// is hard-coded in the kernel and is independent of the uinput device.
pub const UI_GET_VERSION: Request = ior::<c_uint>(UINPUT_IOCTL_BASE, 45);

// To write a force-feedback-capable driver, the upload_effect
// and erase_effect callbacks in input_dev must be implemented.
// The uinput driver will generate a fake input event when one of
// these callbacks are invoked. The userspace code then uses
// ioctls to retrieve additional parameters and send the return code.
// The callback blocks until this return code is sent.
//
// The described callback mechanism is only used if ff_effects_max
// is set.
//
// To implement upload_effect():
//   1. Wait for an event with type == EV_UINPUT and code == UI_FF_UPLOAD.
//      A request ID will be given in 'value'.
//   2. Allocate a UinputFfUpload, fill in request_id with
//      the 'value' from the EV_UINPUT event.
//   3. Issue a UI_BEGIN_FF_UPLOAD ioctl, giving it the
//      UinputFfUpload. It will be filled in with the
//      FfEffects passed to upload_effect().
//   4. Perform the effect upload, and place a return code back into
//      the UinputFfUpload.
//   5. Issue a UI_END_FF_UPLOAD ioctl, also giving it the
//      UinputFfUpload. This will complete execution
//      of our upload_effect() handler.
//
// To implement erase_effect():
//   1. Wait for an event with type == EV_UINPUT and code == UI_FF_ERASE.
//      A request ID will be given in 'value'.
//   2. Allocate a UinputFfErase, fill in request_id with
//      the 'value' from the EV_UINPUT event.
//   3. Issue a UI_BEGIN_FF_ERASE ioctl, giving it the
//      UinputFfErase. It will be filled in with the
//      effect ID passed to erase_effect().
//   4. Perform the effect erasure, and place a return code back
//      into the UinputFfErase.
//   5. Issue a UI_END_FF_ERASE ioctl, also giving it the
//      UinputFfErase. This will complete execution
//      of our erase_effect() handler.
