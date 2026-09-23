/// Device properties and quirks
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InputProp {
  /// needs a pointer
  Pointer,
  /// direct input devices
  Direct,
  /// has button(s) under pad
  ButtonPad,
  /// touch rectangle only
  SemiMt,
  /// softbuttons at top of pad
  TopButtonpad,
  /// is a pointing stick
  PointingStick,
  /// has accelerometer
  Accelerometer,
  /// pressure triggers clicks
  Pressurepad,

  Max,
  Cnt,
}

impl InputProp {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      InputProp::Pointer => 0x00,
      InputProp::Direct => 0x01,
      InputProp::ButtonPad => 0x02,
      InputProp::SemiMt => 0x03,
      InputProp::TopButtonpad => 0x04,
      InputProp::PointingStick => 0x05,
      InputProp::Accelerometer => 0x06,
      InputProp::Pressurepad => 0x07,
      InputProp::Max => 0x1f,
      InputProp::Cnt => InputProp::Max.code() + 1,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      InputProp::Pointer => "INPUT_PROP_POINTER",
      InputProp::Direct => "INPUT_PROP_DIRECT",
      InputProp::ButtonPad => "INPUT_PROP_BUTTONPAD",
      InputProp::SemiMt => "INPUT_PROP_SEMI_MT",
      InputProp::TopButtonpad => "INPUT_PROP_TOPBUTTONPAD",
      InputProp::PointingStick => "INPUT_PROP_POINTING_STICK",
      InputProp::Accelerometer => "INPUT_PROP_ACCELEROMETER",
      InputProp::Pressurepad => "INPUT_PROP_PRESSUREPAD",
      InputProp::Max => "INPUT_PROP_MAX",
      InputProp::Cnt => "INPUT_PROP_CNT",
    }
  }
}
