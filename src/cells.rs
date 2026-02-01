use crate::cell;

pub struct Cells {
    data: Vec<cell::Cell>,
}

impl Cells {
    pub fn new() -> Self {
        Self {
            data: vec![],
        }
    }

    pub fn add_cell(&mut self, x: i32, y: i32, cell_type: cell::CellType) -> bool {
        if !self.cell_exists(x, y) {
            self.data.push(cell::Cell::new(x, y, cell_type));
            return true;
        }
        return false;
    }

    fn cell_exists(&self, x: i32, y: i32) -> bool {
        self.data.iter().find(|cell| cell.x == x && cell.y == y).is_some()
    }

    fn cell_below(&self, cell: &cell::Cell) -> bool {
        self.data.iter().find(|neighbour| (neighbour.x == cell.x && neighbour.y == cell.y + 1)).is_some()
    }

    fn cell_below_left(&self, cell: &cell::Cell) -> bool {
        self.data.iter().find(|neighbour| (neighbour.x == cell.x - 1 && neighbour.y == cell.y + 1)).is_some()
    }

    fn cell_below_right(&self, cell: &cell::Cell) -> bool {
        self.data.iter().find(|neighbour| (neighbour.x == cell.x + 1 && neighbour.y == cell.y + 1)).is_some()
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn update(&mut self) {
        for i in 0..self.data.len() {
            match self.data[i].cell_type {
                cell::CellType::Stone => continue,
                cell::CellType::Sand => {
                    if !self.cell_below(&self.data[i]) {
                        self.data[i].y += 1;
                    } else if !self.cell_below_left(&self.data[i]) {
                        self.data[i].y += 1;
                        self.data[i].x -= 1;
                    } else if !self.cell_below_right(&self.data[i]) {
                        self.data[i].y += 1;
                        self.data[i].x += 1;
                    }
                }
            }
        }

        // self.data.retain_mut(|cell| {
        //     !(cell.y as usize >= WINDOW_HEIGHT / CELL_SIZE as usize)
        // });
    }

    pub fn for_each_mut(&self, mut call: impl FnMut(&cell::Cell)) {
        self.data.iter().for_each(|c| (call)(c));
    }
}
