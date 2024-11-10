//! # Utility functions.
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

use super::*;

/// An utility function for axial round.
/// for more information, see the [documentation](https://www.redblobgames.com/grids/hexagons/#rounding).
///
/// # Example
///
/// ```
/// use hexing::HexPosition;
///
/// let position = (0.75, 0.1);
/// let rounded = hexing::utils::axial_round(position);
/// assert_eq!(rounded, HexPosition(1, 0));
/// ```
pub fn axial_round(pos: (f32, f32)) -> HexPosition<i32> {
    let (q, r) = pos;
    let s = -q - r;

    let (rq, rr, rs) = (q.round(), r.round(), s.round());
    let (q_diff, r_diff, s_diff) = ((rq - q).abs(), (rr - r).abs(), (rs - s).abs());

    if q_diff > r_diff && q_diff > s_diff {
        HexPosition((-rr - rs) as i32, rr as i32)
    } else if r_diff > s_diff {
        HexPosition(rq as i32, (-rq - rs) as i32)
    } else {
        HexPosition(rq as i32, rr as i32)
    }
}

/// An utility function for lerp between two hexagonal positions.
/// for more information, see the [documentation](https://www.redblobgames.com/grids/hexagons/#line-drawing).
///
/// # Example
///
/// ```
/// use hexing::HexPosition;
///
/// let position = HexPosition(0, 0);
/// let other_position = HexPosition(-4, 2);
/// let lerp = hexing::utils::hexagonal_lerp(position, other_position, 0.5);
/// assert_eq!(lerp, (-2.0, 1.0));
/// ```
pub fn hexagonal_lerp<T: Number>(a: HexPosition<T>, b: HexPosition<T>, t: f32) -> (f32, f32) {
    let a = (a.0.to_f32(), a.1.to_f32());
    let b = (b.0.to_f32(), b.1.to_f32());

    (lerp(a.0, b.0, t), lerp(a.1, b.1, t))
}

/// An utility function for linear interpolation.
/// for more information, see the [documentation](https://en.wikipedia.org/wiki/Linear_interpolation).
///
/// # Example
///
/// ```
/// let a = 1.0;
/// let b = 2.0;
/// let t = 0.5;
/// let lerp = hexing::utils::lerp(a, b, t);
/// assert_eq!(lerp, 1.5);
/// ```
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a.mul_add(1.0 - t, b * t)
}

/// An utility function for clamp.
/// for more information, see the [documentation](https://en.wikipedia.org/wiki/Clamping_(computing)).
///
/// # Example
///
/// ```
/// let value = 5;
/// let min = 0;
/// let max = 10;
/// let clamp = hexing::utils::clamp(value, min, max);
/// assert_eq!(clamp, 5);
/// ```
pub fn clamp<T: Number>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// An utility function for getting the neighbors of a hexagonal position.
/// for more information, see the [documentation](https://www.redblobgames.com/grids/hexagons/#neighbors).
///
/// # Example
///
/// ```
/// use hexing::HexPosition;
///
/// let position = HexPosition(0, 0);
/// let neighbors = hexing::utils::neighbors(position);
/// assert_eq!(neighbors.len(), 6);
/// ```
pub fn neighbors<T: Number>(pos: HexPosition<T>) -> Vec<HexPosition<T>> {
    let mut neighbors = Vec::with_capacity(6);
    for direction in HexDirection::iter() {
        let neighbor = pos + direction.to_vector();
        neighbors.push(neighbor);
    }
    neighbors
}

/// The `HexBound` struct defines a hexagonal area centered around a specific hexagonal position.
///
/// This struct encapsulates a central position (`HexPosition<T>`) and a radius (`usize`).
/// It allows checking whether a given position is within this hexagonal area.
///
/// Note that this feature is inspired by the [hexx](https://github.com/ManevilleF/hexx?tab=readme-ov-file#wrapping) project,
/// which provides similar tools for working with hexagonal grids in Rust.
///
/// # Type Parameters
/// - `T`: A type that implements the `Number` trait. This type is used for the hexagonal coordinates in the `HexPosition` struct.
///
/// # Example
/// ```
/// use hexing::{utils::HexBound, HexPosition};
///
/// let center = HexPosition::new(0, 0);
/// let bound = HexBound::new(center, 2);
/// let pos = HexPosition::new(1, -1);
/// assert!(bound.contains(pos));
/// ```
pub struct HexBound<T: Number>(HexPosition<T>, usize);

impl<T: Number> HexBound<T> {
    /// Creates a new hexagonal area with a given center and radius.
    ///
    /// # Arguments
    /// - `center`: The central position of the hexagonal area.
    /// - `radius`: The radius of the hexagonal area.
    ///
    /// # Returns
    /// Returns a new instance of `HexBound`.
    ///
    /// # Example
    /// ```
    /// use hexing::{utils::HexBound, HexPosition};
    ///
    /// let center = HexPosition::new(0, 0);
    /// let bound = HexBound::new(center, 2);
    /// ```
    pub const fn new(center: HexPosition<T>, radius: usize) -> Self {
        Self(center, radius)
    }

    /// Returns the radius of the hexagonal area.
    ///
    /// # Returns
    /// Returns a `usize` representing the radius of the area.
    ///
    /// # Example
    /// ```
    /// use hexing::{utils::HexBound, HexPosition};
    ///
    /// let bound = HexBound::new(HexPosition::new(0, 0), 2);
    /// assert_eq!(bound.radius(), 2);
    /// ```
    pub const fn radius(&self) -> usize {
        self.1
    }

    /// Returns the central position of the hexagonal area.
    ///
    /// # Returns
    /// Returns a `HexPosition<T>` representing the central position of the area.
    ///
    /// # Example
    /// ```
    /// use hexing::{utils::HexBound, HexPosition};
    ///
    /// let center = HexPosition::new(0, 0);
    /// let bound = HexBound::new(center, 2);
    /// assert_eq!(bound.center(), center);
    /// ```
    pub const fn center(&self) -> HexPosition<T> {
        self.0
    }

    /// Checks if a given position is within the defined hexagonal area.
    ///
    /// # Arguments
    /// - `pos`: The hexagonal position to check.
    ///
    /// # Returns
    /// Returns `true` if the position is within the hexagonal area, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use hexing::{utils::HexBound, HexPosition};
    ///
    /// let center = HexPosition::new(0, 0);
    /// let bound = HexBound::new(center, 2);
    /// let pos = HexPosition::new(1, -1);
    /// assert!(bound.contains(pos));
    ///
    /// let pos_outside = HexPosition::new(3, -3);
    /// assert!(!bound.contains(pos_outside));
    /// ```
    pub fn contains(&self, pos: HexPosition<T>) -> bool {
        if self.0.distance(pos).to_isize() > self.1 as isize {
            return false;
        }
        true
    }
}
