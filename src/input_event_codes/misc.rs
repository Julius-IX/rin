/// Misc events
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Misc {
  Serial,
  Pulseled,
  Gesture,
  Raw,
  Scan,
  Timestamp,
  Max,
  Cnt,
}

impl Misc {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      Misc::Serial => 0x00,
      Misc::Pulseled => 0x01,
      Misc::Gesture => 0x02,
      Misc::Raw => 0x03,
      Misc::Scan => 0x04,
      Misc::Timestamp => 0x05,
      Misc::Max => 0x07,
      Misc::Cnt => Misc::Max.code() + 1,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      Misc::Serial => "MSC_SERIAL",
      Misc::Pulseled => "MSC_PULSELED",
      Misc::Gesture => "MSC_GESTURE",
      Misc::Raw => "MSC_RAW",
      Misc::Scan => "MSC_SCAN",
      Misc::Timestamp => "MSC_TIMESTAMP",
      Misc::Max => "MSC_MAX",
      Misc::Cnt => "MSC_CNT",
    }
  }
}
