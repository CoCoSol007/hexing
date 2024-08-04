<a id="readme-top"></a>
<div align="center">

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![License][license-shield]][license-url]

</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <img src="https://github.com/CoCoSol007/hexing/blob/main/logo.png" alt="Logo" width="300"></p>
  <h3 align="center">Hexing</h3>
  <p align="center">
    <br />
    <i>"Hexagons are the bestagons"</i>
    <br />
    A basic Rust library to manipulate hexagonal grids.
  </p>
</div>

---

`hexing` is a Rust library designed for manipulation and calculations on hexagonal grids. It provides tools for working with hexagonal positions and directions, as well as iterators for exploring hexagonal rings and spirals.

### Features

- **Hexagonal Coordinate Manipulation**: Represent and manipulate positions in a hexagonal grid using axial coordinates.
- **Distance Calculations**: Compute the distance between two hexagonal positions.
- **Pixel Coordinate Conversion**: Convert hexagonal positions to pixel coordinates for graphical use.
- **Reflection and Rotation**: Apply reflection and rotation to hexagonal positions.
- **Ring and Spiral Iterators**: Obtain positions in a ring or spiral around a central position.
- **Line Iterators**: Obtain positions along a line between two hexagonal positions.

### Number Trait

The library uses the `Number` trait to allow generic calculations with various numeric types. This trait is implemented for several types, including integers and floating-point numbers.

### Main Types

#### `HexPosition<T>`

Represents a position in a hexagonal grid with coordinates `T`. Coordinates are in axial format `(x, y)`.

- **Creation**:
Creates a new [HexPosition] with the given `q` and `r` coordinates in an axial format. Coordinates are explained in this [documentation](https://www.redblobgames.com/grids/hexagons/#coordinates).

  ```rust
  let pos = HexPosition::new(1, 2);
  let pos2 = HexPosition(3, 4);

  // Constant: The origin of the hexagonal grid.
  let origin = HexPosition::ORIGIN;
  ```

- **Conversion to Pixel Coordinates**:
Converts the current [HexPosition] into a pixel coordinate. Basically, it converts a position in a hexagonal grid to a position in a orthogonal grid.

  ```rust
  let pixel_coords = pos.to_pixel_coordinates();
  let pixel_coords2 = HexPosition::from_pixel_coordinates(pixel_coords);
  ```

- **Distance Calculation**:
Calculates the distance between two hexagonal positions, using the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry).

  ```rust
  let distance = pos.distance(HexPosition::new(3, 4));
  ```

- **Rotation**:
Will apply a rotation of 2 x 60 degrees around the origin.

  ```rust
  let rotated_pos = pos.rotation(2);
  ```

- **Reflection**:
Will apply a central symmetric reflection around the origin.

  ```rust
  let reflected_pos = pos.reflect();
  ```

- **Ring Iterators**:
A iterator that returns positions in a ring around a central position. The iterator will return positions in a ring with the given radius.

  ```rust
  for ring_pos in pos.ring(2) {
      println!("{:?}", ring_pos);
  }
  ```

- **Spiral Iterators**:
A iterator that returns positions in a spiral around a central position. The iterator will return positions in a spiral with the given radius.

  ```rust
  for spiral_pos in pos.spiral(2) {
      println!("{:?}", spiral_pos);
  }
  ```

- **Line Iterators**
A iterator that returns positions along a line between two hexagonal positions.

  ```rust
  let a = HexPosition(0, 0);
  let b = HexPosition(-2, -1);
  for line_pos in a.line(b) {
      println!("{:?}", line_pos);
  }
  ```

#### `HexDirection`

Enum representing the six possible directions in a hexagonal grid.

- **Available Directions**:
  - `Right` (1, 0)
  - `UpRight` (1, -1)
  - `UpLeft` (0, -1)
  - `Left` (-1, 0)
  - `DownLeft` (-1, 1)
  - `DownRight` (0, 1)

- **Convert to Vector**:
You can convert a [HexDirection] to a [HexPosition] by using the `to_vector` method.

  ```rust
  let direction = HexDirection::Right;
  let vector = direction.to_vector();
  ```

### Usage Examples

Here are some examples to illustrate the features of `hexing`.

#### Creating Hexagonal Positions

```rust
use hexing::HexPosition;

let pos = HexPosition::new(1, 2);
let pos2 = HexPosition(3, 4);
let origin = HexPosition::ORIGIN;

println!("Position 1: {:?}", pos);
println!("Position 2: {:?}", pos2);
println!("Origin: {:?}", origin);
```

#### Conversion to Pixel Coordinates

```rust
use hexing::HexPosition;

let position = HexPosition::new(1, 0);
let pixel_coords = position.to_pixel_coordinates();
println!("Pixel coordinates: {:?}", pixel_coords);

let new_position: HexPosition<i32> = HexPosition::from_pixel_coordinates(pixel_coords);
println!("New position: {:?}", new_position);

assert!(position == new_position);
```

#### Calculating Distance Between Positions

```rust
use hexing::HexPosition;

let pos1 = HexPosition::new(0, 0);
let pos2 = HexPosition::new(-2, -1);
let dist = pos1.distance(pos2);
println!("Distance: {:?}", dist);
```

#### Iterating Over Rings and Spirals

```rust
use hexing::{HexPosition, HexRing, HexSpiral};

let center = HexPosition::new(0, 0);

// Ring of radius 1
let ring = center.ring(1);
for pos in ring {
    println!("Ring position: {:?}", pos);
}

// Spiral of radius 2
let spiral = center.spiral(2);
for pos in spiral {
    println!("Spiral position: {:?}", pos);
}
```

#### Rotation of Hexagonal Position

```rust
use hexing::HexPosition;
let rotation = 120;
let pos = HexPosition::new(1, 2);
let rotated_pos = pos.rotation(rotation/60); // Rotates 120 degrees
println!("Rotated Position: {:?}", rotated_pos);
```

#### Reflection of Hexagonal Position

```rust
use hexing::HexPosition;

let pos = HexPosition::new(1, 2);
let reflected_pos = pos.reflect();
println!("Reflected Position: {:?}", reflected_pos);
```

#### Line Iterator

```rust
use hexing::HexPosition;

let start = HexPosition::new(0, 0);
let end = HexPosition::new(3, -3);
for pos in start.line(end) {
    println!("Line Position: {:?}", pos);
}
```

#### Using HexDirection

```rust
use hexing::HexDirection;

let direction = HexDirection::UpRight;
let vector = direction.to_vector();
println!("Vector for Right Direction: {:?}", vector);

let new_position = HexPosition::new(0, 0) + vector * 3;
println!("New Position after moving Right: {:?}", new_position);
```

### Full Documentation

For more detailed documentation and additional explanations about hexagonal grids, please refer to the [Red Blob Games hexagonal grid documentation](https://www.redblobgames.com/grids/hexagons/).

### Installation

Add `hexing` to your `Cargo.toml`:

```toml
[dependencies]
hexing = "0.1.3"
```

<!-- You have to change every link to the great repo -->

[contributors-shield]: https://img.shields.io/github/contributors/cocosol007/hexing.svg?style=for-the-badge
[contributors-url]: https://github.com/cocosol007/hexing/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/cocosol007/hexing.svg?style=for-the-badge
[forks-url]: https://github.com/cocosol007/hexing/network/members
[stars-shield]: https://img.shields.io/github/stars/cocosol007/hexing.svg?style=for-the-badge
[stars-url]: https://github.com/cocosol007/hexing/stargazers
[issues-shield]: https://img.shields.io/github/issues/cocosol007/hexing.svg?style=for-the-badge
[issues-url]: https://github.com/cocosol007/hexing/issues
[license-shield]: https://img.shields.io/github/license/cocosol007/hexing.svg?style=for-the-badge
[license-url]: https://github.com/cocosol007/hexing/blob/main/LICENSE
