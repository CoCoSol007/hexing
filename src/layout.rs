//! This module contains the layout logic for creating and managing hexagonal grids.
//!
//! The layout is a simple `HashMap` structure that facilitates the creation and manipulation of hexagonal grids.
//!
//! - `[D]` represents the data stored at each position in the grid. It must implement the `Default` trait to be initialized when creating a new, empty `HexLayout`.
//! - `[T]` denotes the type of the coordinates in the grid.
//!
//! ## Why use a `HexLayout`?
//!
//! `HexLayout` is designed to implement useful features on hexagonal grids, including:
//! - Field of view
//! - Pathfinding
//! - Movement fields
//! - Noise maps
//!
//! ## Examples
//!
//! Basic usage:
//!
//! ```rust
//! use hexing::{layout::HexLayout, HexPosition};
//!
//! let mut map: HexLayout<i32, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
//! assert_eq!(map.len(), 1);
//!
//! // Access the data using the get function
//! map.get(HexPosition(0, 0));
//!
//! // Modify the data using the set function
//! map.set(HexPosition(0, 0), 10);
//!
//! // Delete data using the delete function
//! map.delete(HexPosition(0, 0));
//!
//! assert!(map.is_empty());
//!
//! // Create unexistent data using the set function
//! map.set(HexPosition(1, 0), 10);
//!
//! assert_eq!(map.get(HexPosition(1, 0)), Some(&10));
//! ```
//!
//! ## Using Noise (Requires the `noise` feature)
//!
//! If you have enabled the `noise` feature, you can use noise functions as follows:
//!
//! ```rust
//! use hexing::{layout::HexLayout, HexPosition};
//! use noise::{Fbm, Perlin, MultiFractal};
//!
//! let fbm = Fbm::<Perlin>::new(12345)
//!     .set_octaves(2)
//!     .set_frequency(0.2)
//!     .set_lacunarity(0.4)
//!     .set_persistence(0.5);
//!
//! let mut map = HexLayout::new_from_range(10, HexPosition(0, 0));
//! map.init_noise(fbm);
//!
//! for pos in map.positions() {
//!     assert!(*map.get(*pos).unwrap() > -1.0 && *map.get(*pos).unwrap() < 1.0);
//! }
//! ```
//! Ensure to enable the `noise` feature to run this example.
//!
//! Field of view, pathfinding, and movement fields:
//! ```rust
//! use hexing::HexPosition;
//! use hexing::layout::HexLayout;
//!
//! let mut map: HexLayout<bool, isize> = HexLayout::new_from_range(5, HexPosition(0, 0));
//!
//! // Example of Field of View
//! map.set(HexPosition(0, 1), true);
//! map.set(HexPosition(1, 0), true);
//! map.set(HexPosition(0, -2), true);
//!
//! let start_pos = HexPosition(0, 0);
//! let visible_positions = map.field_of_view(start_pos, None);
//!
//! println!("Visible positions without range limit:");
//! for pos in &visible_positions {
//!     println!("{:?}", pos);
//! }
//!
//! let visible_positions_with_range = map.field_of_view(start_pos, Some(2));
//!
//! println!("Visible positions with a range of 2:");
//! for pos in &visible_positions_with_range {
//!     println!("{:?}", pos);
//! }
//!
//! // Example of Pathfinding
//! let start = HexPosition(0, 0);
//! let goal = HexPosition(2, 2);
//!
//! let path = map.pathfinding(start, goal);
//!
//! println!("Path from start to goal:");
//! for pos in &path {
//!     println!("{:?}", pos);
//! }
//!
//! // Example of Movement Fields
//! let reachable_positions = map.field_of_move(start, 2);
//!
//! println!("Positions reachable within a range of 2:");
//! for pos in &reachable_positions {
//!     println!("{:?}", pos);
//! }
//! ```

use std::collections::{HashMap, HashSet};

#[cfg(feature = "noise")]
use noise::NoiseFn;

use priority_queue::PriorityQueue;
use utils::neighbors;

use crate::*;

/// A layout structure represented by a `HashMap`, allowing for the creation and manipulation of hexagonal grids.
///
/// - `[D]` represents the data stored at each position in the grid. It must implement the `Default` trait to be initialized when creating a new, empty `HexLayout`.
/// - `[T]` denotes the type of the coordinates in the grid.
///
/// ## Why use a `HexLayout`?
///
/// `HexLayout` is designed to implement useful features on hexagonal grids, including:
/// - Field of view
/// - Pathfinding
/// - Movement fields
/// - Noise maps
///
/// ## Examples
///
/// ```rust
/// use hexing::{layout::HexLayout, HexPosition};
///
/// let mut map = HexLayout::new_from_range(1, HexPosition(0, 0));
/// assert_eq!(map.len(), 1);
///
/// // Access the data using the get function
/// map.get(HexPosition(0, 0));
///
/// // Modify the data using the set function
/// map.set(HexPosition(0, 0), 10.0);
///
/// // Create unexistent data using the set function
/// map.set(HexPosition(1, 0), 10.0);
///
/// assert_eq!(map.get(HexPosition(1, 0)), Some(&10.0));
/// ```
///
/// ## Usage in a Video Game
///
/// In a video game, hexagonal grids can be managed using the `HexLayout` structure. This allows for different data to be recorded independently.
/// For example, a `blocked_layout` structure where `T` is a `bool` can be used to record blocked positions in the grid, enabling pathfinding, field of view, and movement field calculations.
/// Another layer can be used to track the number of resources available at each hexagonal position, etc.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HexLayout<D: Default, T: Number>(HashMap<HexPosition<T>, D>);

impl<T: Default> HexLayout<T, isize> {
    /// Creates a new layout with the given range and center position.
    ///
    /// The `range` specifies the number of hexagons that will be created.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map: HexLayout<f64, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
    /// assert_eq!(map.len(), 1);
    ///
    /// let mut map2: HexLayout<u8, isize> = HexLayout::new_from_range(2, HexPosition(3, -2));
    /// assert_eq!(map2.len(), 7);
    /// ```
    pub fn new_from_range(range: usize, center: HexPosition<isize>) -> Self {
        let mut grid = HashMap::new();

        let range = range as isize - 1;

        for q in -range..=range {
            for r in -range..=range {
                let s = -q - r;
                if s >= -range && s <= range {
                    let hex_pos = HexPosition::new(q + center.0, r + center.1);
                    grid.insert(hex_pos, T::default());
                }
            }
        }
        Self(grid)
    }
}

impl<T: Number> HexLayout<f64, T> {
    /// Initializes a noise map for the layout.
    ///
    /// The data type `[D]` must be a `f64` and the noise function `[NoiseFn]` must be of dimension 2.
    ///
    /// Note: You must include the `noise` crate in your project to use this function.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    /// use noise::{Fbm, MultiFractal, Perlin};
    ///
    /// let fbm = Fbm::<Perlin>::new(12345)
    ///     .set_octaves(2)
    ///     .set_frequency(0.2)
    ///     .set_lacunarity(0.4)
    ///     .set_persistence(0.5);
    ///
    /// let mut map = HexLayout::new_from_range(1, HexPosition(0, 0));
    /// map.init_noise(fbm);
    ///
    /// for pos in map.positions() {
    ///     assert!(*map.get(*pos).unwrap() > -1.0 && *map.get(*pos).unwrap() < 1.0);
    /// }
    /// ```
    #[cfg(feature = "noise")]
    pub fn init_noise<N: NoiseFn<f64, 2>>(&mut self, noise: N) {
        let keys: Vec<_> = self.positions().cloned().collect();
        for pos in keys {
            let position = pos.to_pixel_coordinates();
            let noise_value = noise.get([position.0 as f64, position.1 as f64]);
            self.set(pos, noise_value);
        }
    }
}

impl<T: Default, S: Number> HexLayout<T, S> {
    /// Returns a reference to the data associated with the given position if it exists, otherwise returns `None`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map: HexLayout<f64, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
    /// assert_eq!(map.get(HexPosition(0, 0)), Some(f64::default()).as_ref());
    /// assert_eq!(map.get(HexPosition(0, 1)), None);
    /// ```
    pub fn get(&self, pos: HexPosition<S>) -> Option<&T> {
        self.0.get(&pos)
    }

    /// Returns a mutable reference to the data associated with the given position if it exists, otherwise returns `None`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map: HexLayout<f64, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
    /// assert_eq!(map.get_mut(HexPosition(0, 0)), Some(f64::default()).as_mut());
    /// assert_eq!(map.get_mut(HexPosition(0, 1)), None);
    /// ```
    pub fn get_mut(&mut self, pos: HexPosition<S>) -> Option<&mut T> {
        self.0.get_mut(&pos)
    }

    /// Returns a mutable reference to the data associated with the given position if it exists, otherwise returns `None`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map: HexLayout<f64, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
    /// assert_eq!(map.get_mut(HexPosition(0, 0)), Some(f64::default()).as_mut());
    /// assert_eq!(map.get_mut(HexPosition(0, 1)), None);
    /// ```
    pub fn set(&mut self, pos: HexPosition<S>, data: T) -> Option<T> {
        self.0.insert(pos, data)
    }

    /// Deletes the data at the given position if it exists. Returns the data if it existed, otherwise returns `None`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map: HexLayout<f64, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
    ///
    /// assert_eq!(map.get(HexPosition(0, 0)), Some(f64::default()).as_ref());
    ///
    /// map.delete(HexPosition(0, 0));
    /// assert_eq!(map.get(HexPosition(0, 0)), None);
    /// ```
    pub fn delete(&mut self, pos: HexPosition<S>) -> Option<T> {
        self.0.remove(&pos)
    }

    /// Returns an iterator over all the positions in the layout.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let map: HexLayout<f64, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
    ///
    /// for pos in map.positions() {
    ///     assert_eq!(*pos, HexPosition(0, 0));
    /// }
    /// ```
    pub fn positions(&self) -> impl Iterator<Item = &HexPosition<S>> {
        self.0.keys()
    }

    /// Returns an iterator over all the data in the layout.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let map: HexLayout<f64, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
    ///
    /// for data in map.data() {
    ///     assert_eq!(*data, f64::default());
    /// }
    /// ```
    pub fn data(&self) -> impl Iterator<Item = &T> {
        self.0.values()
    }

    /// Returns an iterator over all the data as mutable references in the layout.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map: HexLayout<f64, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
    ///
    /// for data in map.data_mut() {
    ///     *data = 10.0;
    /// }
    ///
    /// for data in map.data() {
    ///     assert_eq!(*data, 10.0);
    /// }
    /// ```
    pub fn data_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.values_mut()
    }

    /// Returns an iterator over all the positions and data in the layout.
    ///
    /// ## Examples
    ///
    /// ```rust
    ///
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let map: HexLayout<f64, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
    ///
    /// for (pos, data) in map.iter() {
    ///     assert_eq!(*pos, HexPosition(0, 0));
    ///     assert_eq!(*data, f64::default());
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (&HexPosition<S>, &T)> {
        self.0.iter()
    }

    /// Returns the number of positions in the layout.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let map: HexLayout<f64, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
    ///
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the layout is empty, otherwise returns `false`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map: HexLayout<f64, isize> = HexLayout::new_from_range(1, HexPosition(0, 0));
    ///
    /// assert!(!map.is_empty());
    ///
    /// map.delete(HexPosition(0, 0));
    ///
    /// assert!(map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clears all data from the layout, leaving it empty.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map: HexLayout<f64, isize> = HexLayout::new_from_range(7, HexPosition(0, 0));
    ///
    /// assert!(!map.is_empty());
    ///
    /// map.clear();
    ///
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Computes the logical AND operation between two layouts, returning a set of positions that exist in both layouts.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map1: HexLayout<f64, isize> = HexLayout::new_from_range(3, HexPosition(0, 0));
    /// let mut map2: HexLayout<f64, isize> = HexLayout::new_from_range(2, HexPosition(-2, 0));
    ///
    /// let and = map1.and(&map2);
    ///
    /// assert_eq!(and.len(), 4);
    /// ```
    pub fn and(&self, other: &Self) -> HashSet<HexPosition<S>> {
        self.0
            .keys()
            .filter_map(|pos| {
                if other.0.contains_key(pos) {
                    Some(*pos)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Computes the logical OR operation between two layouts, returning a set of all unique positions that exist in either layout.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map1: HexLayout<f64, isize> = HexLayout::new_from_range(3, HexPosition(0, 0));
    /// let mut map2: HexLayout<f64, isize> = HexLayout::new_from_range(2, HexPosition(-2, 0));
    ///
    /// let or = map1.or(&map2);
    ///
    /// assert_eq!(or.len(), 22);
    /// ```
    pub fn or(&self, other: &Self) -> HashSet<HexPosition<S>> {
        let mut result = HashSet::with_capacity(self.0.len() + other.0.len());
        result.extend(self.0.keys().copied());
        result.extend(other.0.keys().copied());
        result
    }

    /// Computes the logical XOR operation between two layouts, returning a set of positions that exist in one layout but not the other.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map1: HexLayout<f64, isize> = HexLayout::new_from_range(3, HexPosition(0, 0));
    /// let mut map2: HexLayout<f64, isize> = HexLayout::new_from_range(2, HexPosition(-2, 0));
    ///
    /// let xor = map1.xor(&map2);
    ///
    /// assert_eq!(xor.len(), 18);
    /// ```
    pub fn xor(&self, other: &Self) -> HashSet<HexPosition<S>> {
        let mut result = HashSet::with_capacity(self.0.len() + other.0.len());

        result.extend(self.0.keys().filter(|k| !other.0.contains_key(k)).copied());
        result.extend(other.0.keys().filter(|k| !self.0.contains_key(k)).copied());

        result
    }
}

impl<S: Number> HexLayout<bool, S> {
    /// Finds the shortest path between two positions on a hexagonal grid.
    /// To use the `pathfinding` feature, the data associated with each position must be a [bool] in order to represent whether the position is blocked or not.
    /// ``True`` means that the position is blocked, and ``False`` means that the position is not blocked.
    ///
    /// # Parameters
    ///
    /// - `from`: The starting position on the hexagonal grid. This position must be a `HexPosition<S>`.
    /// - `to`: The target position on the hexagonal grid. This position must also be a `HexPosition<S>`.
    ///
    /// # Returns
    ///
    /// This function returns a `Vec<HexPosition<S>>` representing the shortest path from `from` to `to`.
    /// The path is returned in order from start to end. If `from` and `to` are the same, the vector will contain only `from`.
    ///
    /// # Panics
    ///
    /// The function will panic if either `from` or `to` are not present in the hexagonal grid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hexing::{HexPosition, layout::HexLayout};
    ///
    /// let mut map: HexLayout<bool, isize> = HexLayout::new_from_range(3, HexPosition(0, 0));
    ///
    /// map.set(HexPosition(-1, 1), true);
    /// map.set(HexPosition(1, -1), true);
    /// map.set(HexPosition(1, 0), true);
    /// map.set(HexPosition(0, 1), true);
    ///
    /// let start = HexPosition(0, 0);
    /// let goal = HexPosition(0, 2);
    ///
    /// let path = map.pathfinding(start, goal);
    ///
    /// for pos in &path {
    ///     println!("Position: {:?}", pos);
    /// }
    /// assert_eq!(path, vec![HexPosition(0, 0), HexPosition(-1, 0), HexPosition(-2, 1), HexPosition(-2, 2), HexPosition(-1, 2), HexPosition(0, 2)]);
    /// ```
    ///
    /// # Note
    ///
    /// This implementation uses the A* algorithm to guarantee finding the shortest path.
    /// The heuristic used is tailored to hexagonal grids, where the axial distance is used to estimate the cost to the destination.
    pub fn pathfinding(&self, from: HexPosition<S>, to: HexPosition<S>) -> Vec<HexPosition<S>> {
        if from == to {
            return vec![from];
        }

        if !self.0.contains_key(&from) || !self.0.contains_key(&to) {
            panic!("Position not in layout");
        }

        let mut frontier = PriorityQueue::new();
        frontier.push(from, 0);

        let mut came_from: HashMap<HexPosition<S>, Option<HexPosition<S>>> = HashMap::new();
        came_from.insert(from, None);

        let mut cost_so_far: HashMap<HexPosition<S>, i32> = HashMap::new();
        cost_so_far.insert(from, 0);

        while let Some((current, _)) = frontier.pop() {
            if current == to {
                break;
            }

            for next in self.neighbors_unblocked(current) {
                if came_from.contains_key(&next) {
                    continue;
                }

                let Some(new_cost) = cost_so_far.get(&current).map(|c| c + 1) else {
                    continue;
                };

                if !cost_so_far.contains_key(&next)
                    || Some(new_cost) < cost_so_far.get(&next).copied()
                {
                    cost_so_far.insert(next, new_cost);
                    frontier.push(next, -new_cost - next.distance(to).to_isize() as i32);
                    came_from.insert(next, Some(current));
                }
            }
        }

        let mut path = vec![to];
        let mut current = to;
        while let Some(Some(prev)) = came_from.get(&current) {
            path.push(*prev);
            current = *prev;
        }
        path.reverse();
        path
    }

    /// Calculates the positions visible from a given position on a hexagonal map.
    ///
    /// This function returns a set of positions that are visible from the `center` position.
    /// Visibility is determined based on distance and obstacles. Obstacles are defined as positions
    /// with an associated value of `true`.
    ///
    /// # Parameters
    ///
    /// - `center`: The starting position from which visibility is calculated.
    /// - `range`: An optional parameter specifying the maximum visibility range. If `None`, there is no
    ///   range limit.
    ///
    /// # Return Value
    ///
    /// Returns a `HashSet` containing the positions visible from `center`. A position is considered
    /// visible if it is within the specified range (if a radius is provided) and if there are no obstacles
    /// blocking the line of sight between `center` and the position.
    ///
    /// # Example
    ///
    /// ```
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map = HexLayout::new_from_range(6, HexPosition(0, 0));
    ///
    /// let start_pos = HexPosition::new(0, 0);
    ///
    /// map.set(HexPosition(0, 1), true);
    /// map.set(HexPosition(1, 0), true);
    /// map.set(HexPosition(-2, 0), true);
    /// map.set(HexPosition(-2, 1), true);
    /// map.set(HexPosition(0, -2), true);
    ///
    /// let reachable_positions = map.field_of_view(start_pos, None);
    ///
    /// let range = 2;
    /// let reachable_positions_with_range = map.field_of_view(start_pos, Some(range));
    ///
    /// for pos in reachable_positions {
    ///    println!("{}", pos);
    /// }
    ///
    /// for pos in reachable_positions_with_range {
    ///     println!("{}", pos);
    /// }
    /// ```
    pub fn field_of_view(
        &self,
        center: HexPosition<S>,
        range: Option<usize>,
    ) -> HashSet<HexPosition<S>> {
        let mut visibles = HashSet::new();

        for position in self.positions() {
            let mut is_visible = true;

            if let Some(radius) = range {
                if position.distance(center).to_isize() > radius as isize {
                    is_visible = false;
                }
            }

            for position_between in center.line_to(*position) {
                if !self.0.contains_key(&position_between)
                    || self.get(position_between) == Some(&true)
                {
                    is_visible = false;
                    break;
                }
            }

            if is_visible {
                visibles.insert(*position);
            }
        }

        visibles
    }

    /// Computes the set of positions reachable from a given starting position within a specified range.
    ///
    /// This function performs a breadth-first search to determine all the positions that can be reached from the
    /// starting position `pos` within a movement range of `range` steps. It accounts for obstacles or blocked positions
    /// using the `neighbors_unblocked` method, which provides a list of neighboring positions that are not blocked.
    ///
    /// # Arguments
    ///
    /// * `pos` - The starting position from which to compute the field of move. It is of type `HexPosition<S>`.
    /// * `range` - The maximum number of steps that can be taken from the starting position. It is of type `usize`.
    ///
    /// # Returns
    ///
    /// Returns a `HashSet<HexPosition<S>>` containing all the positions reachable within the specified range, including
    /// the starting position itself.
    ///
    /// # Example
    ///
    /// ```rust
    ///
    /// use hexing::HexPosition;
    /// use hexing::layout::HexLayout;
    ///
    /// let mut map = HexLayout::new_from_range(3, HexPosition(0, 0));
    ///
    /// let start_pos = HexPosition::new(0, 0);
    /// let range = 2;
    ///
    /// let reachable_positions = map.field_of_move(start_pos, range);
    /// assert_eq!(reachable_positions.len(), 19);
    ///
    /// map.set(HexPosition(0, 1), true);
    /// map.set(HexPosition(1, 0), true);
    /// map.set(HexPosition(0, -2), true);
    ///
    /// let reachable_positions = map.field_of_move(start_pos, range);
    /// assert_eq!(reachable_positions.len(), 13);
    /// ```
    ///
    /// # Notes
    ///
    /// The function uses a breadth-first approach, iterating level by level up to the specified range. It maintains
    /// a set of visited positions to avoid reprocessing and ensure that each position is added only once. The number
    /// of positions added at each level is managed to ensure the function scales efficiently with the size of the range.
    ///
    /// # Complexity
    ///
    /// The time complexity is O(n), where n is the number of positions within the specified range, assuming neighbor
    /// checks are constant-time operations.
    pub fn field_of_move(&self, pos: HexPosition<S>, range: usize) -> HashSet<HexPosition<S>> {
        let mut visited = HashSet::new();
        visited.insert(pos);
        let mut fringes = Vec::new();
        fringes.push(vec![pos]);

        for k in 1..=range {
            fringes.push(Vec::new());

            let mut to_add = Vec::with_capacity(fringes[k - 1].len() * 6);

            for pos in fringes[k - 1].iter() {
                for neighbor in self.neighbors_unblocked(*pos) {
                    if visited.contains(&neighbor) {
                        continue;
                    }

                    visited.insert(neighbor);
                    to_add.push(neighbor);
                }
            }

            fringes[k] = to_add;
        }

        visited
    }

    /// Returns a list of all the neighbors that are not blocked.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// use hexing::{layout::HexLayout, HexPosition};
    ///
    /// let mut map: HexLayout<bool, isize> = HexLayout::new_from_range(3, HexPosition(0, 0));
    ///
    /// let neighbors = map.neighbors_unblocked(HexPosition(0, 0));
    /// assert_eq!(neighbors.len(), 6);
    ///
    /// map.delete(HexPosition(0, 0));
    /// map.set(HexPosition(1, 0), true);
    ///
    /// let neighbors = map.neighbors_unblocked(HexPosition(1, 1));
    /// assert_eq!(neighbors.len(), 3); // edge of the map
    /// ```
    pub fn neighbors_unblocked(&self, pos: HexPosition<S>) -> Vec<HexPosition<S>> {
        let mut result_neighbors = Vec::with_capacity(6);
        for neighbor in neighbors(pos) {
            if !self.0.get(&neighbor).unwrap_or(&true) {
                result_neighbors.push(neighbor);
            }
        }
        result_neighbors
    }
}
