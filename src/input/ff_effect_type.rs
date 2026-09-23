/// Force feedback effect types
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FfEffectType {
  Haptic,
  Rumble,
  Periodic,
  Constant,
  Spring,
  Friction,
  Damper,
  Inertia,
  Ramp,

  EffectMin,
  EffectMax,
}

impl FfEffectType {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      FfEffectType::Haptic => 0x4f,
      FfEffectType::Rumble => 0x50,
      FfEffectType::Periodic => 0x51,
      FfEffectType::Constant => 0x52,
      FfEffectType::Spring => 0x53,
      FfEffectType::Friction => 0x54,
      FfEffectType::Damper => 0x55,
      FfEffectType::Inertia => 0x56,
      FfEffectType::Ramp => 0x57,
      FfEffectType::EffectMin => FfEffectType::Haptic.code(),
      FfEffectType::EffectMax => FfEffectType::Ramp.code(),
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      FfEffectType::Haptic => "FF_HAPTIC",
      FfEffectType::Rumble => "FF_RUMBLE",
      FfEffectType::Periodic => "FF_PERIODIC",
      FfEffectType::Constant => "FF_CONSTANT",
      FfEffectType::Spring => "FF_SPRING",
      FfEffectType::Friction => "FF_FRICTION",
      FfEffectType::Damper => "FF_DAMPER",
      FfEffectType::Inertia => "FF_INERTIA",
      FfEffectType::Ramp => "FF_RAMP",
      FfEffectType::EffectMin => "FF_EFFECT_MIN",
      FfEffectType::EffectMax => "FF_EFFECT_MAX",
    }
  }
}
