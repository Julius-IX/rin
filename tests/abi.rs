//! Sanity checks against values produced by the C headers on x86_64 Linux.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use rin::input::ioctls::*;
use rin::input::*;
use rin::input_event_codes::{AbsoluteAxis, EventTypes};
use rin::uinput::*;
use std::mem::{align_of, size_of};

#[test]
fn struct_sizes() {
  assert_eq!(size_of::<InputEvent>(), 24);
  assert_eq!(size_of::<InputId>(), 8);
  assert_eq!(size_of::<InputAbsinfo>(), 24);
  assert_eq!(size_of::<InputKeymapEntry>(), 40);
  assert_eq!(size_of::<InputMask>(), 16);
  assert_eq!(size_of::<FfEffect>(), 48);
  assert_eq!(align_of::<FfEffect>(), 8);
  assert_eq!(size_of::<FfHapticEffect>(), 12);
  assert_eq!(size_of::<UinputSetup>(), 92);
  assert_eq!(size_of::<UinputAbsSetup>(), 28);
  assert_eq!(size_of::<UinputFfUpload>(), 104);
  assert_eq!(size_of::<UinputFfErase>(), 12);
  assert_eq!(size_of::<UinputUserDev>(), 80 + 8 + 4 + 4 * 4 * 64);
}

#[test]
fn field_offsets() {
  let e = InputEvent::default();
  let base = &e as *const _ as usize;
  assert_eq!(&e.r#type as *const _ as usize - base, 16);
  assert_eq!(&e.code as *const _ as usize - base, 18);
  assert_eq!(&e.value as *const _ as usize - base, 20);
  let a = UinputAbsSetup::default();
  let base = &a as *const _ as usize;
  assert_eq!(&a.absinfo as *const _ as usize - base, 4);
}

#[test]
fn ioctl_numbers() {
  assert_eq!(EVIOCGVERSION as u32, 0x80044501);
  assert_eq!(EVIOCGID as u32, 0x80084502);
  assert_eq!(EVIOCGKEYCODE_V2 as u32, 0x80284504);
  assert_eq!(eviocgname(256) as u32, 0x81004506);
  assert_eq!(eviocgbit(EventTypes::Key.code(), 96) as u32, 0x80604521);
  assert_eq!(eviocgabs(AbsoluteAxis::X.code()) as u32, 0x80184540);
  assert_eq!(eviocsabs(AbsoluteAxis::X.code()) as u32, 0x401845c0);
  assert_eq!(EVIOCSFF as u32, 0x40304580);
  assert_eq!(EVIOCGRAB as u32, 0x40044590);
  assert_eq!(EVIOCGMASK as u32, 0x80104592);
  assert_eq!(EVIOCSCLOCKID as u32, 0x400445a0);

  assert_eq!(UI_DEV_CREATE as u32, 0x5501);
  assert_eq!(UI_DEV_DESTROY as u32, 0x5502);
  assert_eq!(UI_DEV_SETUP as u32, 0x405c5503);
  assert_eq!(UI_ABS_SETUP as u32, 0x401c5504);
  assert_eq!(UI_SET_EVBIT as u32, 0x40045564);
  assert_eq!(UI_SET_KEYBIT as u32, 0x40045565);
  assert_eq!(UI_SET_PHYS as u32, 0x4008556c);
  assert_eq!(UI_BEGIN_FF_UPLOAD as u32, 0xc06855c8);
  assert_eq!(ui_get_sysname(64) as u32, 0x8040552c);
  assert_eq!(UI_GET_VERSION as u32, 0x8004552d);
}

#[test]
fn consts_agree_with_codes() {
  assert_eq!(ABS_CNT as u32, AbsoluteAxis::Cnt.code());
  assert_eq!(FF_MAX_EFFECTS, FfProperty::Gain.code());
  assert_eq!(FfEffectType::EffectMax.code(), 0x57);
}

#[test]
fn name_helper() {
  let mut s = UinputSetup::default();
  s.set_name("virtual kbd");
  assert_eq!(s.name[0] as u8, b'v');
  assert_eq!(s.name[11], 0);
  s.set_name(&"x".repeat(200));
  assert_eq!(s.name[79], 0);
  assert_eq!(s.name[78] as u8, b'x');
}
