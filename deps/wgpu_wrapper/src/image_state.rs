use anyhow::Result;
use gm::{
    flat::{Point, Rect},
    volume::UIVertex,
};
use wgpu::util::DeviceExt;

use crate::{image::Image, utils::make_pipeline};

#[derive(Debug)]
pub struct ImageState {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer:   wgpu::Buffer,
    pub num_vertices:    u32,
}

impl ImageState {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat) -> Result<Self> {
        const VERTICES: &[UIVertex] = &[
            UIVertex {
                pos: Point::new(-1.0, 1.0),
                uv:  Point::new(0.0, 0.0),
            },
            UIVertex {
                pos: Point::new(-1.0, -1.0),
                uv:  Point::new(0.0, 1.0),
            },
            UIVertex {
                pos: Point::new(1.0, 1.0),
                uv:  Point::new(1.0, 0.0),
            },
            UIVertex {
                pos: Point::new(1.0, -1.0),
                uv:  Point::new(1.0, 1.0),
            },
        ];

        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/ui_image.wgsl"));

        let bind_group_layout = Image::bind_group_layout(device);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                Some("Image Pipeline Layout"),
            bind_group_layouts:   &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = make_pipeline::<UIVertex>(
            "Image Render Pipeline",
            device,
            &pipeline_layout,
            &shader,
            texture_format,
        );

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("Image Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage:    wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = u32::try_from(VERTICES.len()).unwrap();

        Ok(Self {
            render_pipeline,
            vertex_buffer,
            num_vertices,
        })
    }

    pub fn draw<'a>(&'a self, image: &'static Image, rect: &Rect, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0.0, 1.0);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &image.bind, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);
    }
}
