#![feature(generic_const_exprs)]
#![allow(incomplete_features, unused)]

use nalgebra_glm::vec3;

pub mod voxel;

use avalon;

fn main() {
    let mut engine = avalon::engine();

    let mut grid: voxel::Grid<8, 1> = voxel::Grid::new();
    grid.cell_mut(vec3(0, 0, 0)).set_empty(0);
    grid.calculate_distance_field();

    while engine.is_open() {
        engine.start_frame();
        engine.poll_events();
        engine.render();
        engine.end_frame();
    }
}
