mod managed;

use cfg_if::cfg_if;

cfg_if! { if #[cfg(android)] {
    mod android_sound;
    use android_sound as sound;
} else {
    mod sound;
}}

pub use sound::Sound;
