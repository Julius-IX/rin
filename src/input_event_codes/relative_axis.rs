/// Relative axes
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RelativeAxis {
  X,
  Y,
  Z,
  Rx,
  Ry,
  Rz,
  Hwheel,
  Dial,
  Wheel,
  Misc,
  // 0x0a is reserved and should not be used in input drivers.
  // It was used by HID as REL_MISC+1 and userspace needs to detect if
  // the next REL_* event is correct or is just REL_MISC + n.
  // We define here REL_RESERVED so userspace can rely on it and detect
  // the situation described above.
  Reserved,
  WheelHiRes,
  HwheelHiRes,
  Max,
  Cnt,
}

impl RelativeAxis {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      RelativeAxis::X => 0x00,
      RelativeAxis::Y => 0x01,
      RelativeAxis::Z => 0x02,
      RelativeAxis::Rx => 0x03,
      RelativeAxis::Ry => 0x04,
      RelativeAxis::Rz => 0x05,
      RelativeAxis::Hwheel => 0x06,
      RelativeAxis::Dial => 0x07,
      RelativeAxis::Wheel => 0x08,
      RelativeAxis::Misc => 0x09,
      RelativeAxis::Reserved => 0x0a,
      RelativeAxis::WheelHiRes => 0x0b,
      RelativeAxis::HwheelHiRes => 0x0c,
      RelativeAxis::Max => 0x0f,
      RelativeAxis::Cnt => RelativeAxis::Max.code() + 1,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      RelativeAxis::X => "REL_X",
      RelativeAxis::Y => "REL_Y",
      RelativeAxis::Z => "REL_Z",
      RelativeAxis::Rx => "REL_RX",
      RelativeAxis::Ry => "REL_RY",
      RelativeAxis::Rz => "REL_RZ",
      RelativeAxis::Hwheel => "REL_HWHEEL",
      RelativeAxis::Dial => "REL_DIAL",
      RelativeAxis::Wheel => "REL_WHEEL",
      RelativeAxis::Misc => "REL_MISC",
      RelativeAxis::Reserved => "REL_RESERVED",
      RelativeAxis::WheelHiRes => "REL_WHEEL_HI_RES",
      RelativeAxis::HwheelHiRes => "REL_HWHEEL_HI_RES",
      RelativeAxis::Max => "REL_MAX",
      RelativeAxis::Cnt => "REL_CNT",
    }
  }
}
