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
  <img src="logo.png" alt="Logo" width="300"></p>
  <h3 align="center">Hexing</h3>
  <p align="center">
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
- **Ring and Spiral Iterators**: Obtain positions in a ring or spiral around a central position.

### Number Trait

The library uses the `Number` trait to allow generic calculations with various numeric types. This trait is implemented for several types, including integers and floating-point numbers.

### Main Types

#### `HexPosition<T>`

Represents a position in a hexagonal grid with coordinates `T`. Coordinates are in axial format `(x, y)`.

- **Creation**:

  ```rust
  let pos = HexPosition::new(1, 2);

  // Constant: The origin of the hexagonal grid.
  let origin = HexPosition::ORIGIN;
  ```

- **Conversion to Pixel Coordinates**:

  ```rust
  let pixel_coords = pos.to_pixel_coordinates();
  ```

- **Distance Calculation**:

  ```rust
  let distance = pos.distance(HexPosition::new(3, 4));
  ```

- **Ring Iterators**:

  ```rust
  for ring_pos in pos.ring(2) {
      println!("{:?}", ring_pos);
  }
  ```

- **Spiral Iterators**:

  ```rust
  for spiral_pos in pos.spiral(2) {
      println!("{:?}", spiral_pos);
  }
  ```

#### `HexDirection`

Enum representing the six possible directions in a hexagonal grid.

- **Available Directions**:
  - `Right`
  - `UpRight`
  - `UpLeft`
  - `Left`
  - `DownLeft`
  - `DownRight`

- **Convert to Vector**:

  ```rust
  let direction = HexDirection::Right;
  let vector = direction.to_vector();
  ```

### Usage Examples

Here are some examples to illustrate the features of `hexing`.

#### Conversion to Pixel Coordinates

```rust
use hexing::HexPosition;

let position = HexPosition::new(1, 0);
let pixel_coords = position.to_pixel_coordinates();
println!("Pixel coordinates: {:?}", pixel_coords);
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

### Full Documentation

For more detailed documentation and additional explanations about hexagonal grids, please refer to the [Red Blob Games hexagonal grid documentation](https://www.redblobgames.com/grids/hexagons/).

### Installation

Add `hexing` to your `Cargo.toml`:

```toml
[dependencies]
hexing = "0.1.1"
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
