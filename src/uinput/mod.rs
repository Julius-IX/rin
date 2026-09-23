//! (Mostly) Literal translation of [uinput.h](https://github.com/torvalds/linux/blob/master/include/uapi/linux/uinput.h).
//!
//! User level driver support for the input subsystem. Open `/dev/uinput`,
//! configure the device with the `UI_SET_*` / `UI_DEV_SETUP` / `UI_ABS_SETUP`
//! ioctls, then create it with [`UI_DEV_CREATE`] and write
//! [`InputEvent`](crate::input::InputEvent)s to the file descriptor.
//!
//! Heavily based on evdev.c by Vojtech Pavlik
//! Author of the C header: Aristeu Sergio Rozanski Filho <aris@cathedrallabs.org>
//!
//! Protocol revision history (from the C header):
//! - 0.5 08/13/2015: add `UI_DEV_SETUP`, `UI_ABS_SETUP`, `UI_GET_VERSION` ioctls
//! - 0.4 01/09/2014: add `UI_GET_SYSNAME` ioctl
//! - 0.3 24/05/2006: update ff support for the changes in kernel interface, add `UINPUT_VERSION`
//! - 0.2 16/10/2004: added force feedback support, added `UI_SET_PHYS`
//! - 0.1 20/06/2002: first public version

mod ioctls;
mod structs;

pub use ioctls::*;
pub use structs::*;

pub const UINPUT_VERSION: u32 = 5;
pub const UINPUT_MAX_NAME_SIZE: usize = 80;

/// `ABS_CNT` (`ABS_MAX + 1`), as a `const` so it can be used as an array length.
///
/// Equal to `AbsoluteAxis::Cnt.code()`, which is not usable in a const context.
pub const ABS_CNT: usize = 0x40;

/// This is the new event type, used only by uinput.
/// `code` is [`UI_FF_UPLOAD`] or [`UI_FF_ERASE`], and `value`
/// is the unique request ID. This number was picked
/// arbitrarily, above `EV_MAX` (since the input system
/// never sees it) but in the range of a 16-bit int.
pub const EV_UINPUT: u16 = 0x0101;
pub const UI_FF_UPLOAD: u16 = 1;
pub const UI_FF_ERASE: u16 = 2;
