/// Event types
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EventTypes {
  Syn,
  Key,
  Rel,
  Abs,
  Msc,
  Sw,
  Led,
  Snd,
  Rep,
  Ff,
  Pwd,
  FfStatus,
  Max,
  Cnt,
}

impl EventTypes {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      EventTypes::Syn => 0x00,
      EventTypes::Key => 0x01,
      EventTypes::Rel => 0x02,
      EventTypes::Abs => 0x03,
      EventTypes::Msc => 0x04,
      EventTypes::Sw => 0x05,
      EventTypes::Led => 0x11,
      EventTypes::Snd => 0x12,
      EventTypes::Rep => 0x14,
      EventTypes::Ff => 0x15,
      EventTypes::Pwd => 0x16,
      EventTypes::FfStatus => 0x17,
      EventTypes::Max => 0x1f,
      EventTypes::Cnt => EventTypes::Max.code() + 1,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      EventTypes::Syn => "EV_SYN",
      EventTypes::Key => "EV_KEY",
      EventTypes::Rel => "EV_REL",
      EventTypes::Abs => "EV_ABS",
      EventTypes::Msc => "EV_MSC",
      EventTypes::Sw => "EV_SW",
      EventTypes::Led => "EV_LED",
      EventTypes::Snd => "EV_SND",
      EventTypes::Rep => "EV_REP",
      EventTypes::Ff => "EV_FF",
      EventTypes::Pwd => "EV_PWR",
      EventTypes::FfStatus => "EV_FF_STATUS",
      EventTypes::Max => "EV_MAX",
      EventTypes::Cnt => "EV_CNT",
    }
  }
}
