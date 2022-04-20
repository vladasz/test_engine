#![allow(incomplete_features)]
#![allow(clippy::mismatched_target_os)]
#![feature(trait_upcasting)]
#![feature(stmt_expr_attributes)]
#![feature(core_ffi_c)]
#![feature(const_trait_impl)]

pub use audio;
pub use gl_image;
pub use gl_wrapper;
pub use gm;
pub use maze;
pub use rtools;
pub use sprites;
pub use ui;

pub use crate::screen::Screen;

pub mod app;
pub mod assets;
pub mod debug_view;
pub mod game_view;
pub mod paths;
pub mod screen;
pub mod sprite_view;
mod sprites_drawer;
mod ui_drawer;
pub mod ui_layer;

pub use gl_image::Image;
pub use sprites::{Level, LevelBase, Sprite};

#[cfg(mobile)]
#[macro_use]
extern crate mashup;

#[macro_use]
extern crate log;
