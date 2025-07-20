use crate::voxel::Cell as CanonCell;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum CollisionFlag {
    None = 0,
    Floor = 1,
    SpaceTimeFus = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Cell {
    Air =           0,
    Floor =         1,
    SpaceTimeFus =  2,
    Void =          255,
}

impl From<Cell> for u32 {
    fn from(cell: Cell) -> u32 {
        cell as u32
    }
}

impl From<Cell> for CanonCell {
    fn from(cell: Cell) -> CanonCell {
        let mut canon_cell = CanonCell::EMPTY;
        canon_cell.set_cell_id(cell as u32);
        match cell {
            Cell::Air => {
                canon_cell.set_empty(1);
                canon_cell.set_opaque(0);
                canon_cell.set_collision_flag(CollisionFlag::None as u32);
            },
            Cell::Floor => {
                canon_cell.set_empty(0);
                canon_cell.set_opaque(1);
                canon_cell.set_collision_flag(CollisionFlag::Floor as u32);
            },
            Cell::SpaceTimeFus => {
                canon_cell.set_empty(0);
                canon_cell.set_opaque(0);
                canon_cell.set_collision_flag(CollisionFlag::SpaceTimeFus as u32);
            },
            Cell::Void => {
                canon_cell.set_empty(1);
                canon_cell.set_opaque(0);
                canon_cell.set_collision_flag(CollisionFlag::None as u32);
            },
        };
        canon_cell
    }
}
