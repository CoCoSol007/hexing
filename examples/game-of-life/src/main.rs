use hexing::{layout::HexLayout, utils::neighbors, HexPosition};
use rand::Rng;

const GRID_SIZE: usize = 10;

fn main() {
    let mut grid = HexLayout::<bool, isize>::new_from_range(GRID_SIZE, HexPosition(0, 0));
    initialize_grid(&mut grid);

    for _ in 0..10 {
        grid = next_generation(&grid);
    }
}

fn initialize_grid(grid: &mut HexLayout<bool, isize>) {
    let mut rng = rand::thread_rng();
    for pos in HexPosition::<isize>::ORIGIN.spiral(GRID_SIZE) {
        let state = rng.gen_bool(0.5); // 50% chance of alive or dead
        grid.set(pos, state);
    }
}

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
