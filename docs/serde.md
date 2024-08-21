# Serde Feature

The `hexing` library allows for the serialization and deserialization of hexagonal data structures through the `serde` feature. This feature is enabled by the `serde` feature of the library.

## Enabling the `serde` Feature

Simply run the command `cargo add hexing --features=serde` in your project or manually add it to your `Cargo.toml`.

```toml
[dependencies]
hexing = { version = "0.3.3", features = ["serde"] }
```

## Serialization and Deserialization with `serde`

When the `serde` feature is enabled, the following data structures automatically implement the `Serialize` and `Deserialize` traits:

- **HexPosition**
- **HexDirection**
- **HexRing**
- **HexSpiral**
- **HexLine**

**Note**: To use all the examples below, you need to add the `serde_json` library to your `Cargo.toml`.

### HexPosition

`HexPosition<T>` represents a position in a hexagonal grid. With the `serde` feature, this structure can be serialized and deserialized, making it easier to save and retrieve hexagonal positions in formats like JSON, BSON, etc.

**Example: Serialization and Deserialization**

```rust
use hexing::HexPosition;

fn main() {
    let position = HexPosition::new(1, -2);

    // Serialize to JSON
    let serialized = serde_json::to_string(&position).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize from JSON
    let deserialized: HexPosition<i32> = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
```

### HexDirection

`HexDirection` is an enumeration representing all possible directions in a hexagonal grid. With the `serde` feature, this enumeration can be serialized and deserialized.

**Example: Serialization and Deserialization**

```rust
use hexing::HexDirection;

fn main() {
    let direction = HexDirection::Right;

    // Serialize to JSON
    let serialized = serde_json::to_string(&direction).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize from JSON
    let deserialized: HexDirection = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
```

### HexRing

`HexRing<T>` is a structure that allows iteration over positions in a hexagonal ring. This structure can also be serialized and deserialized with the `serde` feature.

**Example: Serialization and Deserialization**

```rust
use hexing::{HexPosition, HexRing};

fn main() {
    let ring = HexPosition::new(0, 0).ring(2);

    // Serialize to JSON
    let serialized = serde_json::to_string(&ring).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize from JSON
    let deserialized: HexRing<i32> = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
```

### HexSpiral

`HexSpiral<T>` allows iteration over positions in a hexagonal spiral. Like the other structures, `HexSpiral<T>` is serializable and deserializable with `serde`.

**Example: Serialization and Deserialization**

```rust
use hexing::{HexPosition, HexSpiral};

fn main() {
    let spiral = HexPosition::new(0, 0).spiral(2);

    // Serialize to JSON
    let serialized = serde_json::to_string(&spiral).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize from JSON
    let deserialized: HexSpiral<i32> = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
```

### HexLine

`HexLine<T>` allows iteration over positions on a hexagonal line between two points. This structure is also serializable and deserializable.

**Example: Serialization and Deserialization**

```rust
use hexing::{HexPosition, HexLine};

fn main() {
    let line = HexPosition::new(0, 0).line_to(HexPosition::new(2, -1));

    // Serialize to JSON
    let serialized = serde_json::to_string(&line).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize from JSON
    let deserialized: HexLine<i32> = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
```
