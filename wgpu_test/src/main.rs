#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(let_chains)]

mod wgpu_test_view;

use std::ops::{Deref, DerefMut};

use anyhow::Result;
use log::warn;
use test_engine::{
    assets::Assets,
    gm::{
        axis::Axis,
        flat::{IntSize, Rect, Size},
        Color,
    },
    manage::data_manager::DataManager,
    paths::git_root,
    ui::{refs::Own, Container, View, ViewAnimation, ViewData, ViewFrame, ViewLayout, ViewSubviews},
    ui_views::{ImageView, Label},
    wgpu_wrapper::{
        app::App,
        text::Font,
        wgpu::RenderPass,
        wgpu_app::WGPUApp,
        wgpu_drawer::WGPUDrawer,
        wgpu_text::glyph_brush::{BuiltInLineBreaker, HorizontalAlign, Layout, Section, Text, VerticalAlign},
    },
};

use crate::wgpu_test_view::WGPUTestView;

struct TEApp {
    pub(crate) root_view: Own<dyn View>,
}

impl TEApp {
    fn rescale_frame(rect: &Rect, display_scale: f32, window_size: Size) -> Rect {
        (
            rect.origin.x * display_scale,
            (window_size.height - rect.origin.y - rect.size.height) * display_scale,
            rect.size.width * display_scale,
            rect.size.height * display_scale,
        )
            .into()
    }

    fn update_view(&self, view: &mut dyn View) {
        if view.is_hidden() {
            return;
        }
        view.layout();
        view.commit_animations();
        view.calculate_absolute_frame();
        view.update();
        for view in view.subviews_mut() {
            self.update_view(view.deref_mut());
        }
    }

    fn draw<'a>(
        &'a self,
        pass: &mut RenderPass<'a>,
        drawer: &'a WGPUDrawer,
        view: &'a dyn View,
        sections: &mut Vec<Section<'a>>,
    ) {
        if view.is_hidden() {
            return;
        }

        if view.absolute_frame().size.is_invalid() {
            warn!(
                "View has invalid frame: {}. Frame: {:?} ",
                view.label(),
                view.frame()
            );
            return;
        }

        let frame = Self::rescale_frame(view.absolute_frame(), 1.0, drawer.window_size);

        drawer.fill_rect(pass, &frame, view.color());

        if let Some(image_view) = view.as_any().downcast_ref::<ImageView>() {
            if image_view.image.is_ok() {
                let image = image_view.image;
                let size: Size = image.size.into();
                let frame = &size.fit_in_rect::<{ Axis::X }>(view.absolute_frame());
                let frame = Self::rescale_frame(frame, 1.0, drawer.window_size);

                drawer.draw_image(pass, image.get_static(), &frame);
            }
        } else if let Some(label) = view.as_any().downcast_ref::<Label>()
            && !label.text.is_empty()
        {
            let center = frame.center();

            let section = Section::default()
                .add_text(Text::new(&label.text).with_scale(64.0).with_color(Color::BLACK.as_slice()))
                .with_bounds((frame.width(), frame.height()))
                .with_layout(
                    Layout::default()
                        .v_align(VerticalAlign::Center)
                        .h_align(HorizontalAlign::Center)
                        .line_breaker(BuiltInLineBreaker::UnicodeLineBreaker),
                )
                .with_screen_position((center.x, center.y));

            sections.push(section);
        }

        for view in view.subviews() {
            if view.dont_hide() || view.absolute_frame().intersects(self.root_view.frame()) {
                self.draw(pass, drawer, view.deref(), sections)
            }
        }
    }
}

impl Default for TEApp {
    fn default() -> Self {
        Self {
            root_view: Container::make_root_view(),
        }
    }
}

impl App for TEApp {
    fn window_ready(&mut self) {
        let view = self.root_view.add_view::<WGPUTestView>();
        view.place().back();
        self.update();
    }

    fn update(&mut self) {
        self.update_view(self.root_view.weak().deref_mut());
    }

    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>, drawer: &'a WGPUDrawer) {
        let mut sections: Vec<Section> = vec![];
        self.draw(pass, drawer, self.root_view.deref(), &mut sections);

        Font::helvetice().brush.queue(&drawer.device, &drawer.queue, sections).unwrap()
    }

    fn resize(&mut self, size: IntSize) {
        self.root_view.set_size(size);
        self.update();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    Assets::init(git_root().expect("git_root()"));
    WGPUApp::start(TEApp::default(), 1200, 1200).await
}
