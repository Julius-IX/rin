//! (Mostly) Literal translation of [input-event-codes.h](https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h).

mod absolute_axis;
mod autorepeat;
mod event_types;
mod input_prop;
mod key;
mod led;
mod misc;
mod relative_axis;
mod sound;
mod sound_profile;
mod switch_event;
mod syn;

pub use absolute_axis::AbsoluteAxis;
pub use autorepeat::Autorepeat;
pub use event_types::EventTypes;
pub use input_prop::InputProp;
pub use key::Key;
pub use led::Led;
pub use misc::Misc;
pub use relative_axis::RelativeAxis;
pub use sound::Sound;
pub use sound_profile::SoundProfile;
pub use switch_event::SwitchEvent;
pub use syn::Syn;
