/// ABS_SND_PROFILE values
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SoundProfile {
  ProfileSilent,
  ProfileVibrate,
  ProfileRing,
}

impl SoundProfile {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      SoundProfile::ProfileSilent => 0x00,
      SoundProfile::ProfileVibrate => 0x01,
      SoundProfile::ProfileRing => 0x02,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      SoundProfile::ProfileSilent => "SND_PROFILE_SILENT",
      SoundProfile::ProfileVibrate => "SND_PROFILE_VIBRATE",
      SoundProfile::ProfileRing => "SND_PROFILE_RING",
    }
  }
}
