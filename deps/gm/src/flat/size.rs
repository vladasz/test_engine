use std::{
    borrow::Borrow,
    fmt::Display,
    hash::{Hash, Hasher},
    ops::{Div, Mul},
};

use bytemuck::{Pod, Zeroable};
use rtools::{IntoF32, Random};
use serde::{Deserialize, Serialize};

use crate::{
    axis::Axis,
    flat::{Point, Rect},
    num::lossy_convert::LossyConvert,
};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size<T = f32> {
    pub width:  T,
    pub height: T,
}

unsafe impl<T: Zeroable> Zeroable for Size<T> {}
unsafe impl<T: Pod> Pod for Size<T> {}

impl<T: Copy> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub const fn as_slice(&self) -> [T; 2] {
        [self.width, self.height]
    }
}

impl<T: Mul<Output = T> + Copy> Size<T> {
    pub fn area(&self) -> T {
        self.width.mul(self.height)
    }
}

impl Size<f32> {
    pub fn diagonal(&self) -> f32 {
        (self.width * self.width + self.height * self.height).sqrt()
    }

    pub fn is_valid(&self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }

    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    pub fn is_negative(&self) -> bool {
        self.width < 0.0 || self.height < 0.0
    }

    pub fn center(&self) -> Point {
        Point {
            x: self.width / 2.0,
            y: self.height / 2.0,
        }
    }

    pub fn side<const AXIS: Axis>(&self) -> f32 {
        match AXIS {
            Axis::X => self.width,
            Axis::Y => self.height,
        }
    }

    pub fn other_size<const AXIS: Axis>(&self) -> f32 {
        match AXIS {
            Axis::X => self.height,
            Axis::Y => self.width,
        }
    }

    pub fn fit_height(&self, height: impl IntoF32) -> Size {
        let ratio = height.into_f32() / self.height;
        *self * ratio
    }

    pub fn fit_width(&self, width: impl IntoF32) -> Size {
        let ratio = width.into_f32() / self.width;
        *self * ratio
    }

    pub fn ratios(&self, other: Size) -> Size {
        Size::new(other.width / self.width, other.height / self.height)
    }

    pub fn fit_in_rect<const AXIS: Axis>(&self, rect: impl Borrow<Rect>) -> Rect {
        let rect = rect.borrow();
        let ratio = rect.length::<AXIS>() / self.side::<AXIS>();
        let size = *self * ratio;
        let pos = rect.other_position::<AXIS>() + rect.other_length::<AXIS>() / 2.0
            - size.other_size::<AXIS>() / 2.0;

        let mut result: Rect = (size.width, size.height).into();

        result.set_position::<AXIS>(rect.position::<AXIS>());
        result.set_other_position::<AXIS>(pos);

        result
    }
}

impl<W: ~const IntoF32, H: ~const IntoF32> const From<(W, H)> for Size<f32> {
    fn from(tup: (W, H)) -> Self {
        Self {
            width:  tup.0.into_f32(),
            height: tup.1.into_f32(),
        }
    }
}

impl<T: IntoF32> Mul<T> for Size<f32> {
    type Output = Size;
    fn mul(self, rhs: T) -> Self::Output {
        (self.width * rhs.into_f32(), self.height * rhs.into_f32()).into()
    }
}

impl<T: IntoF32> Div<T> for Size<f32> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        (self.width / rhs.into_f32(), self.height / rhs.into_f32()).into()
    }
}

impl Hash for Size<f32> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.width.to_bits().hash(state);
        self.height.to_bits().hash(state);
    }
}

impl From<Size<u32>> for Size<f32> {
    fn from(value: Size<u32>) -> Self {
        Self {
            width:  value.width as _,
            height: value.height as _,
        }
    }
}

impl From<Size<f32>> for Size<u32> {
    fn from(value: Size<f32>) -> Self {
        Self {
            width:  value.width.lossy_convert(),
            height: value.height.lossy_convert(),
        }
    }
}

impl From<(u32, u32)> for Size<u32> {
    fn from(tup: (u32, u32)) -> Self {
        Self {
            width:  tup.0,
            height: tup.1,
        }
    }
}

impl Random for Size<u32> {
    fn random() -> Self {
        Self {
            width:  u32::random_in(200..800),
            height: u32::random_in(200..800),
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width: {:.2}, height: {:.2}", self.width, self.height)
    }
}

impl Display for Size<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width: {}, height: {}", self.width, self.height)
    }
}

#[test]
fn size_ratios() {
    let a = Size::new(2.0, 2.0);
    let b = Size::new(6.0, 12.0);

    assert_eq!(a.ratios(b), Size::new(3.0, 6.0));
}
