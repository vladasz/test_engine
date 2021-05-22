#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate tools;
#[macro_use]
extern crate guard;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate mashup;

mod gm;
mod image;
mod te;
mod ui;
#[macro_use]
mod gl_wrapper;

use crate::gl_wrapper::GLDrawer;
use crate::gm::Size;
use crate::te::Screen;

const KOK_SOS: i32 = 10;
const KOK_SOSAT: i32 = 20;

fn main() {
    GLDrawer::<Screen>::with_size(Size {
        width: 1000.0,
        height: 600.0,
    })
    .start_main_loop();
}
