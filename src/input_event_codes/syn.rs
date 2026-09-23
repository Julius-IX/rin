/// Synchronization events.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Syn {
  Report,
  Config,
  MtReport,
  Dropped,
  Max,
  Cnt,
}

impl Syn {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      Syn::Report => 0,
      Syn::Config => 1,
      Syn::MtReport => 2,
      Syn::Dropped => 3,
      Syn::Max => 0xf,
      Syn::Cnt => Syn::Max.code() + 1,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      Syn::Report => "SYN_REPORT",
      Syn::Config => "SYN_CONFIG",
      Syn::MtReport => "SYN_MT_REPORT",
      Syn::Dropped => "SYN_DROPPED",
      Syn::Max => "SYN_MAX",
      Syn::Cnt => "SYN_CNT",
    }
  }
}
