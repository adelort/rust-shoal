mod fish;
mod grid;
mod shoal;
mod vector;

use grid::Grid;
use shoal::Shoal;
use vector::Vector;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

const SCHOOL_FISH_COUNT: usize = 300;
const PREDATOR_COUNT: usize = 0;

const REPULSION_DISTANCE: f64 = 50.0;
const ATTRACTION_DISTANCE: f64 = 100.0;
const VISIBILITY_DISTANCE: f64 = 200.0;

const REPULSION_FORCE_FACTOR: f64 = 1e4;
const ATTRACTION_FORCE_FACTOR: f64 = 1e1;

const BACKGROUND_COLOR: u32 = 0x000000;
const GRID_COLOR: u32 = 0x777777;
const GRID_DOT_SPACING: usize = 8;

fn main() {
    let mut grid = Grid::new();

    let mut shoal = Shoal::new();

    while grid.is_open() {
        grid.clear();

        shoal.swim(grid.elapsed_secs());

        grid.center_screen(&shoal.fishes);

        grid.draw_grid();

        shoal.draw(&mut grid);

        grid.print_buffer();
    }
}
