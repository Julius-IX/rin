use std::os::raw::c_ulong;

#[cfg(all(target_os = "linux", target_arch = "sparc64"))]
use std::os::raw::c_uint;

/// `__kernel_ulong_t`
pub type KernelUlong = c_ulong;

/// Protocol version.
pub const EV_VERSION: u32 = 0x010001;

/// The event structure itself.
///
/// This is the `__kernel_ulong_t` flavour of the C struct (the branch of
/// `input.h` used with `__KERNEL__` or 64-bit `time_t`). On 64-bit targets it
/// is identical to the `struct timeval` flavour; on 32-bit targets it matches
/// what the kernel actually reads and writes.
///
/// The C header exposes the timestamp as the `input_event_sec` /
/// `input_event_usec` macros; here they are the `sec` / `usec` fields.
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct InputEvent {
  /// `input_event_sec`
  pub sec: KernelUlong,
  /// `input_event_usec`
  #[cfg(all(target_os = "linux", target_arch = "sparc64"))]
  pub usec: c_uint,
  #[cfg(all(target_os = "linux", target_arch = "sparc64"))]
  pub __pad: c_uint,
  /// `input_event_usec`
  #[cfg(not(all(target_os = "linux", target_arch = "sparc64")))]
  pub usec: KernelUlong,
  pub r#type: u16,
  pub code: u16,
  pub value: i32,
}

impl InputEvent {
  /// Builds an event with a zeroed timestamp (the kernel fills it in for
  /// events written to a uinput device).
  pub fn new(event_type: crate::input_event_codes::EventTypes, code: u32, value: i32) -> Self {
    InputEvent {
      r#type: event_type.code() as u16,
      code: code as u16,
      value,
      ..Default::default()
    }
  }
}
