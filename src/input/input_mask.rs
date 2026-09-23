/// `struct input_mask` - argument of `EVIOCGMASK`/`EVIOCSMASK`.
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct InputMask {
  /// The event type to query/set.
  pub r#type: u32,
  /// Size of the buffer `codes_ptr` points to, in bytes.
  pub codes_size: u32,
  /// Userspace address of the mask buffer (always 64 bits wide, even on
  /// 32-bit targets).
  pub codes_ptr: u64,
}
