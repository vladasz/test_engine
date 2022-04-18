use gl_wrapper::Buffer;
use gm::{flat::PointsPath, Color};

#[derive(Debug)]
pub struct PathData {
    pub buffer:    Buffer,
    pub path:      PointsPath,
    pub color:     Color,
    pub draw_mode: DrawMode,
}

impl PathData {}

#[derive(Debug)]
pub enum DrawMode {
    Outline,
    Fill,
}

impl DrawMode {
    pub fn to_gl(&self) -> u32 {
        6 //GL_TRIANGLE_FAN
    }
}
