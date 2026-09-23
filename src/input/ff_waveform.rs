/// Force feedback periodic effect types
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FfWaveform {
  Square,
  Triangle,
  Sine,
  SawUp,
  SawDown,
  Custom,

  WaveformMin,
  WaveformMax,
}

impl FfWaveform {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      FfWaveform::Square => 0x58,
      FfWaveform::Triangle => 0x59,
      FfWaveform::Sine => 0x5a,
      FfWaveform::SawUp => 0x5b,
      FfWaveform::SawDown => 0x5c,
      FfWaveform::Custom => 0x5d,
      FfWaveform::WaveformMin => FfWaveform::Square.code(),
      FfWaveform::WaveformMax => FfWaveform::Custom.code(),
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      FfWaveform::Square => "FF_SQUARE",
      FfWaveform::Triangle => "FF_TRIANGLE",
      FfWaveform::Sine => "FF_SINE",
      FfWaveform::SawUp => "FF_SAW_UP",
      FfWaveform::SawDown => "FF_SAW_DOWN",
      FfWaveform::Custom => "FF_CUSTOM",
      FfWaveform::WaveformMin => "FF_WAVEFORM_MIN",
      FfWaveform::WaveformMax => "FF_WAVEFORM_MAX",
    }
  }
}
