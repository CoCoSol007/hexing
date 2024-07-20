//! Utility functions.

use super::*;

/// An utility function for axial round.
/// for more information, see the [documentation](https://www.redblobgames.com/grids/hexagons/#rounding).
///
/// # Example
///
/// ```
/// use hexing::HexPosition;
///
/// let position = HexPosition(0.75, 0.1);
/// let rounded = hexing::utils::axial_round(position);
/// assert_eq!(rounded, HexPosition(1, 0));
/// ```
pub fn axial_round(pos: HexPosition<f32>) -> HexPosition<i32> {
    let (q, r) = (pos.0, pos.1);
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
