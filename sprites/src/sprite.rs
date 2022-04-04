use std::fmt::Debug;

use gl_image::Image;
use gm::{Color, Point, Rect, Size};
use rapier2d::{geometry::ColliderHandle, prelude::RigidBodyHandle};
use rtools::{address::Address, Rglica};

use crate::{Level, SpriteData};

pub trait Sprite: Debug {
    fn update(&mut self) {}

    fn size(&self) -> Size {
        self.data().size
    }

    fn position(&self) -> Point {
        self.data().position
    }

    fn rotation(&self) -> f32 {
        self.data().rotation
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.data_mut().rotation = rotation
    }

    fn contains(&self, point: Point) -> bool {
        let pos = self.position();
        let size = self.size();
        point.x >= pos.x - size.width
            && point.x <= pos.x + size.width
            && point.y >= pos.y - size.height
            && point.y <= pos.y + size.height
    }

    fn color(&self) -> Color {
        self.data().color
    }

    fn set_color(&mut self, color: Color) {
        self.data_mut().color = color
    }

    fn image(&self) -> Option<&Image> {
        self.data().image.as_ref()
    }

    fn image_mut(&mut self) -> Option<&mut Image> {
        self.data_mut().image.as_mut()
    }

    fn set_image(&mut self, image: Image) {
        self.data_mut().image = image.into()
    }

    fn rigid_body_handle(&self) -> Option<RigidBodyHandle> {
        None
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        None
    }

    fn is_selected(&self) -> bool {
        self.data().is_selected
    }

    fn set_selected(&mut self, selected: bool) {
        self.data_mut().is_selected = selected
    }

    fn remove(&mut self) {
        let address = self.address();
        self.level_mut().remove(address);
    }

    fn level(&self) -> &Rglica<dyn Level> {
        debug_assert!(self.data().level.is_ok(), "Null Level");
        &self.data().level
    }

    fn level_mut(&mut self) -> &mut Rglica<dyn Level> {
        debug_assert!(self.data().level.is_ok(), "Null Level");
        &mut self.data_mut().level
    }

    fn draw(&self) {
        self.level().drawer().draw(self.data())
    }

    fn data(&self) -> &SpriteData;
    fn data_mut(&mut self) -> &mut SpriteData;
    fn make(rect: Rect, level: Rglica<dyn Level>) -> Box<Self>
    where
        Self: Sized;
}
