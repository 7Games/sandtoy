pub enum CellType {
    Sand = 0,
    Stone,
    AntiSand,
    Water,
}

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub cell_type: CellType,
}

impl Cell {
    pub fn new(x: i32, y: i32, cell_type: CellType) -> Self {
        Self {
            x,
            y,
            cell_type,
        }
    }
}
