/// Sounds
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Sound {
  Click,
  Bell,
  Tone,
  Max,
  Cnt,
}

impl Sound {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      Sound::Click => 0x00,
      Sound::Bell => 0x01,
      Sound::Tone => 0x02,
      Sound::Max => 0x07,
      Sound::Cnt => Sound::Max.code() + 1,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      Sound::Click => "SND_CLICK",
      Sound::Bell => "SND_BELL",
      Sound::Tone => "SND_TONE",
      Sound::Max => "SND_MAX",
      Sound::Cnt => "SND_CNT",
    }
  }
}
