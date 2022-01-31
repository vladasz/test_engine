use gl_image::Image;
use rtools::Event;

use crate::{view_base::ViewBase, Touch, View};

#[derive(Default)]
pub struct Button {
    base:       ViewBase,
    pub on_tap: Event,
    pub image:  Option<Image>,
}

impl View for Button {
    fn on_touch(&mut self, touch: &Touch) {
        if touch.is_began() {
            self.on_tap.trigger(());
        }
    }

    fn setup(&mut self) {
        self.enable_touch();
    }

    fn image(&self) -> Option<Image> {
        self.image.clone()
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
