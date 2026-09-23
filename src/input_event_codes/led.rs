/// LEDs
///
/// Do not add any new LED definitions to the input subsystem. The existing
/// definitions are legacy and grandfathered for backwards compatibility with
/// userspace (via evdev). Any new LEDs should be implemented using the
/// LED subsystem (struct led_classdev). The input core provides a bridge to
/// the LED subsystem in drivers/input/input-leds.c.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Led {
  Numl,
  Capsl,
  Scrolll,
  Compose,
  Kana,
  Sleep,
  Suspend,
  Mute,
  Misc,
  Mail,
  Charging,
  Max,
  Cnt,
}

impl Led {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      Led::Numl => 0x00,
      Led::Capsl => 0x01,
      Led::Scrolll => 0x02,
      Led::Compose => 0x03,
      Led::Kana => 0x04,
      Led::Sleep => 0x05,
      Led::Suspend => 0x06,
      Led::Mute => 0x07,
      Led::Misc => 0x08,
      Led::Mail => 0x09,
      Led::Charging => 0x0a,
      Led::Max => 0x0f,
      Led::Cnt => Led::Max.code() + 1,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      Led::Numl => "LED_NUML",
      Led::Capsl => "LED_CAPSL",
      Led::Scrolll => "LED_SCROLLL",
      Led::Compose => "LED_COMPOSE",
      Led::Kana => "LED_KANA",
      Led::Sleep => "LED_SLEEP",
      Led::Suspend => "LED_SUSPEND",
      Led::Mute => "LED_MUTE",
      Led::Misc => "LED_MISC",
      Led::Mail => "LED_MAIL",
      Led::Charging => "LED_CHARGING",
      Led::Max => "LED_MAX",
      Led::Cnt => "LED_CNT",
    }
  }
}
