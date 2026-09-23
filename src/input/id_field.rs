/// Indices into the [`InputId`](super::InputId) fields (`ID_*`).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IdField {
  Bus,
  Vendor,
  Product,
  Version,
}

impl IdField {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      IdField::Bus => 0,
      IdField::Vendor => 1,
      IdField::Product => 2,
      IdField::Version => 3,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      IdField::Bus => "ID_BUS",
      IdField::Vendor => "ID_VENDOR",
      IdField::Product => "ID_PRODUCT",
      IdField::Version => "ID_VERSION",
    }
  }
}
