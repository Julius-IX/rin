/// MT_TOOL types
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MtTool {
  Finger,
  Pen,
  Palm,
  Dial,
  Max,
}

impl MtTool {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      MtTool::Finger => 0x00,
      MtTool::Pen => 0x01,
      MtTool::Palm => 0x02,
      MtTool::Dial => 0x0a,
      MtTool::Max => 0x0f,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      MtTool::Finger => "MT_TOOL_FINGER",
      MtTool::Pen => "MT_TOOL_PEN",
      MtTool::Palm => "MT_TOOL_PALM",
      MtTool::Dial => "MT_TOOL_DIAL",
      MtTool::Max => "MT_TOOL_MAX",
    }
  }
}
