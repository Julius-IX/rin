//! Linux `_IOC` request-number encoding, as found in `asm-generic/ioctl.h`
//! (and the few architecture-specific overrides).
//!
//! The kernel headers implement these as preprocessor macros, which is what
//! `input.h` and `uinput.h` build on. Here they are `const fn`s so that the
//! request numbers can still be `const`s.

use std::mem::size_of;

/// The type of the `request` argument of [`libc::ioctl`] on the current target
/// (`c_ulong` on glibc, `c_int` on musl).
pub type Request = libc::Ioctl;

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;

// mips, powerpc and sparc use a different split of the upper bits than the
// generic layout.
#[cfg(any(
  target_arch = "mips",
  target_arch = "mips32r6",
  target_arch = "mips64",
  target_arch = "mips64r6",
  target_arch = "powerpc",
  target_arch = "powerpc64",
  target_arch = "sparc",
  target_arch = "sparc64",
))]
mod arch {
  pub const IOC_SIZEBITS: u32 = 13;
  pub const IOC_NONE: u32 = 1;
  pub const IOC_READ: u32 = 2;
  pub const IOC_WRITE: u32 = 4;
}

#[cfg(not(any(
  target_arch = "mips",
  target_arch = "mips32r6",
  target_arch = "mips64",
  target_arch = "mips64r6",
  target_arch = "powerpc",
  target_arch = "powerpc64",
  target_arch = "sparc",
  target_arch = "sparc64",
)))]
mod arch {
  pub const IOC_SIZEBITS: u32 = 14;
  pub const IOC_NONE: u32 = 0;
  pub const IOC_READ: u32 = 2;
  pub const IOC_WRITE: u32 = 1;
}

pub use arch::{IOC_NONE, IOC_READ, IOC_WRITE};
use arch::IOC_SIZEBITS;

const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;

/// `_IOC(dir, type, nr, size)`
pub const fn ioc(dir: u32, ty: u8, nr: u8, size: usize) -> Request {
  assert!(size < (1usize << IOC_SIZEBITS), "ioctl argument too large");
  let v = (dir << IOC_DIRSHIFT)
    | ((ty as u32) << IOC_TYPESHIFT)
    | ((nr as u32) << IOC_NRSHIFT)
    | ((size as u32) << IOC_SIZESHIFT);
  v as Request
}

/// `_IO(type, nr)`
pub const fn io(ty: u8, nr: u8) -> Request {
  ioc(IOC_NONE, ty, nr, 0)
}

/// `_IOR(type, nr, T)`
pub const fn ior<T>(ty: u8, nr: u8) -> Request {
  ioc(IOC_READ, ty, nr, size_of::<T>())
}

/// `_IOW(type, nr, T)`
pub const fn iow<T>(ty: u8, nr: u8) -> Request {
  ioc(IOC_WRITE, ty, nr, size_of::<T>())
}

/// `_IOWR(type, nr, T)`
pub const fn iowr<T>(ty: u8, nr: u8) -> Request {
  ioc(IOC_READ | IOC_WRITE, ty, nr, size_of::<T>())
}
