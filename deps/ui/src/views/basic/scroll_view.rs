use gm::{flat::Size, IntoF32};
use refs::Weak;
use ui_proc::view;
use vents::Event;

use crate::{
    view::{ViewData, ViewFrame, ViewSubviews},
    Slider, Sub, UIManager, ViewCallbacks, ViewSetup,
};
mod test_engine {
    pub(crate) use refs;

    pub(crate) use crate as ui;
}

#[view]
pub struct ScrollView {
    slider:        Sub<Slider>,
    content_size:  Size,
    pub on_scroll: Event<f32>,
}

impl ScrollView {
    pub fn remove_all_subviews(&mut self) {
        let slider_addr = self.slider.addr();

        for mut view in self.subviews_mut() {
            if view.addr() == slider_addr {
                continue;
            }

            view.remove_from_superview();
        }
    }

    fn max_offset(&self) -> f32 {
        -(self.content_size.height - self.height())
    }

    pub fn content_offset(&self) -> f32 {
        self.content_offset
    }

    pub fn set_content_offset(&mut self, offset: impl IntoF32) -> &mut Self {
        self.content_offset = offset.into_f32();

        if self.content_offset < self.max_offset() {
            self.content_offset = self.max_offset()
        }

        self
    }

    pub fn set_content_size(&mut self, size: impl Into<Size>) -> &mut Self {
        self.content_size = size.into();
        self
    }

    pub fn set_content_width(&mut self, width: impl IntoF32) -> &mut Self {
        self.content_size.width = width.into_f32();
        self
    }

    pub fn set_content_height(&mut self, height: impl IntoF32) -> &mut Self {
        self.content_size.height = height.into_f32();

        if self.content_offset < self.max_offset() {
            self.content_offset = self.max_offset()
        }

        self
    }
}

impl ViewSetup for ScrollView {
    fn setup(mut self: Weak<Self>) {
        self.dont_hide = true;
        self.slider.place().w(40).r(0);
        self.slider.on_change.val(move |val| {
            let val = 1.0 - val;
            let range = self.content_size.height - self.height();
            self.content_offset = -range * val;
            self.on_scroll.trigger(self.content_offset);
        });

        UIManager::on_scroll(self, move |scroll| {
            self.on_scroll(scroll.y);
        });

        self.size_changed().sub(move || {
            self.on_scroll(0.0);
        })
    }
}

impl ViewCallbacks for ScrollView {
    fn update(&mut self) {
        let co = self.content_offset;
        self.slider.set_y(-co);
        let height = self.height();
        self.slider.set_height(height);
        let hidden = self.height() >= self.content_size.height;
        self.slider.set_hidden(hidden);
    }

    fn content_size(&self) -> &Size {
        &self.content_size
    }
}

impl ScrollView {
    fn on_scroll(mut self: Weak<Self>, scroll: f32) {
        if self.height() >= self.content_size.height {
            return;
        }
        self.content_offset += scroll;
        let range = self.content_size.height - self.height();
        self.content_offset = self.content_offset.clamp(-range, 0.0);
        let slider_val = -self.content_offset / range;
        self.slider.set_value_without_event(1.0 - slider_val);

        self.on_scroll.trigger(self.content_offset);
    }
}
