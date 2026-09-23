/// Values describing the status of a force-feedback effect
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FfStatus {
  Stopped,
  Playing,
  Max,
}

impl FfStatus {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      FfStatus::Stopped => 0x00,
      FfStatus::Playing => 0x01,
      FfStatus::Max => 0x01,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      FfStatus::Stopped => "FF_STATUS_STOPPED",
      FfStatus::Playing => "FF_STATUS_PLAYING",
      FfStatus::Max => "FF_STATUS_MAX",
    }
  }
}
