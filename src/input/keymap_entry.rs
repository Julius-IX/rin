/// `INPUT_KEYMAP_BY_INDEX`: perform the lookup in the keymap by `index`
/// instead of `scancode`.
pub const INPUT_KEYMAP_BY_INDEX: u8 = 1 << 0;

/// `struct input_keymap_entry` - used by `EVIOCGKEYCODE`/`EVIOCSKEYCODE` ioctls
///
/// The structure is used to retrieve and modify keymap data. Users have
/// option of performing lookup either by `scancode` itself or by `index`
/// in keymap entry. `EVIOCGKEYCODE` will also return scancode or index
/// (depending on which element was used to perform lookup).
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct InputKeymapEntry {
  /// allows to specify how kernel should handle the request. For example,
  /// setting [`INPUT_KEYMAP_BY_INDEX`] indicates that kernel should perform
  /// lookup in keymap by `index` instead of `scancode`
  pub flags: u8,
  /// length of the scancode that resides in `scancode` buffer.
  pub len: u8,
  /// index in the keymap, may be used instead of scancode
  pub index: u16,
  /// key code assigned to this scancode
  pub keycode: u32,
  /// scancode represented in machine-endian form.
  pub scancode: [u8; 32],
}
