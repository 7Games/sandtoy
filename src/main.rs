use imgui::{Context, FontSource, Condition};
use raylib::prelude::*;
use raylib_imgui_rs::Renderer;

const CELL_SIZE: i32     = 8;
const WINDOW_WIDTH: i32  = 640;
const WINDOW_HEIGHT: i32 = 480;

#[derive(PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Debug)]
enum CellType {
    Sand = 0,
    // Water,
}

#[derive(Debug)]
struct Cell {
    point: Point,
    r#type: CellType,
    grounded: bool,
}

impl Cell {
    fn new(x: i32, y: i32, r#type: CellType) -> Self {
        Self {
            point: Point::new(x, y),
            r#type,
            grounded: false,
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        let color: Color = match self.r#type {
            CellType::Sand  => Color::YELLOW,
            // CellType::Water => Color::BLUE,
        };

        d.draw_rectangle(self.point.x * CELL_SIZE,
                         self.point.y * CELL_SIZE,
                         CELL_SIZE,
                         CELL_SIZE,
                         color);
    }
}

fn cell_exists(cells: &Vec<Cell>, point: Point) -> bool {
    cells.iter().find(|cell| cell.point.x == point.x && cell.point.y == point.y).is_some()
}

fn get_neighbours(cells: &Vec<Cell>, point: Point) -> Vec<&Cell> {
    cells.iter().filter(|cell| !(cell.point.x == point.x && cell.point.y == point.y) &&
                        ((cell.point.x >= point.x - 1 && cell.point.x <= point.x + 1) &&
                         (cell.point.y >= point.y - 1 && cell.point.y <= point.y + 1))).collect()
}

fn cell_below(neighbours: &Vec<&Cell>, cell: &Cell) -> bool {
    neighbours.iter().find(|c|
                           c.point.x == cell.point.x &&
                           c.point.y == cell.point.y + 1).is_some()
}

fn cell_below_left(neighbours: &Vec<&Cell>, cell: &Cell) -> bool {
    neighbours.iter().find(|c|
                           c.point.x == cell.point.x - 1 &&
                           c.point.y == cell.point.y + 1).is_some()
}

fn cell_below_right(neighbours: &Vec<&Cell>, cell: &Cell) -> bool {
    neighbours.iter().find(|c|
                           c.point.x == cell.point.x + 1 &&
                           c.point.y == cell.point.y + 1).is_some()
}

fn update_cells(cells: &mut Vec<Cell>) {
    for i in 0..cells.len() {
        if cells[i].point.y >= WINDOW_HEIGHT / CELL_SIZE - 1 {
            cells[i].grounded = true;
            continue;
        } else {
            cells[i].grounded = false;
        }

        let cells_around = get_neighbours(cells, cells[i].point.clone());

        if !cell_below(&cells_around, &cells[i]) {
            cells[i].point.y += 1;
        } else if !cell_below_left(&cells_around, &cells[i]) {
                cells[i].point.y += 1;
                cells[i].point.x -= 1;
        } else if !cell_below_right(&cells_around, &cells[i]) {
            cells[i].point.y += 1;
            cells[i].point.x += 1;
        }
    }
}

fn main() {
    // Init raylib
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Sandtoy")
        .build();
    rl.set_target_fps(60);

    let mut imgui = Context::create();
    imgui.fonts().add_font(&[FontSource::DefaultFontData { config: None }]);
    imgui.set_ini_filename(None);

    let mut renderer = Renderer::create(&mut imgui, &mut rl, &thread);

    let mut update_interval: f32 = 0.05;
    let mut next_update = rl.get_time() + update_interval as f64;
    let mut show_ui: bool = true;

    let mut cells: Vec<Cell> = vec![];

    let camera: Camera2D = Camera2D {
        offset: Vector2 { x: 0.0, y: 0.0 },
        target: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };

    while !rl.window_should_close() {
        renderer.update(&mut imgui, &mut rl);
        let ui = imgui.new_frame();

        let current_time: f64 = rl.get_time();
        let mouse_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), camera);
        let mouse_x: i32 = (mouse_pos.x as i32) / CELL_SIZE;
        let mouse_y: i32 = (mouse_pos.y as i32) / CELL_SIZE;

        if current_time >= next_update {
            update_cells(&mut cells);
            next_update = rl.get_time() + update_interval as f64;
        }

        // Make debug UI using ImGUI
        if show_ui {
            ui.window("Starview debug window")
                .size([300.0, 120.0], Condition::FirstUseEver)
                .position([640.0-300.0-10.0, 480.0-120.0-10.0], Condition::FirstUseEver)
                .resizable(false)
                .movable(false)
                .collapsible(false)
                .build(|| {
                    let fps = rl.get_fps();
                    ui.text(format!("FPS: {}", fps));
                    ui.text(format!("Cells: {}", cells.len()));
                    ui.slider("Interval: ", 0.001, 0.5, &mut update_interval);
                });
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) && !cell_exists(&cells, Point::new(mouse_x, mouse_y)) {
            cells.push(Cell::new(mouse_x, mouse_y, CellType::Sand));
        }

        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            show_ui = !show_ui;
        }

        // Render
        {
            let mut d = rl.begin_drawing(&thread);

            {
                let mut d = d.begin_mode2D(camera);

                d.clear_background(Color::BLACK);

                // Draw cells
                cells.iter().for_each(|cell| {
                    cell.draw(&mut d);
                });
            }

            // Render
            renderer.render(&mut imgui, &mut d);
        }
    }

}
