/// Set ff device properties
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FfProperty {
  Gain,
  Autocenter,
}

impl FfProperty {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      FfProperty::Gain => 0x60,
      FfProperty::Autocenter => 0x61,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      FfProperty::Gain => "FF_GAIN",
      FfProperty::Autocenter => "FF_AUTOCENTER",
    }
  }
}
