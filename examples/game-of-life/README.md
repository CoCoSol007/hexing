# Hexagonal Conway's Game of Life

This project demonstrates an implementation of Conway's Game of Life using a **hexagonal grid** layout. It uses the `hexing` crate to manage the hexagonal grid and `rand` to initialize the grid with random states. The project simulates the evolution of a grid of cells over multiple generations based on a set of simple rules, adapted to the hexagonal grid structure.

### Libraries Used:
- **`hexing`**: Provides utilities for creating and managing hexagonal grids, including layout management and neighbor calculations.
- **`rand`**: Used for generating random values, specifically to randomly initialize the cells as alive or dead.

---

### Function Overview

#### 1. **`main`**  
The main function serves as the entry point to the application. It creates a hexagonal grid, initializes it with random values, and then simulates 10 generations of cell evolution.

```rust
fn main() {
    let mut grid = HexLayout::<bool, isize>::new_from_range(GRID_SIZE, HexPosition(0, 0));
    initialize_grid(&mut grid);

    for _ in 0..10 {
        grid = next_generation(&grid);
    }
}
```

**Explanation**:
- **`HexLayout::<bool, isize>::new_from_range(GRID_SIZE, HexPosition(0, 0))`**: Creates a new hexagonal grid with `GRID_SIZE` cells for the radius, where each cell stores a boolean value (alive or dead).
- **`initialize_grid(&mut grid)`**: Randomly initializes the state of the cells (alive or dead).
- **`next_generation(&grid)`**: Advances the grid to the next generation by applying the Game of Life rules.

---

#### 2. **`initialize_grid`**  
This function initializes the grid with random states, where each cell has a 50% chance of being alive or dead.

```rust
fn initialize_grid(grid: &mut HexLayout<bool, isize>) {
    let mut rng = rand::thread_rng();
    for pos in HexPosition::<isize>::ORIGIN.spiral(GRID_SIZE) {
        let state = rng.gen_bool(0.5); // 50% chance of alive or dead
        grid.set(pos, state);
    }
}
```

**Explanation**:
- **`HexPosition::<isize>::ORIGIN.spiral(GRID_SIZE)`**: Generates a spiral pattern of hexagonal grid positions to iterate through, ensuring all positions in the grid are initialized.
- **`rng.gen_bool(0.5)`**: Randomly assigns a state (alive or dead) to each cell with a 50% chance.
- **`grid.set(pos, state)`**: Sets the cell at position `pos` to the generated random state.

---

#### 3. **`next_generation`**  
This function computes the next generation of the grid, applying Conway's Game of Life rules to each cell based on the number of alive neighbors it has.

```rust
fn next_generation(grid: &HexLayout<bool, isize>) -> HexLayout<bool, isize> {
    let mut next_grid = HexLayout::<bool, isize>::new_from_range(GRID_SIZE, HexPosition(0, 0));

    for pos in grid.positions() {
        let neighbors = neighbors(*pos);
        let alive_neighbors = neighbors
            .iter()
            .filter(|&&neighbor| *grid.get(neighbor).unwrap_or(&false))
            .count();

        let current_state = *grid.get(*pos).unwrap();
        let next_state = match (current_state, alive_neighbors) {
            (true, x) if x < 2 || x > 2 => false,
            (false, 2) => true,
            _ => current_state,
        };

        next_grid.set(*pos, next_state);
    }

    next_grid
}
```

**Explanation**:
- **`HexLayout::<bool, isize>::new_from_range(GRID_SIZE, HexPosition(0, 0))`**: Creates a new empty hexagonal grid to store the next generation of cells.
- **`grid.positions()`**: Iterates over all the positions in the grid.
- **`neighbors(*pos)`**: Retrieves the neighboring positions of the current cell `pos` on the hexagonal grid.
- **`grid.get(neighbor).unwrap_or(&false)`**: Retrieves the state of each neighboring cell, assuming dead (`false`) for any out-of-bounds positions.
- **`alive_neighbors`**: Counts how many of the neighboring cells are alive.
- **`match (current_state, alive_neighbors)`**: Decides the next state of the cell based on the following rules:
  - A live cell with fewer than 2 or more than 2 live neighbors dies (underpopulation or overpopulation).
  - A dead cell with exactly 2 live neighbors comes to life (reproduction).
  - Otherwise, the cell remains in its current state.
- **`next_grid.set(*pos, next_state)`**: Sets the new state of the cell at position `pos` in the `next_grid`.

