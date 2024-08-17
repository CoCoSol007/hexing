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
