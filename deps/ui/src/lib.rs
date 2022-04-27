#![allow(clippy::mismatched_target_os)]
#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(specialization)]

use std::{path::PathBuf, sync::Mutex};

pub use basic::{ImageView, Label};
pub use complex::DPadView;
pub use input::Touch;
use lazy_static::lazy_static;
pub use text::{Font, Glyph};

pub mod basic;
pub mod complex;
pub mod input;
pub mod placer;
pub mod test;
pub mod text;
mod ui_drawer;
mod view;

pub use ui_drawer::UIDrawer;
pub use view::*;

#[macro_use]
extern crate log;
extern crate core;

lazy_static! {
    pub(crate) static ref DEFAULT_FONT_PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
    pub(crate) static ref DEFAULT_FONT: Mutex<Font> =
        Mutex::new(Font::new(&DEFAULT_FONT_PATH.lock().unwrap(), 48).unwrap());
}

pub fn set_default_font_path(font: PathBuf) {
    let mut font_path = DEFAULT_FONT_PATH.lock().unwrap();
    *font_path = font;
}
