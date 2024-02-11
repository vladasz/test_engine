use gm::flat::{IntSize, Point};
use refs::Rglica;
use wgpu::RenderPass;
use winit::event::{ElementState, MouseButton};

use crate::{wgpu_drawer::WGPUDrawer, WGPUApp};

pub trait App {
    fn window_ready(&mut self);
    fn update(&mut self) -> bool;
    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer);
    fn resize(&mut self, size: IntSize);
    fn mouse_moved(&mut self, position: Point) -> bool;
    fn mouse_event(&mut self, state: ElementState, button: MouseButton) -> bool;
    fn set_wgpu_app(&mut self, app: Rglica<WGPUApp>);
}
