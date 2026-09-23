/// `struct input_absinfo` - used by `EVIOCGABS`/`EVIOCSABS` ioctls
///
/// Note that input core does not clamp reported values to the
/// `[minimum, maximum]` limits, such task is left to userspace.
///
/// The default resolution for main axes (`ABS_X`, `ABS_Y`, `ABS_Z`,
/// `ABS_MT_POSITION_X`, `ABS_MT_POSITION_Y`) is reported in units
/// per millimeter (units/mm), resolution for rotational axes
/// (`ABS_RX`, `ABS_RY`, `ABS_RZ`) is reported in units per radian.
/// The resolution for the size axes (`ABS_MT_TOUCH_MAJOR`,
/// `ABS_MT_TOUCH_MINOR`, `ABS_MT_WIDTH_MAJOR`, `ABS_MT_WIDTH_MINOR`)
/// is reported in units per millimeter (units/mm).
/// When `INPUT_PROP_ACCELEROMETER` is set the resolution changes.
/// The main axes (`ABS_X`, `ABS_Y`, `ABS_Z`) are then reported in
/// units per g (units/g) and in units per degree per second
/// (units/deg/s) for rotational axes (`ABS_RX`, `ABS_RY`, `ABS_RZ`).
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct InputAbsinfo {
  /// latest reported value for the axis.
  pub value: i32,
  /// specifies minimum value for the axis.
  pub minimum: i32,
  /// specifies maximum value for the axis.
  pub maximum: i32,
  /// specifies fuzz value that is used to filter noise from the event stream.
  pub fuzz: i32,
  /// values that are within this value will be discarded by joydev interface
  /// and reported as 0 instead.
  pub flat: i32,
  /// specifies resolution for the values reported for the axis.
  pub resolution: i32,
}
