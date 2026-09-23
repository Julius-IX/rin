/// Bus types (`BUS_*`), used in [`InputId::bustype`](super::InputId).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Bus {
  Pci,
  Isapnp,
  Usb,
  Hil,
  Bluetooth,
  Virtual,

  Isa,
  I8042,
  Xtkbd,
  Rs232,
  Gameport,
  Parport,
  Amiga,
  Adb,
  I2c,
  Host,
  Gsc,
  Atari,
  Spi,
  Rmi,
  Cec,
  IntelIshtp,
  AmdSfh,
  Sdw,
}

impl Bus {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      Bus::Pci => 0x01,
      Bus::Isapnp => 0x02,
      Bus::Usb => 0x03,
      Bus::Hil => 0x04,
      Bus::Bluetooth => 0x05,
      Bus::Virtual => 0x06,
      Bus::Isa => 0x10,
      Bus::I8042 => 0x11,
      Bus::Xtkbd => 0x12,
      Bus::Rs232 => 0x13,
      Bus::Gameport => 0x14,
      Bus::Parport => 0x15,
      Bus::Amiga => 0x16,
      Bus::Adb => 0x17,
      Bus::I2c => 0x18,
      Bus::Host => 0x19,
      Bus::Gsc => 0x1A,
      Bus::Atari => 0x1B,
      Bus::Spi => 0x1C,
      Bus::Rmi => 0x1D,
      Bus::Cec => 0x1E,
      Bus::IntelIshtp => 0x1F,
      Bus::AmdSfh => 0x20,
      Bus::Sdw => 0x21,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      Bus::Pci => "BUS_PCI",
      Bus::Isapnp => "BUS_ISAPNP",
      Bus::Usb => "BUS_USB",
      Bus::Hil => "BUS_HIL",
      Bus::Bluetooth => "BUS_BLUETOOTH",
      Bus::Virtual => "BUS_VIRTUAL",
      Bus::Isa => "BUS_ISA",
      Bus::I8042 => "BUS_I8042",
      Bus::Xtkbd => "BUS_XTKBD",
      Bus::Rs232 => "BUS_RS232",
      Bus::Gameport => "BUS_GAMEPORT",
      Bus::Parport => "BUS_PARPORT",
      Bus::Amiga => "BUS_AMIGA",
      Bus::Adb => "BUS_ADB",
      Bus::I2c => "BUS_I2C",
      Bus::Host => "BUS_HOST",
      Bus::Gsc => "BUS_GSC",
      Bus::Atari => "BUS_ATARI",
      Bus::Spi => "BUS_SPI",
      Bus::Rmi => "BUS_RMI",
      Bus::Cec => "BUS_CEC",
      Bus::IntelIshtp => "BUS_INTEL_ISHTP",
      Bus::AmdSfh => "BUS_AMD_SFH",
      Bus::Sdw => "BUS_SDW",
    }
  }
}
