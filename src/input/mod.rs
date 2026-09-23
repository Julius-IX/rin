//! (Mostly) Literal translation of [input.h](https://github.com/torvalds/linux/blob/master/include/uapi/linux/input.h).
//!
//! The `#define`d families of constants (`BUS_*`, `MT_TOOL_*`, `FF_*`, ...) are
//! modelled as enums with `code()`/`name()`, like in
//! [`input_event_codes`](crate::input_event_codes). Standalone values are plain
//! `const`s, and the `EVIOC*` ioctls live in [`ioctls`].

mod absinfo;
mod bus;
mod ff_effect_type;
mod ff_property;
mod ff_status;
mod ff_waveform;
mod force_feedback;
mod id_field;
mod input_event;
mod input_id;
mod input_mask;
mod keymap_entry;
mod mt_tool;

pub mod ioctls;

pub use absinfo::InputAbsinfo;
pub use bus::Bus;
pub use ff_effect_type::FfEffectType;
pub use ff_property::FfProperty;
pub use ff_status::FfStatus;
pub use ff_waveform::FfWaveform;
pub use force_feedback::{
  FfConditionEffect, FfConstantEffect, FfEffect, FfEffectData, FfEnvelope, FfHapticEffect,
  FfPeriodicEffect, FfRampEffect, FfReplay, FfRumbleEffect, FfTrigger,
};
pub use id_field::IdField;
pub use input_event::{InputEvent, KernelUlong, EV_VERSION};
pub use input_id::InputId;
pub use input_mask::InputMask;
pub use keymap_entry::{InputKeymapEntry, INPUT_KEYMAP_BY_INDEX};
pub use mt_tool::MtTool;

/// `FF_GAIN` is the first effect id that collides with another ff method
/// (`ff->set_gain()`), so the greatest safe effect id is `FF_GAIN - 1` and the
/// total number of effects should never exceed `FF_GAIN`.
pub const FF_MAX_EFFECTS: u32 = 0x60;

/// Highest force feedback code (`EV_FF` event codes are `0..=FF_MAX`).
pub const FF_MAX: u32 = 0x7f;
pub const FF_CNT: u32 = FF_MAX + 1;
