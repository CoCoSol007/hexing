//! All functions related to calculations in a hexagonal grid.

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use paste::paste;

/// Represents a number that can be used in calculations for hexagonal grids.
pub trait Number:
    Copy
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + std::fmt::Debug
{
    /// The number -1.
    const MINUS_ONE: Self;

    /// The number 0.
    const ZERO: Self;

    /// The number 1.
    const ONE: Self;

    /// Returns the maximum of `self` and `other`.
    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }

    /// Returns the minimum of `self` and `other`.
    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }

    /// Returns the absolute value of `self`.
    fn abs(self) -> Self {
        if self < Self::ZERO {
            -self
        } else {
            self
        }
    }

    /// Converts an `usize` to `Self`.
    fn from_usize(value: usize) -> Self;

    /// Converts `self` to an `f32`.
    fn to_f32(self) -> f32;

    /// Converts an `f32` to `Self`.
    fn from_f32(value: f32) -> Self;
}

/// Implements the `Number` trait for the given types.
macro_rules! number_impl {
    ($($t:ty,)*) => {paste!{$(
        impl Number for $t {
            const MINUS_ONE: Self = - [< 1 $t >];
            const ZERO: Self = [< 0 $t >];
            const ONE: Self = [< 1 $t >];


            fn from_usize(value: usize) -> Self {
                value as $t
            }

            fn to_f32(self) -> f32 {
                self as f32
            }

            fn from_f32(value: f32) -> Self {
                value as $t
            }
        }
    )*}};
}

number_impl! {
    i8, i16, i32, i64, i128, isize,
    f32, f64,
}

/// Represents a position in a hexagonal grid.
/// We use the axial coordinate system explained in this
/// [documentation](https://www.redblobgames.com/grids/hexagons/#coordinates).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HexPosition<T: Number>(pub T, pub T);

/// All possible directions in a hexagonal grid.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HexDirection {
    /// The direction right.
    Right,

    /// The direction up-right.
    UpRight,

    /// The direction up-left.
    UpLeft,

    /// The direction left.
    Left,

    /// The direction down-left.
    DownLeft,

    /// The direction down-right.
    DownRight,
}

impl HexDirection {
    /// Returns the vector ([HexPosition]) of the direction.
    ///
    /// # Example
    ///
    /// ```
    /// use hexing::{HexDirection, HexPosition};
    ///
    /// let direction = HexDirection::Right;
    /// assert_eq!(direction.to_vector(), HexPosition(1, 0));
    /// ```
    pub const fn to_vector<T: Number>(self) -> HexPosition<T> {
        match self {
            Self::Right => HexPosition(T::ONE, T::ZERO),
            Self::UpRight => HexPosition(T::ONE, T::MINUS_ONE),
            Self::UpLeft => HexPosition(T::ZERO, T::MINUS_ONE),
            Self::Left => HexPosition(T::MINUS_ONE, T::ZERO),
            Self::DownLeft => HexPosition(T::MINUS_ONE, T::ONE),
            Self::DownRight => HexPosition(T::ZERO, T::ONE),
        }
    }
}

/// A hexagonal ring iterator.
pub struct HexRing<T: Number> {
    /// The current position in the ring.
    current: HexPosition<T>,

    /// The direction of the current position to the next in the ring.
    direction: HexDirection,

    /// The radius of the ring.
    radius: usize,

    /// The index of the current position in the ring.
    index: usize,
}

impl<T: Number> Iterator for HexRing<T> {
    type Item = HexPosition<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.radius {
            self.direction = match self.direction {
                HexDirection::Right => HexDirection::UpRight,
                HexDirection::UpRight => HexDirection::UpLeft,
                HexDirection::UpLeft => HexDirection::Left,
                HexDirection::Left => HexDirection::DownLeft,
                HexDirection::DownLeft => HexDirection::DownRight,
                HexDirection::DownRight => return None,
            };
            self.index = 0;
        }
        let result = self.current;
        self.current += self.direction.to_vector();
        self.index += 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = match self.direction {
            HexDirection::Right => self.radius * 6,
            HexDirection::UpRight => self.radius * 5,
            HexDirection::UpLeft => self.radius * 4,
            HexDirection::Left => self.radius * 3,
            HexDirection::DownLeft => self.radius * 2,
            HexDirection::DownRight => self.radius,
        } - self.index;
        (remaining, Some(remaining))
    }
}

/// A hexagonal spiral iterator.
pub struct HexSpiral<T: Number> {
    /// The origin of the spiral.
    origin: HexPosition<T>,

    /// The current ring of the spiral.
    current: HexRing<T>,

    /// The radius of the spiral.
    radius: usize,

    /// The index of the current ring in the spiral.
    index: usize,
}

impl<T: Number> Iterator for HexSpiral<T> {
    type Item = HexPosition<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // The origin of the spiral.
        if self.index == 0 {
            self.index += 1;
            return Some(self.origin);
        }
        if self.index > self.radius {
            return None;
        }
        let mut result = self.current.next();
        if result.is_none() && self.index < self.radius {
            self.index += 1;
            self.current = self.origin.ring(self.index);
            result = self.current.next();
        }
        result
    }
}

impl<T: Number> HexPosition<T> {
    /// Creates a new [HexPosition].
    pub const fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    /// Converts the current [HexPosition] into a pixel coordinate.
    ///
    /// If you want to learn more about pixel coordinates conversion,
    /// you can check the
    /// [documentation](https://www.redblobgames.com/grids/hexagons/#hex-to-pixel).
    ///
    /// # Example
    ///
    /// ```
    /// use hexing::HexPosition;
    ///
    /// let position = HexPosition(1, 0);
    /// assert_eq!(position.to_pixel_coordinates(), (3f32.sqrt(), 0.0).into());
    /// ```
    pub fn to_pixel_coordinates(&self) -> (f32, f32) {
        (
            3f32.sqrt()
                .mul_add(T::to_f32(self.0), 3f32.sqrt() / 2.0 * T::to_f32(self.1)),
            3.0 / 2.0 * T::to_f32(self.1),
        )
    }

    /// Returns the distance between two [HexPosition]s.
    ///
    /// # How it works
    ///
    /// In the hexagonal grid, using the
    /// [cube coordinate system](https://www.redblobgames.com/grids/hexagons/#coordinates),
    /// it's akin to a cube in 3D space.
    /// The Manhattan distance between two positions is equal to half of
    /// the sum of abs(dx) + abs(dy) + abs(dz).
    /// However, in hexagonal grids, z is defined as -q - r.
    ///
    /// # Example
    ///
    /// ```
    /// use hexing::HexPosition;
    ///
    /// let a = HexPosition(0, 0);
    /// let b = HexPosition(-2, -1);
    ///
    /// assert_eq!(a.distance(b), 3);
    /// ```
    pub fn distance(self, other: Self) -> T {
        (self.0 - other.0)
            .abs()
            .max((self.1 - other.1).abs())
            .max((-self.0 - self.1 - (-other.0 - other.1)).abs())
    }

    /// Returns the hexagonal ring of the given radius.
    /// If you want to learn more about hexagonal grids, check the
    /// [documentation](https://www.redblobgames.com/grids/hexagons/#rings)
    ///
    /// # Example
    ///
    /// ```
    /// use hexing::HexPosition;
    ///
    /// let position = HexPosition(0, 0);
    /// let radius = 1;
    ///
    /// for ring_position in position.ring(radius) {
    ///     println!("{:?}", ring_position);
    /// }
    /// ```
    pub fn ring(self, radius: usize) -> HexRing<T> {
        HexRing {
            current: self + HexDirection::DownLeft.to_vector() * T::from_usize(radius),
            direction: HexDirection::Right,
            radius,
            index: 0,
        }
    }

    /// Returns the hexagonal spiral of the given radius.
    /// If you want to learn more about hexagonal grids, check the
    /// [documentation](https://www.redblobgames.com/grids/hexagons/#rings-spiral)
    ///
    /// # Example
    ///
    /// ```
    /// use hexing::HexPosition;
    ///
    /// let position = HexPosition(0, 0);
    /// let radius = 1;
    ///
    /// for spiral_position in position.spiral(radius) {
    ///     println!("{:?}", spiral_position);
    /// }
    /// ```
    pub fn spiral(self, radius: usize) -> HexSpiral<T> {
        HexSpiral {
            origin: self,
            current: self.ring(1),
            radius,
            index: 0,
        }
    }
}

/// Implementation of the arithmetic operators for hexagonal positions.
macro_rules! impl_ops {
    ($(($t:ty, $n:ident),)*) => {paste!{$(
        impl<T: Number> $t for HexPosition<T> {
            type Output = Self;

            fn $n(self, rhs: Self) -> Self {
                Self(self.0.$n(rhs.0), self.1.$n(rhs.1))
            }
        }

        impl<T: Number> $t<T> for HexPosition<T> {
            type Output = Self;

            fn $n(self, rhs: T) -> Self {
                Self(self.0.$n(rhs), self.1.$n(rhs))
            }
        }

        impl<T: Number> [< $t Assign >] for HexPosition<T> {
            fn [< $n _assign >](&mut self, rhs: Self) {
                self.0.[< $n _assign >](rhs.0) ;
                self.1.[< $n _assign >](rhs.1) ;
            }
        }

        impl<T: Number> [< $t Assign >]<T> for HexPosition<T> {
            fn [< $n _assign >](&mut self, rhs: T) {
                self.0.[< $n _assign >](rhs);
                self.1.[< $n _assign >](rhs);
            }
        }
    )*}};
}

impl_ops! {
    (Add, add),
    (Sub, sub),
    (Mul, mul),
    (Div, div),
    (Rem, rem),
}
