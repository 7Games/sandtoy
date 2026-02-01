use imgui::{Condition, Context, FontSource};
use raylib::prelude::*;
use raylib_imgui_rs::Renderer;

const CELL_SIZE: i32     = 8;
const WINDOW_WIDTH: i32  = 1280;
const WINDOW_HEIGHT: i32 = 720;

enum CellType {
    Sand = 0, // dynamic
    Stone,    // static
}

struct Cell {
    x: i32,
    y: i32,
    cell_type: CellType,
}

//TODO: Add more cell types
impl Cell {
    fn new(x: i32, y: i32, cell_type: CellType) -> Self {
        Self {
            x,
            y,
            cell_type,
        }
    }

    // TODO: Make raylib agnostic
    fn draw(&self, d: &mut RaylibDrawHandle) {
        let color: Color = match self.cell_type {
            CellType::Sand  => Color::YELLOW,
            CellType::Stone => Color::GRAY,
        };

        d.draw_rectangle(self.x * CELL_SIZE,
                         self.y * CELL_SIZE,
                         CELL_SIZE,
                         CELL_SIZE,
                         color);
    }
}

struct Cells {
    data: Vec<Cell>,
}

impl Cells {
    fn new() -> Self {
        Self {
            data: vec![],
        }
    }

    pub fn add_cell(&mut self, x: i32, y: i32, cell_type: CellType) -> bool {
        if !self.cell_exists(x, y) {
            self.data.push(Cell::new(x, y, cell_type));
            return true;
        }
        return false;
    }

    fn cell_exists(&self, x: i32, y: i32) -> bool {
        self.data.iter().find(|cell| cell.x == x && cell.y == y).is_some()
    }

    fn cell_below(&self, cell: &Cell) -> bool {
        self.data.iter().find(|neighbour| (neighbour.x == cell.x && neighbour.y == cell.y + 1)).is_some()
    }

    fn cell_below_left(&self, cell: &Cell) -> bool {
        self.data.iter().find(|neighbour| (neighbour.x == cell.x - 1 && neighbour.y == cell.y + 1)).is_some()
    }

    fn cell_below_right(&self, cell: &Cell) -> bool {
        self.data.iter().find(|neighbour| (neighbour.x == cell.x + 1 && neighbour.y == cell.y + 1)).is_some()
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn update(&mut self) {
        for i in 0..self.data.len() {
            match self.data[i].cell_type {
                CellType::Stone => continue,
                CellType::Sand => {
                    // TODO: Temp
                    if self.data[i].y >= WINDOW_HEIGHT / CELL_SIZE - 1 {
                        continue;
                    }

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
    }

    // TODO: Make raylib agnostic
    pub fn draw_cells(&self, d: &mut RaylibDrawHandle) {
        self.data.iter().for_each(|cell| cell.draw(d));
    }
}

fn main() {
    // Init raylib
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Sandtoy")
        .build();
    rl.set_target_fps(60);

    // TODO: Clean-up {
    let mut imgui = Context::create();
    imgui.fonts().add_font(&[FontSource::DefaultFontData { config: None }]);
    imgui.set_ini_filename(None);

    let mut renderer = Renderer::create(&mut imgui, &mut rl, &thread);

    let pixels_available = ["Sand", "Stone"];

    let mut update_interval: f32 = 0.03;
    let mut next_update = rl.get_time() + update_interval as f64;
    let mut show_ui: bool = true;
    let mut selected_pixel: i32 = 0;

    let mut cells: Cells = Cells::new();

    let camera: Camera2D = Camera2D {
        offset: Vector2 { x: 0.0, y: 0.0 },
        target: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };
    // }

    while !rl.window_should_close() {
        renderer.update(&mut imgui, &mut rl);
        let ui = imgui.new_frame();

        let current_time: f64 = rl.get_time();
        let mouse_pos = rl.get_screen_to_world2D(rl.get_mouse_position(), camera);
        let mouse_x: i32 = (mouse_pos.x as i32) / CELL_SIZE;
        let mouse_y: i32 = (mouse_pos.y as i32) / CELL_SIZE;

        if current_time >= next_update {
            cells.update();
            next_update = rl.get_time() + update_interval as f64;
        }

        // Make debug UI using ImGUI
        if show_ui {
            // TODO: Add cells selector
            ui.window("Sandtoy")
                .size([(WINDOW_WIDTH as f32) / 4.0, WINDOW_HEIGHT as f32], Condition::FirstUseEver)
                .position([(WINDOW_WIDTH as f32) - ((WINDOW_WIDTH as f32) / 4.0), 0.0], Condition::FirstUseEver)
                .resizable(false)
                .movable(false)
                .collapsible(false)
                .build(|| {
                    let fps = rl.get_fps();
                    ui.text_colored([0.7, 0.7, 0.7, 1.0], "== Info ==");
                    ui.text(format!("FPS: {}", fps));
                    ui.text(format!("Cells: {}", cells.count()));
                    ui.slider("Interval", 0.001, 0.15, &mut update_interval);
                    ui.separator();
                    ui.text_colored([0.7, 0.7, 0.7, 1.0], "== Available Cells ==");
                    ui.list_box("Cells", &mut selected_pixel, &pixels_available, 2);
                    ui.separator();
                    ui.text_colored([0.7, 0.7, 0.7, 1.0], "== Controls ==");
                    ui.text("- TAB: Toggle UI");
                    ui.text("- LEFT MOUSE: Place cell");
                });
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) &&
            !cells.cell_exists(mouse_x, mouse_y) &&
            !(show_ui && (mouse_pos.x >= (WINDOW_WIDTH as f32) - ((WINDOW_WIDTH as f32) / 4.0))) {
                match selected_pixel {
                    0 => cells.add_cell(mouse_x, mouse_y, CellType::Sand),
                    1 => cells.add_cell(mouse_x, mouse_y, CellType::Stone),
                    _ => panic!("What pixel?"),
                };
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

                cells.draw_cells(&mut d);
            }

            // Render
            renderer.render(&mut imgui, &mut d);
        }
    }
}
