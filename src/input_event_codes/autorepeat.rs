/// Autorepeat values
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Autorepeat {
  Delay,
  Period,
  Max,
  Cnt,
}

impl Autorepeat {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      Autorepeat::Delay => 0x00,
      Autorepeat::Period => 0x01,
      Autorepeat::Max => 0x01,
      Autorepeat::Cnt => Autorepeat::Max.code() + 1,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      Autorepeat::Delay => "REP_DELAY",
      Autorepeat::Period => "REP_PERIOD",
      Autorepeat::Max => "REP_MAX",
      Autorepeat::Cnt => "REP_CNT",
    }
  }
}
