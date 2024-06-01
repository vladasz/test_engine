use std::ops::{Deref, DerefMut};

use gm::flat::{Point, Shape};
use rapier2d::{dynamics::RigidBodyHandle, geometry::ColliderHandle, na::Vector2, prelude::RigidBodyBuilder};
use refs::Own;

use crate::{control::Control, LevelManager, Sprite, SpriteData, ToCollider};

pub struct Body {
    collider_handle: ColliderHandle,
    rigid_handle:    RigidBodyHandle,
    sprite:          SpriteData,
}

impl Body {
    pub fn velocity(&self) -> Point {
        let vel = self.rigid_body().linvel();
        (vel.x, vel.y).into()
    }

    pub fn set_velocity(&mut self, vel: Point) -> &mut Self {
        self.rigid_body_mut().set_linvel([vel.x, vel.y].into(), true);
        self
    }

    pub fn lock_rotations(&mut self) -> &mut Self {
        self.rigid_body_mut().lock_rotations(true, true);
        self
    }

    pub fn unlock_rotation(&mut self) -> &mut Self {
        self.rigid_body_mut().lock_rotations(false, true);
        self
    }
}

impl Sprite for Body {
    fn make(shape: Shape, position: Point) -> Own<Self>
    where Self: Sized {
        let level = LevelManager::level_mut().deref_mut();

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(Vector2::new(position.x, position.y))
            .build();

        let rigid_handle = level.sets.rigid_bodies.insert(rigid_body);

        let collider = shape.make_collider().build();

        let collider_handle =
            level
                .sets
                .colliders
                .insert_with_parent(collider, rigid_handle, &mut level.sets.rigid_bodies);

        let sprite = SpriteData::make(shape, position);

        Own::new(Self {
            collider_handle,
            rigid_handle,
            sprite,
        })
    }

    fn collider_handle(&self) -> Option<ColliderHandle> {
        self.collider_handle.into()
    }

    fn rigid_handle(&self) -> Option<RigidBodyHandle> {
        self.rigid_handle.into()
    }
}

const FORCE: f32 = 10.0;
impl Control for Body {
    fn jump(&mut self) {
        self.add_impulse((0, FORCE).into());
    }

    fn go_left(&mut self) {
        self.add_impulse((-FORCE, 0).into());
    }

    fn go_right(&mut self) {
        self.add_impulse((FORCE, 0).into());
    }

    fn go_down(&mut self) {
        self.add_impulse((0, -FORCE).into());
    }

    fn add_impulse(&mut self, impulse: Point) {
        self.rigid_body_mut()
            .apply_impulse([impulse.x * 100.0, impulse.y * 100.0].into(), true);
    }
}

impl Deref for Body {
    type Target = SpriteData;
    fn deref(&self) -> &SpriteData {
        &self.sprite
    }
}

impl DerefMut for Body {
    fn deref_mut(&mut self) -> &mut SpriteData {
        &mut self.sprite
    }
}
