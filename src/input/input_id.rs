/// `struct input_id`
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct InputId {
  /// One of [`Bus`](super::Bus)'s codes.
  pub bustype: u16,
  pub vendor: u16,
  pub product: u16,
  pub version: u16,
}
