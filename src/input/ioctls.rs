//! The `EVIOC*` ioctls of `input.h`.
//!
//! Fixed requests are `const`s; the ones the C header defines as
//! function-like macros are `const fn`s.
//!
//! `IOCTLS (0x00 - 0x7f)`

use std::os::raw::{c_int, c_uint};

use super::{FfEffect, InputAbsinfo, InputId, InputKeymapEntry, InputMask};
use crate::ioctl::{ioc, ior, iow, Request, IOC_READ};

/// get driver version
pub const EVIOCGVERSION: Request = ior::<c_int>(b'E', 0x01);
/// get device ID
pub const EVIOCGID: Request = ior::<InputId>(b'E', 0x02);
/// get repeat settings
pub const EVIOCGREP: Request = ior::<[c_uint; 2]>(b'E', 0x03);
/// set repeat settings
pub const EVIOCSREP: Request = iow::<[c_uint; 2]>(b'E', 0x03);

/// get keycode
pub const EVIOCGKEYCODE: Request = ior::<[c_uint; 2]>(b'E', 0x04);
pub const EVIOCGKEYCODE_V2: Request = ior::<InputKeymapEntry>(b'E', 0x04);
/// set keycode
pub const EVIOCSKEYCODE: Request = iow::<[c_uint; 2]>(b'E', 0x04);
pub const EVIOCSKEYCODE_V2: Request = iow::<InputKeymapEntry>(b'E', 0x04);

/// get device name
pub const fn eviocgname(len: usize) -> Request {
  ioc(IOC_READ, b'E', 0x06, len)
}
/// get physical location
pub const fn eviocgphys(len: usize) -> Request {
  ioc(IOC_READ, b'E', 0x07, len)
}
/// get unique identifier
pub const fn eviocguniq(len: usize) -> Request {
  ioc(IOC_READ, b'E', 0x08, len)
}
/// get device properties
pub const fn eviocgprop(len: usize) -> Request {
  ioc(IOC_READ, b'E', 0x09, len)
}

/// `EVIOCGMTSLOTS(len)` - get MT slot values
///
/// `len` is the size of the data buffer in bytes.
///
/// The ioctl buffer argument should be binary equivalent to
///
/// ```c
/// struct input_mt_request_layout {
///   __u32 code;
///   __s32 values[num_slots];
/// };
/// ```
///
/// where `num_slots` is the (arbitrary) number of MT slots to extract.
///
/// The ioctl size argument (`len`) is the size of the buffer, which
/// should satisfy `len = (num_slots + 1) * sizeof(__s32)`. If `len` is
/// too small to fit all available slots, the first `num_slots` are
/// returned.
///
/// Before the call, `code` is set to the wanted `ABS_MT` event type. On
/// return, `values[]` is filled with the slot values for the specified
/// `ABS_MT` code.
///
/// If the request code is not an `ABS_MT` value, `-EINVAL` is returned.
pub const fn eviocgmtslots(len: usize) -> Request {
  ioc(IOC_READ, b'E', 0x0a, len)
}

/// get global key state
pub const fn eviocgkey(len: usize) -> Request {
  ioc(IOC_READ, b'E', 0x18, len)
}
/// get all LEDs
pub const fn eviocgled(len: usize) -> Request {
  ioc(IOC_READ, b'E', 0x19, len)
}
/// get all sounds status
pub const fn eviocgsnd(len: usize) -> Request {
  ioc(IOC_READ, b'E', 0x1a, len)
}
/// get all switch states
pub const fn eviocgsw(len: usize) -> Request {
  ioc(IOC_READ, b'E', 0x1b, len)
}

/// get event bits
///
/// `ev` is an event type code, e.g. `EventTypes::Key.code()`.
pub const fn eviocgbit(ev: u32, len: usize) -> Request {
  ioc(IOC_READ, b'E', 0x20 + ev as u8, len)
}
/// get abs value/limits
///
/// `abs` is an absolute axis code, e.g. `AbsoluteAxis::X.code()`.
pub const fn eviocgabs(abs: u32) -> Request {
  ior::<InputAbsinfo>(b'E', 0x40 + abs as u8)
}
/// set abs value/limits
///
/// `abs` is an absolute axis code, e.g. `AbsoluteAxis::X.code()`.
pub const fn eviocsabs(abs: u32) -> Request {
  iow::<InputAbsinfo>(b'E', 0xc0 + abs as u8)
}

/// send a force effect to a force feedback device
pub const EVIOCSFF: Request = iow::<FfEffect>(b'E', 0x80);
/// Erase a force effect
pub const EVIOCRMFF: Request = iow::<c_int>(b'E', 0x81);
/// Report number of effects playable at the same time
pub const EVIOCGEFFECTS: Request = ior::<c_int>(b'E', 0x84);

/// Grab/Release device
pub const EVIOCGRAB: Request = iow::<c_int>(b'E', 0x90);
/// Revoke device access
pub const EVIOCREVOKE: Request = iow::<c_int>(b'E', 0x91);

/// `EVIOCGMASK` - Retrieve current event mask
///
/// This ioctl allows user to retrieve the current event mask for specific
/// event type. The argument must be of type [`InputMask`] and
/// specifies the event type to query, the address of the receive buffer and
/// the size of the receive buffer.
///
/// The event mask is a per-client mask that specifies which events are
/// forwarded to the client. Each event code is represented by a single bit
/// in the event mask. If the bit is set, the event is passed to the client
/// normally. Otherwise, the event is filtered and will never be queued on
/// the client's receive buffer.
///
/// Event masks do not affect global state of the input device. They only
/// affect the file descriptor they are applied to.
///
/// The default event mask for a client has all bits set, i.e. all events
/// are forwarded to the client. If the kernel is queried for an unknown
/// event type or if the receive buffer is larger than the number of
/// event codes known to the kernel, the kernel returns all zeroes for those
/// codes.
///
/// At maximum, `codes_size` bytes are copied.
///
/// This ioctl may fail with `ENODEV` in case the file is revoked, `EFAULT`
/// if the receive-buffer points to invalid memory, or `EINVAL` if the kernel
/// does not implement the ioctl.
pub const EVIOCGMASK: Request = ior::<InputMask>(b'E', 0x92);

/// `EVIOCSMASK` - Set event mask
///
/// This ioctl is the counterpart to [`EVIOCGMASK`]. Instead of receiving the
/// current event mask, this changes the client's event mask for a specific
/// type. See [`EVIOCGMASK`] for a description of event-masks and the
/// argument-type.
///
/// This ioctl provides full forward compatibility. If the passed event type
/// is unknown to the kernel, or if the number of event codes specified in
/// the mask is bigger than what is known to the kernel, the ioctl is still
/// accepted and applied. However, any unknown codes are left untouched and
/// stay cleared. That means, the kernel always filters unknown codes
/// regardless of what the client requests. If the new mask doesn't cover
/// all known event-codes, all remaining codes are automatically cleared and
/// thus filtered.
///
/// This ioctl may fail with `ENODEV` in case the file is revoked. `EFAULT` is
/// returned if the receive-buffer points to invalid memory. `EINVAL` is returned
/// if the kernel does not implement the ioctl.
pub const EVIOCSMASK: Request = iow::<InputMask>(b'E', 0x93);

/// Set clockid to be used for timestamps
pub const EVIOCSCLOCKID: Request = iow::<c_int>(b'E', 0xa0);
