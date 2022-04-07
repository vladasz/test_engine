use gl_image::Image;
use gm::flat::Direction;
use rtools::{Event, Rglica, ToRglica};

use crate::{
    basic::Button,
    view_base::{add_view, ViewBase},
    View,
};

#[derive(Default, Debug)]
pub struct DPadView {
    base:         ViewBase,
    up:           Rglica<Button>,
    down:         Rglica<Button>,
    left:         Rglica<Button>,
    right:        Rglica<Button>,
    pub on_press: Event<Direction>,
}

impl DPadView {
    pub fn set_images(&mut self, up: Image, down: Image, left: Image, right: Image) {
        self.up.set_image(up);
        self.down.set_image(down);
        self.left.set_image(left);
        self.right.set_image(right);
    }
}

impl View for DPadView {
    fn setup(&mut self) {
        self.up = add_view(self);
        self.down = add_view(self);
        self.left = add_view(self);
        self.right = add_view(self);

        let mut this = self.to_rglica();
        self.up
            .on_tap
            .subscribe(move |_| this.on_press.trigger(Direction::Up));

        let mut this = self.to_rglica();
        self.down
            .on_tap
            .subscribe(move |_| this.on_press.trigger(Direction::Down));

        let mut this = self.to_rglica();
        self.left
            .on_tap
            .subscribe(move |_| this.on_press.trigger(Direction::Left));

        let mut this = self.to_rglica();
        self.right
            .on_tap
            .subscribe(move |_| this.on_press.trigger(Direction::Right));
    }

    fn layout(&mut self) {
        let frame = self.frame();
        let third = frame.width() / 3.0;
        let half = frame.height() / 2.0;

        self.up.set_frame((third, 0, third, half).into());
        self.down.set_frame((third, half, third, half).into());
        self.left.set_frame((0, half, third, half).into());
        self.right
            .set_frame((third * 2.0, half, third, half).into());
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}
