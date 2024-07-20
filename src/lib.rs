//! # Hexing
//! All functions related to calculations in a hexagonal grid.
//! For more information, check the [documentation](https://crates.io/crates/hexing) or the [GitHub repository](https://github.com/cocosol007/hexing).
//! Note that all algorithms are inspired by the [Hexagonal Grid Algorithm](https://www.redblobgames.com/grids/hexagons/).
//!
//! ## Copyright (C) 2024  CoCoSol
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! ## Usages
//!
//! ```rust
//!
//! use hexing::*;
//!
//! // Create hexagonal positions
//! let position = HexPosition::new(1, -2);
//! println!("Initial position: {:?}", position);
//!
//! // Convert to pixel coordinates
//! let pixel_coords = position.to_pixel_coordinates();
//! println!("Pixel coordinates: {:?}", pixel_coords);
//!
//! // Calculate the distance between two hexagonal positions
//! let other_position = HexPosition::new(-1, 1);
//! let distance = position.distance(other_position);
//! println!("Distance between {:?} and {:?}: {:?}", position, other_position, distance);
//!
//! // Iterate over a hexagonal ring
//! let radius = 2;
//! println!("Positions in the ring with radius {}:", radius);
//! for pos in position.ring(radius) {
//!     println!("{:?}", pos);
//! }
//!
//! // Iterate over a hexagonal spiral
//! let spiral_radius = 2;
//! println!("Positions in the spiral with radius {}:", spiral_radius);
//! for pos in position.spiral(spiral_radius) {
//!     println!("{:?}", pos);
//! }
//!
//! // Rotate the position by 120 degrees (2 times 60 degrees)
//! let rotated_position = position.rotation(2);
//! println!("Position after 120 degrees rotation: {:?}", rotated_position);
//!
//! // Reflect the position
//! let reflected_position = position.reflect();
//! println!("Position after reflection: {:?}", reflected_position);
//!
//! ```
//!
//! This example demonstrates basic usage of the `hexing` library, including creating hexagonal positions, converting to pixel coordinates, calculating distances, and iterating over hexagonal rings and spirals.

pub mod utils;
use utils::axial_round;

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use lerp::Lerp;
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

/// A hexagonal line iterator.
/// For more information, see the [documentation](https://www.redblobgames.com/grids/hexagons/#line-drawing).
pub struct HexLine<T: Number> {
    /// The starting position of the line.
    start: HexPosition<T>,

    /// The ending position of the line.
    end: HexPosition<T>,

    /// The length of the line.
    max_index: u32,

    /// The index of the current position in the line.
    current_index: u32,
}

impl<T: Number> Iterator for HexLine<T> {
    type Item = HexPosition<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if the line is complete.
        if self.current_index == self.max_index + 1 {
            return None;
        }

        // Check if the line is empty.
        if self.start == self.end {
            self.current_index += 1;
            return Some(self.start);
        }

        // Calculate the next position.
        let t = self.current_index as f32 / self.max_index as f32;
        let result = axial_round(HexPosition(
            self.start.0.to_f32().lerp(self.end.0.to_f32(), t),
            self.start.1.to_f32().lerp(self.end.1.to_f32(), t),
        ));

        self.current_index += 1;
        Some(HexPosition(
            T::from_f32(result.0 as f32),
            T::from_f32(result.1 as f32),
        ))
    }
}

impl<T: Number> HexPosition<T> {
    /// Creates a new [HexPosition].
    pub const fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    /// Returns the origin of the hexagonal grid.
    /// Equivalent to `HexPosition::new(0, 0)`.
    ///
    /// # Example
    ///
    /// ```
    /// use hexing::HexPosition;
    ///
    /// let origin = HexPosition::ORIGIN;
    /// assert_eq!(origin, HexPosition::new(0, 0));
    /// ```
    pub const ORIGIN: Self = Self(T::ZERO, T::ZERO);

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

    /// Returns the line between two [HexPosition]s as a iterator.
    /// For more information about how it's calculated, check the [documentation](https://www.redblobgames.com/grids/hexagons/#line-drawing)
    ///
    /// # Example
    ///
    /// ```
    /// use hexing::HexPosition;
    ///
    /// let a = HexPosition(0, 0);
    /// let b = HexPosition(-2, -1);
    ///
    /// for pos in a.line_to(b) {
    ///     println!("{:?}", pos);
    /// }
    ///
    /// assert_eq!(a.line_to(b).count(), 4);
    ///
    /// let c = HexPosition(3, -2);
    /// assert_eq!(c.line_to(c).count(), 1);
    /// ```
    pub fn line_to(self, other: Self) -> HexLine<T> {
        HexLine {
            start: self,
            end: other,
            max_index: self.distance(other).to_f32() as u32,
            current_index: 0,
        }
    }

    /// Returns the rotation of the current [HexPosition] by 60 degrees n times.
    /// Note that the rotation is counterclockwise.
    ///
    /// # Example
    ///
    /// ```
    /// use hexing::HexPosition;
    ///
    /// let position = HexPosition(-3, 1);
    /// assert_eq!(position.rotation(2), HexPosition(2, -3));
    /// ```
    pub fn rotation(self, n: i32) -> Self {
        if n == 0 {
            self
        } else {
            let new_position = Self(-self.1, self.0 + self.1);
            new_position.rotation(n - 1)
        }
    }

    /// Returns the reflection of the current [HexPosition].
    /// The reflection is the position with the same distance from the origin but in the opposite direction.
    /// (like a central symmetry)
    ///
    /// # Example
    ///
    /// ```
    /// use hexing::HexPosition;
    ///
    /// let position = HexPosition(1, 0);
    /// assert_eq!(position.reflect(), HexPosition(-1, 0));
    /// ```
    pub fn reflect(self) -> Self {
        Self::new(-self.0, -self.1)
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
