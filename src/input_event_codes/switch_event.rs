/// Switch events
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SwitchEvent {
  /// set = lid shut
  Lid,
  /// set = tablet mode
  TableMode,
  /// set = inserted
  HeadphoneInsert,
  /// rfkill master switch, type "any" set = radio enabled
  RfkillAll,
  /// deprecated
  Radio,
  /// set = inserted
  MicrophoneInsert,
  /// set = plugged into dock
  Dock,
  /// set = inserted
  LineoutInsert,
  /// set = mechanical switch set
  JackPhysicalInsert,
  /// set = inserted
  VideooutInsert,
  /// set = lens covered
  CameraLensCover,
  /// set = keypad slide out
  KeypadSlide,
  /// set = front proximity sensor active
  FrontProximity,
  /// set = rotate locked/disabled
  RotateLock,
  /// set = inserted
  LineinInsert,
  /// set = device disabled
  MuteDevice,
  /// set = pen inserted
  PenInserted,
  /// set = cover closed
  MachineCover,
  /// set = USB audio device connected
  UsbInsert,
  Max,
  Cnt,
}

impl SwitchEvent {
  /// The raw numeric event code, as defined by the Linux kernel.
  pub fn code(&self) -> u32 {
    match self {
      SwitchEvent::Lid => 0x00,
      SwitchEvent::TableMode => 0x01,
      SwitchEvent::HeadphoneInsert => 0x02,
      SwitchEvent::RfkillAll => 0x03,
      SwitchEvent::Radio => SwitchEvent::RfkillAll.code(),
      SwitchEvent::MicrophoneInsert => 0x04,
      SwitchEvent::Dock => 0x05,
      SwitchEvent::LineoutInsert => 0x06,
      SwitchEvent::JackPhysicalInsert => 0x07,
      SwitchEvent::VideooutInsert => 0x08,
      SwitchEvent::CameraLensCover => 0x09,
      SwitchEvent::KeypadSlide => 0x0a,
      SwitchEvent::FrontProximity => 0x0b,
      SwitchEvent::RotateLock => 0x0c,
      SwitchEvent::LineinInsert => 0x0d,
      SwitchEvent::MuteDevice => 0x0e,
      SwitchEvent::PenInserted => 0x0f,
      SwitchEvent::MachineCover => 0x10,
      SwitchEvent::UsbInsert => 0x11,
      SwitchEvent::Max => 0x11,
      SwitchEvent::Cnt => SwitchEvent::Max.code() + 1,
    }
  }

  /// The literal name of the constant as it appears in the Linux kernel header.
  pub fn name(&self) -> &'static str {
    match self {
      SwitchEvent::Lid => "SW_LID",
      SwitchEvent::TableMode => "SW_TABLET_MODE",
      SwitchEvent::HeadphoneInsert => "SW_HEADPHONE_INSERT",
      SwitchEvent::RfkillAll => "SW_RFKILL_ALL",
      SwitchEvent::Radio => "SW_RADIO",
      SwitchEvent::MicrophoneInsert => "SW_MICROPHONE_INSERT",
      SwitchEvent::Dock => "SW_DOCK",
      SwitchEvent::LineoutInsert => "SW_LINEOUT_INSERT",
      SwitchEvent::JackPhysicalInsert => "SW_JACK_PHYSICAL_INSERT",
      SwitchEvent::VideooutInsert => "SW_VIDEOOUT_INSERT",
      SwitchEvent::CameraLensCover => "SW_CAMERA_LENS_COVER",
      SwitchEvent::KeypadSlide => "SW_KEYPAD_SLIDE",
      SwitchEvent::FrontProximity => "SW_FRONT_PROXIMITY",
      SwitchEvent::RotateLock => "SW_ROTATE_LOCK",
      SwitchEvent::LineinInsert => "SW_LINEIN_INSERT",
      SwitchEvent::MuteDevice => "SW_MUTE_DEVICE",
      SwitchEvent::PenInserted => "SW_PEN_INSERTED",
      SwitchEvent::MachineCover => "SW_MACHINE_COVER",
      SwitchEvent::UsbInsert => "SW_USB_INSERT",
      SwitchEvent::Max => "SW_MAX",
      SwitchEvent::Cnt => "SW_CNT",
    }
  }
}
