//! Small `libc::ioctl` wrappers shared by the rest of the `device` module.
//!
//! Not part of the C header translation (see [`crate::ioctl`] for that) -
//! just the three calling conventions [`UI_*`](crate::uinput) / [`EVIOC*`](crate::input::ioctls)
//! requests actually use: no argument, an integer passed by value, and a
//! pointer to a struct/buffer.

use std::{io, os::raw::c_int, os::unix::io::RawFd};

use crate::ioctl::Request;

/// `ioctl(fd, request)` - no argument (e.g. `UI_DEV_CREATE`).
pub(super) fn ioctl_none(fd: RawFd, request: Request) -> io::Result<()> {
  match unsafe { libc::ioctl(fd, request) } {
    -1 => Err(io::Error::last_os_error()),
    _ => Ok(()),
  }
}

/// `ioctl(fd, request, value)` - integer passed directly, not by pointer
/// (e.g. `UI_SET_EVBIT`, `EVIOCGRAB`).
pub(super) fn ioctl_val(fd: RawFd, request: Request, value: c_int) -> io::Result<()> {
  match unsafe { libc::ioctl(fd, request, value) } {
    -1 => Err(io::Error::last_os_error()),
    _ => Ok(()),
  }
}

/// `ioctl(fd, request, &value)` - write a struct into the kernel
/// (e.g. `UI_DEV_SETUP`, `UI_ABS_SETUP`).
pub(super) fn ioctl_write<T>(fd: RawFd, request: Request, value: &T) -> io::Result<()> {
  match unsafe { libc::ioctl(fd, request, value as *const T) } {
    -1 => Err(io::Error::last_os_error()),
    _ => Ok(()),
  }
}

/// `ioctl(fd, request, &mut value)` - read a struct out of the kernel
/// (e.g. `EVIOCGID`, `EVIOCGABS`). Returns the raw return value, which some
/// requests (buffer reads) use to report how much was written.
pub(super) fn ioctl_read<T>(fd: RawFd, request: Request, value: &mut T) -> io::Result<c_int> {
  match unsafe { libc::ioctl(fd, request, value as *mut T) } {
    -1 => Err(io::Error::last_os_error()),
    ret => Ok(ret),
  }
}

/// `ioctl(fd, request, buf.as_mut_ptr())` - read a variable-length byte
/// buffer out of the kernel (e.g. `EVIOCGNAME`, `EVIOCGBIT`).
pub(super) fn ioctl_read_buf(fd: RawFd, request: Request, buf: &mut [u8]) -> io::Result<c_int> {
  match unsafe { libc::ioctl(fd, request, buf.as_mut_ptr()) } {
    -1 => Err(io::Error::last_os_error()),
    ret => Ok(ret),
  }
}
