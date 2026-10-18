use crate::cell;

pub struct Cells {
    data: Vec<cell::Cell>,
}

impl Cells {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn add_cell(&mut self, x: i32, y: i32, cell_type: cell::CellType) -> bool {
        if !self.cell_exists(x, y) {
            self.data.push(cell::Cell::new(x, y, cell_type));
            return true;
        }
        return false;
    }

    pub fn remove_cell(&mut self, x: i32, y: i32) {
        self.data.retain_mut(|cell| !(cell.x == x && cell.y == y));
    }

    fn cell_exists(&self, x: i32, y: i32) -> bool {
        self.data
            .iter()
            .find(|cell| cell.x == x && cell.y == y)
            .is_some()
    }

    pub fn cleanse(&mut self, top: i32, bottom: i32, left: i32, right: i32) {
        self.data
            .retain_mut(|cell| cell.y > top && cell.y < bottom && cell.x > left && cell.y < right);
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn update(&mut self) {
        for i in 0..self.data.len() {
            match self.data[i].cell_type {
                cell::CellType::Stone => continue,
                cell::CellType::Sand => {
                    if !self.cell_exists(self.data[i].x, self.data[i].y + 1) {
                        self.data[i].y += 1;
                    } else if !self.cell_exists(self.data[i].x - 1, self.data[i].y + 1)
                        && !self.cell_exists(self.data[i].x - 1, self.data[i].y)
                    {
                        self.data[i].y += 1;
                        self.data[i].x -= 1;
                    } else if !self.cell_exists(self.data[i].x + 1, self.data[i].y + 1)
                        && !self.cell_exists(self.data[i].x + 1, self.data[i].y)
                    {
                        self.data[i].y += 1;
                        self.data[i].x += 1;
                    }
                }
                cell::CellType::AntiSand => {
                    if !self.cell_exists(self.data[i].x, self.data[i].y - 1) {
                        self.data[i].y -= 1;
                    } else if !self.cell_exists(self.data[i].x - 1, self.data[i].y - 1)
                        && !self.cell_exists(self.data[i].x - 1, self.data[i].y)
                    {
                        self.data[i].y -= 1;
                        self.data[i].x -= 1;
                    } else if !self.cell_exists(self.data[i].x + 1, self.data[i].y - 1)
                        && !self.cell_exists(self.data[i].x + 1, self.data[i].y)
                    {
                        self.data[i].y -= 1;
                        self.data[i].x += 1;
                    }
                },
                cell::CellType::Water => {
                    if !self.cell_exists(self.data[i].x, self.data[i].y + 1) {
                        self.data[i].y += 1;
                    } else if !self.cell_exists(self.data[i].x + 1, self.data[i].y + 1)
                        && !self.cell_exists(self.data[i].x + 1, self.data[i].y)
                    {
                        self.data[i].y += 1;
                        self.data[i].x += 1;
                    } else if !self.cell_exists(self.data[i].x - 1, self.data[i].y + 1)
                        && !self.cell_exists(self.data[i].x - 1, self.data[i].y)
                    {
                        self.data[i].y += 1;
                        self.data[i].x -= 1;
                    } else {
                        if !self.cell_exists(self.data[i].x + 1, self.data[i].y) {
                            self.data[i].x += 1;
                        } else if !self.cell_exists(self.data[i].x - 1, self.data[i].y) {
                            self.data[i].x -= 1;
                        }
                    }
                },
            }
        }
    }

    pub fn for_each_mut(&self, mut call: impl FnMut(&cell::Cell)) {
        self.data.iter().for_each(|c| (call)(c));
    }
}
