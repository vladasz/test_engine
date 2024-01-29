use anyhow::Result;
use gl_wrapper::path_data::{DrawMode, PathData};
use gm::{flat::Rect, Color};
use wgpu::{Device, RenderPass, TextureFormat};

use crate::{image::Image, image_state::ImageState, rect_state::RectState};

#[derive(Debug)]
pub struct WGPUDrawer {
    rect_state:      RectState,
    pub image_state: ImageState,
}

impl WGPUDrawer {
    pub fn new(device: &Device, texture_format: TextureFormat) -> Result<Self> {
        Ok(Self {
            rect_state:  RectState::new(device, texture_format),
            image_state: ImageState::new(device, texture_format)?,
        })
    }
}

impl WGPUDrawer {
    pub fn fill_rect<'a>(
        &'a self,
        device: &Device,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        color: &Color,
        _priority: usize,
    ) {
        self.rect_state.draw(device, render_pass, rect, color);
    }

    pub fn outline_rect(&self, _rect: &Rect, _color: &Color, _priority: usize) {
        todo!()
    }

    pub fn draw_image(&self, _image: &Image, _rect: &Rect, _color: &Color, _priority: usize, _is_text: bool) {
        todo!()
    }

    pub fn draw_path(
        &self,
        _path: &PathData,
        _rect: &Rect,
        _custom_mode: Option<DrawMode>,
        _priority: usize,
    ) {
        todo!()
    }

    // pub fn draw(&self, _view: &dyn View) {
    //     todo!()
    // }

    pub fn set_root_frame(&mut self, _frame: Rect) {
        todo!()
    }
}
