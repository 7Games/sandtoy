use raylib::prelude::{KeyboardKey::*, MouseButton::*};

mod cell;
mod cells;
mod engine;
mod status;

use engine::*;
use cells::*;

const CELL_SIZE: i32     = 8;
const WINDOW_WIDTH: i32  = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn main() {
    let mut window: Window = Window::create(WINDOW_WIDTH, WINDOW_HEIGHT, CELL_SIZE, "Sandtoy");

    // ImGUI stuff
    let mut show_ui: bool = true;
    let mut update_interval: f64 = 0.023;
    let pixels_available = ["Sand", "Stone"];
    let mut selected_pixel: i32 = 0;
    let mut selected_tool:  i32 = 0;
    let mut about_open: bool = false;

    let mut cells: Cells = Cells::new();

    let mut next_update = window.get_time() + update_interval;

    while window.open() {
        window.update();

        let mouse_x: i32 = window.get_world_mouse_x() as i32 / CELL_SIZE;
        let mouse_y: i32 = window.get_world_mouse_y() as i32 / CELL_SIZE;

        // Inputs
        if window.key_pressed(KEY_TAB) {
            show_ui = !show_ui;
        };

        if window.mouse_down(MOUSE_BUTTON_LEFT) {
            if selected_tool == 0 {
                match selected_pixel {
                    0 => cells.add_cell(mouse_x, mouse_y, cell::CellType::Sand),
                    1 => cells.add_cell(mouse_x, mouse_y, cell::CellType::Stone),
                    _ => panic!("What pixel?"),
                };
            } else if selected_tool == 1 {
                cells.remove_cell(mouse_x, mouse_y);
            }
        }

        // if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) &&
        //     !cells.cell_exists(mouse_x, mouse_y) &&
        //     !(show_ui && (mouse_pos.x >= (WINDOW_WIDTH as f64) - ((WINDOW_WIDTH as f32) / 4.0))) {
        //         match selected_pixel {
        //             0 => cells.add_cell(mouse_x, mouse_y, cell::CellType::Sand),
        //             1 => cells.add_cell(mouse_x, mouse_y, cell::CellType::Stone),
        //             _ => panic!("What pixel?"),
        //         };
        //     }

        // Update cells
        if window.get_time() >= next_update {
            cells.update();
            next_update = window.get_time() + update_interval;
        }

        // Render
        if show_ui {
            window.draw_ui(&cells,
                           &mut update_interval,
                           &mut selected_tool,
                           &mut selected_pixel,
                           &pixels_available,
                           &mut about_open);
        }

        window.draw(&cells, mouse_x * CELL_SIZE, mouse_y * CELL_SIZE);

        cells.cleanse(-1,
                      WINDOW_HEIGHT / CELL_SIZE,
                      -1,
                      WINDOW_WIDTH / CELL_SIZE);
    }
}
