use crate::voxel::Cell as CanonCell;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Cell {
    Air =           0,
    Floor =         1,
    SpaceTimeFus =  2,
}

impl From<Cell> for CanonCell {
    fn from(cell: Cell) -> CanonCell {
        let mut canon_cell = CanonCell::EMPTY;
        canon_cell.set_cell_id(cell as u32);
        match cell {
            Cell::Air => {
                canon_cell.set_empty(1);
                canon_cell.set_opaque(0);
                canon_cell.set_collision_flag(0);
            },
            Cell::Floor => {
                canon_cell.set_empty(0);
                canon_cell.set_opaque(1);
                canon_cell.set_collision_flag(1);
            },
            Cell::SpaceTimeFus => {
                canon_cell.set_empty(0);
                canon_cell.set_opaque(0);
                canon_cell.set_collision_flag(2);
            },
        };
        canon_cell
    }
}
