use std::ops::Range;

use bytemuck::cast_slice;
use gm::{
    flat::{Point, Rect},
    num::checked_convert::checked_usize_to_u32,
    Color,
};
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupLayout, Buffer, BufferUsages, Device, IndexFormat, PipelineLayoutDescriptor, PolygonMode,
    RenderPass, RenderPipeline, TextureFormat,
};

use crate::{render::uniform::Uniform, utils::make_pipeline};

const VERTICES: &[Point] = &[
    Point::new(-1.0, 1.0),
    Point::new(-1.0, -1.0),
    Point::new(1.0, 1.0),
    Point::new(1.0, -1.0),
];

const INDICES: &[u16] = &[0, 1, 2, 1, 2, 3];

const VERTEX_RANGE: Range<u32> = 0..checked_usize_to_u32(VERTICES.len());
const INDEX_RANGE: Range<u32> = 0..checked_usize_to_u32(INDICES.len());

#[derive(Debug)]
pub struct RectState {
    color_group_layout: BindGroupLayout,
    z_layout:           BindGroupLayout,
    fill_pipeline:      RenderPipeline,
    line_pipeline:      RenderPipeline,
    vertex_buffer:      Buffer,
    index_buffer:       Buffer,
}

impl RectState {
    pub fn new(device: &Device, texture_format: TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shaders/rect.wgsl"));

        let z_layout = Uniform::z_layout(device);
        let color_group_layout = Uniform::color_layout(device);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:                Some("Rect Pipeline Layout"),
            bind_group_layouts:   &[&z_layout, &color_group_layout],
            push_constant_ranges: &[],
        });

        let fill_pipeline = make_pipeline::<Point>(
            "Rect Fill Render Pipeline",
            device,
            &pipeline_layout,
            &shader,
            texture_format,
            PolygonMode::Fill,
        );

        let line_pipeline = make_pipeline::<Point>(
            "Rect Line Render Pipeline",
            device,
            &pipeline_layout,
            &shader,
            texture_format,
            PolygonMode::Line,
        );

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Rect Vertex Buffer"),
            contents: cast_slice(VERTICES),
            usage:    BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    Some("Index Buffer"),
            contents: cast_slice(INDICES),
            usage:    BufferUsages::INDEX,
        });

        Self {
            color_group_layout,
            z_layout,
            fill_pipeline,
            line_pipeline,
            vertex_buffer,
            index_buffer,
        }
    }

    fn pipeline(&self, polygon_mode: PolygonMode) -> &RenderPipeline {
        match polygon_mode {
            PolygonMode::Fill => &self.fill_pipeline,
            PolygonMode::Line => &self.line_pipeline,
            PolygonMode::Point => unimplemented!(),
        }
    }

    fn draw_vertices<'a>(&'a self, render_pass: &mut RenderPass<'a>, polygon_mode: PolygonMode) {
        match polygon_mode {
            PolygonMode::Fill => render_pass.draw(VERTEX_RANGE, 0..1),
            PolygonMode::Line => {
                render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(INDEX_RANGE, 0, 0..1);
            }
            PolygonMode::Point => unimplemented!(),
        }
    }

    pub fn draw<'a>(
        &'a self,
        device: &Device,
        render_pass: &mut RenderPass<'a>,
        rect: &Rect,
        color: &Color,
        polygon_mode: PolygonMode,
        z_position: f32,
    ) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0.0, 1.0);
        render_pass.set_pipeline(self.pipeline(polygon_mode));

        render_pass.set_bind_group(0, Uniform::z(device, &self.z_layout, z_position), &[]);
        render_pass.set_bind_group(1, Uniform::color(device, &self.color_group_layout, color), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        self.draw_vertices(render_pass, polygon_mode);
    }
}
