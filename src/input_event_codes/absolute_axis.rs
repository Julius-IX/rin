/// Absolute axes
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AbsoluteAxis {
  X,
  Y,
  Z,
  Rx,
  Ry,
  Rz,
  Throttle,
  Rudder,
  Wheel,
  Gas,
  Brake,
  Hat0x,
  Hat0y,
  Hat1x,
  Hat1y,
  Hat2x,
  Hat2y,
  Hat3x,
  Hat3y,
  Pressure,
  Distance,
  TiltX,
  TiltY,
  ToolWidth,

  Volume,
  Profile,
  SndProfile,

  Misc,

  // 0x2e is reserved and should not be used in input drivers.
  // It was used by HID as ABS_MISC+6 and userspace needs to detect if
  // the next ABS_* event is correct or is just ABS_MISC + n.
  // We define here ABS_RESERVED so userspace can rely on it and detect
  // the situation described above.
  Reserved,

  /// MT slot being modified
  MtSlot,
  /// Major axis of touching ellipse
  MtTouchMajor,
  /// Minor axis (omit if circular)
  MtTouchMinor,
  /// Major axis of approaching ellipse
  MtWidthMajor,
  /// Minor axis (omit if circular)
  MtWidthMinor,
  /// Ellipse orientation
  MtOrientation,
  /// Center X touch position
  MtPositionX,
  /// Center Y touch position
  MtPositionY,
  /// Type of touching device
  MtToolType,
  /// Group a set of packets as a blob
  MtBlobId,
  /// Unique ID of initiated contact
  MtTrackingId,
  /// Pressure on contact area
  MtPressure,
  /// Contact hover distance
  MtDistance,
  /// Center X tool position
  MtToolX,
  /// Center Y tool position
  MtToolY,

  Max,
  Cnt,
}

impl AbsoluteAxis {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      AbsoluteAxis::X => 0x00,
      AbsoluteAxis::Y => 0x01,
      AbsoluteAxis::Z => 0x02,
      AbsoluteAxis::Rx => 0x03,
      AbsoluteAxis::Ry => 0x04,
      AbsoluteAxis::Rz => 0x05,
      AbsoluteAxis::Throttle => 0x06,
      AbsoluteAxis::Rudder => 0x07,
      AbsoluteAxis::Wheel => 0x08,
      AbsoluteAxis::Gas => 0x09,
      AbsoluteAxis::Brake => 0x0a,
      AbsoluteAxis::Hat0x => 0x10,
      AbsoluteAxis::Hat0y => 0x11,
      AbsoluteAxis::Hat1x => 0x12,
      AbsoluteAxis::Hat1y => 0x13,
      AbsoluteAxis::Hat2x => 0x14,
      AbsoluteAxis::Hat2y => 0x15,
      AbsoluteAxis::Hat3x => 0x16,
      AbsoluteAxis::Hat3y => 0x17,
      AbsoluteAxis::Pressure => 0x18,
      AbsoluteAxis::Distance => 0x19,
      AbsoluteAxis::TiltX => 0x1a,
      AbsoluteAxis::TiltY => 0x1b,
      AbsoluteAxis::ToolWidth => 0x1c,
      AbsoluteAxis::Volume => 0x20,
      AbsoluteAxis::Profile => 0x21,
      AbsoluteAxis::SndProfile => 0x22,
      AbsoluteAxis::Misc => 0x28,
      AbsoluteAxis::Reserved => 0x2e,
      AbsoluteAxis::MtSlot => 0x2f,
      AbsoluteAxis::MtTouchMajor => 0x30,
      AbsoluteAxis::MtTouchMinor => 0x31,
      AbsoluteAxis::MtWidthMajor => 0x32,
      AbsoluteAxis::MtWidthMinor => 0x33,
      AbsoluteAxis::MtOrientation => 0x34,
      AbsoluteAxis::MtPositionX => 0x35,
      AbsoluteAxis::MtPositionY => 0x36,
      AbsoluteAxis::MtToolType => 0x37,
      AbsoluteAxis::MtBlobId => 0x38,
      AbsoluteAxis::MtTrackingId => 0x39,
      AbsoluteAxis::MtPressure => 0x3a,
      AbsoluteAxis::MtDistance => 0x3b,
      AbsoluteAxis::MtToolX => 0x3c,
      AbsoluteAxis::MtToolY => 0x3d,
      AbsoluteAxis::Max => 0x3f,
      AbsoluteAxis::Cnt => AbsoluteAxis::Max.code() + 1,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      AbsoluteAxis::X => "ABS_X",
      AbsoluteAxis::Y => "ABS_Y",
      AbsoluteAxis::Z => "ABS_Z",
      AbsoluteAxis::Rx => "ABS_RX",
      AbsoluteAxis::Ry => "ABS_RY",
      AbsoluteAxis::Rz => "ABS_RZ",
      AbsoluteAxis::Throttle => "ABS_THROTTLE",
      AbsoluteAxis::Rudder => "ABS_RUDDER",
      AbsoluteAxis::Wheel => "ABS_WHEEL",
      AbsoluteAxis::Gas => "ABS_GAS",
      AbsoluteAxis::Brake => "ABS_BRAKE",
      AbsoluteAxis::Hat0x => "ABS_HAT0X",
      AbsoluteAxis::Hat0y => "ABS_HAT0Y",
      AbsoluteAxis::Hat1x => "ABS_HAT1X",
      AbsoluteAxis::Hat1y => "ABS_HAT1Y",
      AbsoluteAxis::Hat2x => "ABS_HAT2X",
      AbsoluteAxis::Hat2y => "ABS_HAT2Y",
      AbsoluteAxis::Hat3x => "ABS_HAT3X",
      AbsoluteAxis::Hat3y => "ABS_HAT3Y",
      AbsoluteAxis::Pressure => "ABS_PRESSURE",
      AbsoluteAxis::Distance => "ABS_DISTANCE",
      AbsoluteAxis::TiltX => "ABS_TILT_X",
      AbsoluteAxis::TiltY => "ABS_TILT_Y",
      AbsoluteAxis::ToolWidth => "ABS_TOOL_WIDTH",
      AbsoluteAxis::Volume => "ABS_VOLUME",
      AbsoluteAxis::Profile => "ABS_PROFILE",
      AbsoluteAxis::SndProfile => "ABS_SND_PROFILE",
      AbsoluteAxis::Misc => "ABS_MISC",
      AbsoluteAxis::Reserved => "ABS_RESERVED",
      AbsoluteAxis::MtSlot => "ABS_MT_SLOT",
      AbsoluteAxis::MtTouchMajor => "ABS_MT_TOUCH_MAJOR",
      AbsoluteAxis::MtTouchMinor => "ABS_MT_TOUCH_MINOR",
      AbsoluteAxis::MtWidthMajor => "ABS_MT_WIDTH_MAJOR",
      AbsoluteAxis::MtWidthMinor => "ABS_MT_WIDTH_MINOR",
      AbsoluteAxis::MtOrientation => "ABS_MT_ORIENTATION",
      AbsoluteAxis::MtPositionX => "ABS_MT_POSITION_X",
      AbsoluteAxis::MtPositionY => "ABS_MT_POSITION_Y",
      AbsoluteAxis::MtToolType => "ABS_MT_TOOL_TYPE",
      AbsoluteAxis::MtBlobId => "ABS_MT_BLOB_ID",
      AbsoluteAxis::MtTrackingId => "ABS_MT_TRACKING_ID",
      AbsoluteAxis::MtPressure => "ABS_MT_PRESSURE",
      AbsoluteAxis::MtDistance => "ABS_MT_DISTANCE",
      AbsoluteAxis::MtToolX => "ABS_MT_TOOL_X",
      AbsoluteAxis::MtToolY => "ABS_MT_TOOL_Y",
      AbsoluteAxis::Max => "ABS_MAX",
      AbsoluteAxis::Cnt => "ABS_CNT",
    }
  }
}
