pub mod algorithms;
pub mod cells;

use bitfield::bitfield;
use nalgebra_glm::TVec3;
use avalon::texture::GpuTexture3d;
use anyhow::Error;

pub use cells::Cell as CellType;

bitfield!{
    pub struct Cell(u32);
    impl Debug;
    pub collision_flag, set_collision_flag: 4, 0;
    pub empty, set_empty: 5, 5;
    pub opaque, set_opaque: 6, 6;
    safe_step, set_safe_step: 15, 7; // only usable if `is_empty == 1`
    pub cell_id, set_cell_id: 16, 7; // only usable if `is_empty == 0`
}

impl Copy for Cell {}
impl Clone for Cell {
    fn clone(&self) -> Self {
        *self
    }
}

impl Cell {
    #[allow(clippy::unusual_byte_groupings)]
    const EMPTY: Cell = Cell(
        0b000_0000_0000_0000_0000000000_0_1_00000
    );

    pub fn is_empty(&self) -> bool {
        self.empty() == 1
    }
}

pub struct Grid<const SIDE_LENGTH: usize = 32, const VOXELS_PER_METER: u32 = 1> where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:,
    {
    cells: Vec<Cell>,
    dirty: bool,
    gpu_grid: Option<GpuTexture3d>
}

impl<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32> Grid<SIDE_LENGTH, VOXELS_PER_METER> where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
    pub fn new() -> Grid<SIDE_LENGTH, VOXELS_PER_METER> {
        Grid {
            cells: vec![Cell::EMPTY; SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH],
            dirty: false,
            gpu_grid: None
        }
    }

    pub fn vec_to_index(&self, position: TVec3<u8>) -> usize {
        let position: TVec3<usize> = position.cast();
        position.x + SIDE_LENGTH * position.y + SIDE_LENGTH * SIDE_LENGTH * position.z
    }

    pub fn index_to_vec(&self, idx: usize) -> TVec3<u8> {
        let mut position: TVec3<u8> = TVec3::zeros();

        position.x = (idx % SIDE_LENGTH) as u8;
        position.y = ((idx / SIDE_LENGTH) % SIDE_LENGTH) as u8;
        position.z = (idx / (SIDE_LENGTH * SIDE_LENGTH)) as u8;

        position
    }

    pub fn cell_at_position(&self, position: TVec3<f32>) -> Option<&Cell> {
        let position = position * VOXELS_PER_METER as f32;
        let position = TVec3::new(
            position.x.floor() as u8,
            position.y.floor() as u8,
            position.z.floor() as u8
        );

        self.cell(position.cast())
    }

    pub fn cell(&self, position: impl Into<TVec3<i16>>) -> Option<&Cell> {
        let position = position.into();
        if position.iter().any(|v| (*v) as usize >= SIDE_LENGTH) {
            return None
        }
        if position.iter().any(|v| *v < 0) {
            return None
        }

        Some(&self.cells[self.vec_to_index(position.map(|c| c as u8))])
    }

    pub fn cell_mut(&mut self, position: TVec3<u8>) -> &mut Cell {
        if position.iter().any(|v| (*v) as usize >= SIDE_LENGTH) {
            panic!("Attempting to index grid out of bounds!");
        }

        // assume if we get a mutable reference that we will be dirty
        self.dirty = true;
        let idx = self.vec_to_index(position);
        &mut self.cells[idx]
    }
}

impl<const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>
    TryFrom<Grid<SIDE_LENGTH, VOXELS_PER_METER>> for GpuTexture3d where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
    type Error = String;
    fn try_from(value: Grid<SIDE_LENGTH, VOXELS_PER_METER>) -> Result<Self, Self::Error> {
        value.gpu_grid.ok_or("grid needs to be baked before use".to_string())
    }
}

impl<'t, 'g: 't, const SIDE_LENGTH: usize, const VOXELS_PER_METER: u32>
    TryFrom<&'g Grid<SIDE_LENGTH, VOXELS_PER_METER>> for &'t GpuTexture3d where
    [(); SIDE_LENGTH * SIDE_LENGTH * SIDE_LENGTH]:, {
    type Error = String;
    fn try_from(value: &'g Grid<SIDE_LENGTH, VOXELS_PER_METER>) -> Result<Self, Self::Error> {
        value.gpu_grid.as_ref().ok_or("grid needs to be baked before use".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra_glm::vec3;

    #[test]
    fn test_cell_idx_conversion_is_homo() {
        let grid: Grid<4, 1> = Grid::new();
        let idx0 = grid.vec_to_index(vec3(0, 0, 0));
        let idx1 = grid.vec_to_index(vec3(3, 0, 0));
        let idx2 = grid.vec_to_index(vec3(0, 3, 0));
        let idx3 = grid.vec_to_index(vec3(0, 0, 3));
        let idx4 = grid.vec_to_index(vec3(3, 3, 3));

        assert_eq!(idx0, 0);
        assert_eq!(idx1, 3);
        assert_eq!(idx2, 12);
        assert_eq!(idx3, 48);
        assert_eq!(idx4, 63);

        let p0 = grid.index_to_vec(idx0);
        let p1 = grid.index_to_vec(idx1);
        let p2 = grid.index_to_vec(idx2);
        let p3 = grid.index_to_vec(idx3);
        let p4 = grid.index_to_vec(idx4);

        assert_eq!(p0, vec3(0, 0, 0));
        assert_eq!(p1, vec3(3, 0, 0));
        assert_eq!(p2, vec3(0, 3, 0));
        assert_eq!(p3, vec3(0, 0, 3));
        assert_eq!(p4, vec3(3, 3, 3));
    }
}
