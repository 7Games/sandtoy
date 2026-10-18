use std::process::exit;
use raylib::prelude::*;

use crate::cell;
use crate::cells;
use crate::status;

pub struct Window {
    pub width: i32,
    pub height: i32,
    pub cell_size: i32,

    raylib: RaylibHandle,
    raylib_thread: RaylibThread,

    camera: Camera2D,

    imgui_context: imgui::Context,

    renderer: raylib_imgui_rs::Renderer, // Raylib && ImGUI

    status: status::Status,
}

impl Window {
    pub fn create(width: i32, height: i32, cell_size: i32, title: &str) -> Self {
        // Create Raylib window
        let (mut raylib, raylib_thread) = raylib::init()
            .size(width, height)
            .title(title)
            .resizable()
            .build();

        // Create ImGUI context
        let mut imgui_context = imgui::Context::create();
        imgui_context.fonts().add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
        imgui_context.set_ini_filename(None);

        // Create renderer for Raylib and ImGUI
        let renderer: raylib_imgui_rs::Renderer = raylib_imgui_rs::Renderer::create(&mut imgui_context,
                                                                                    &mut raylib,
                                                                                    &raylib_thread);

        let camera: Camera2D = Camera2D {
            offset: Vector2 { x: 0.0, y: 0.0 },
            target: Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            zoom: 1.0,
        };

        Self {
            width,
            height,
            cell_size,
            raylib,
            raylib_thread,
            camera,
            imgui_context,
            renderer,
            status: status::Status::new(),
        }
    }

    pub fn open(&self) -> bool {
        !self.raylib.window_should_close()
    }

    pub fn update(&mut self) {
        self.renderer.update(&mut self.imgui_context, &mut self.raylib);

        // Update size
        self.width = self.raylib.get_render_width();
        self.height = self.raylib.get_render_height();
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }
    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn key_pressed(&self, key: KeyboardKey) -> bool {
        self.raylib.is_key_pressed(key)
    }

    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.raylib.is_mouse_button_down(button)
    }

    pub fn get_time(&self) -> f64 {
        self.raylib.get_time()
    }

    fn get_window_mouse_position(&self) -> Vector2 {
        self.raylib.get_screen_to_world2D(self.raylib.get_mouse_position(), self.camera)
    }

    pub fn get_world_mouse_x(&self) -> f32 {
        self.get_window_mouse_position().x
    }

    pub fn get_world_mouse_y(&self) -> f32 {
        self.get_window_mouse_position().y
    }

    pub fn draw(&mut self, cells: &cells::Cells, mouse_x: i32, mouse_y: i32) {
        let current_time: f64 = self.raylib.get_time();
        let mut d = self.raylib.begin_drawing(&self.raylib_thread);

        // World
        {
            let mut d = d.begin_mode2D(self.camera);

            d.clear_background(Color::BLACK);

            // Cells
            cells.for_each_mut(|cell| {
                let color: Color = match cell.cell_type {
                    cell::CellType::Sand     => Color::YELLOW,
                    cell::CellType::Stone    => Color::GRAY,
                    cell::CellType::AntiSand => Color::WHITE,
                    cell::CellType::Water    => Color::BLUE,
                };

                d.draw_rectangle(cell.x * self.cell_size,
                                 cell.y * self.cell_size,
                                 self.cell_size,
                                 self.cell_size,
                                 color);
            });

            // Cursor
            d.draw_line(mouse_x,
                        mouse_y,
                        mouse_x + self.cell_size,
                        mouse_y,
                        Color::WHITE);
            d.draw_line(mouse_x,
                        mouse_y,
                        mouse_x,
                        mouse_y + self.cell_size,
                        Color::WHITE);
            d.draw_line(mouse_x + self.cell_size,
                        mouse_y + self.cell_size,
                        mouse_x + self.cell_size,
                        mouse_y,
                        Color::WHITE);
            d.draw_line(mouse_x + self.cell_size,
                        mouse_y + self.cell_size,
                        mouse_x,
                        mouse_y + self.cell_size,
                        Color::WHITE);

            d.draw_pixel(mouse_x + (self.cell_size / 2) - 1,
                         mouse_y + (self.cell_size / 2) - 1,
                         Color::WHITE);
        }

        // Draw status text
        let mut i: i32 = 1;
        let mut remove_message: Vec<usize> = vec![];
        for msg in self.status.get_messages() {
            if current_time >= (msg.time_created + msg.destroy_timer) {
                remove_message.push((i-1) as usize);
                continue;
            }

            let mut color: Color = Color::WHITE;

            match msg.message_type {
                status::StatusMessageType::Info    => {
                },
                status::StatusMessageType::Warning => {
                    color = Color::YELLOW;
                },
                status::StatusMessageType::Error   => {
                    color = Color::RED;
                },
            }

            d.draw_text(&msg.message_text, 10, 32 * i, 32, color);

            i += 1;
        }

        remove_message.iter().for_each(|idx| {
            self.status.remove_at(*idx);
        });

        self.renderer.render(&mut self.imgui_context, &mut d);
    }

    pub fn draw_ui(&mut self, cells: &cells::Cells,
                   mut update_interval: &mut f64,
                   selected_tool: &mut i32,
                   mut selected_pixel: &mut i32,
                   pixels_available: &[&str],
                   about_open: &mut bool) {
        let imgui_frame: &mut imgui::Ui = self.imgui_context.new_frame();
        let current_time: f64 = self.raylib.get_time();

        // Side panel
        imgui_frame.window("Sandtoy")
            .size([(self.width as f32) / 4.0, self.height as f32], imgui::Condition::Always)
            .position([(self.width as f32) - ((self.width as f32) / 4.0), 0.0], imgui::Condition::Always)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .build(|| {
                let fps = self.raylib.get_fps();
                imgui_frame.text_colored([0.7, 0.7, 0.7, 1.0], "== Info ==");
                imgui_frame.text(format!("FPS: {}", fps));
                imgui_frame.text(format!("Cells: {}", cells.count()));
                imgui_frame.slider("Interval", 0.001, 0.15, &mut update_interval);
                imgui_frame.separator();
                imgui_frame.text_colored([0.7, 0.7, 0.7, 1.0], "== Tools ==");

                imgui_frame.radio_button("Pen", selected_tool, 0);
                imgui_frame.same_line();
                imgui_frame.radio_button("Eraser", selected_tool, 1);

                imgui_frame.list_box("Cells", &mut selected_pixel, &pixels_available, 10);

                imgui_frame.separator();
                if imgui_frame.button("Clear all cells") {
                    self.status.add(String::from("Implement me"), status::StatusMessageType::Error, current_time, 5.0);
                }

                imgui_frame.separator();
                imgui_frame.text_colored([0.7, 0.7, 0.7, 1.0], "== Controls ==");
                imgui_frame.text("- TAB: Toggle UI");
                imgui_frame.text("- LEFT MOUSE: Place cell");
            });

        // Menu bar
        {
            let main = imgui_frame.begin_main_menu_bar().unwrap();

            {
                let sandtoy = imgui_frame.begin_menu("Sandtoy");
                if sandtoy.is_some() {
                    if imgui_frame.menu_item("Save") {
                        self.status.add(String::from("IMPLEMENT Save SYSTEM"), status::StatusMessageType::Warning, current_time, 5.0);
                    }
                    if imgui_frame.menu_item("Load") {
                        self.status.add(String::from("IMPLEMENT Load SYSTEM"), status::StatusMessageType::Warning, current_time, 5.0);
                    }
                    if imgui_frame.menu_item("Screenshot") {
                        self.raylib.take_screenshot(&self.raylib_thread, &format!("{}-{}.png", "screenshot", "00"));
                    }
                    imgui_frame.separator();
                    if imgui_frame.menu_item("Exit") {
                        exit(0);
                    }
                    sandtoy.unwrap().end();
                }
            }

            if imgui_frame.menu_item("About") {
                *about_open = !*about_open;
                self.status.add(String::from("About opened"), status::StatusMessageType::Info, current_time, 5.0);
            }

            main.end();
        }

        // About window
        if *about_open {
            imgui_frame.window("About")
                .size([300.0, 100.0], imgui::Condition::FirstUseEver)
                .position([20.0, 50.0], imgui::Condition::FirstUseEver)
                .resizable(false)
                .movable(true)
                .opened(about_open)
                .collapsible(true)
                .build(|| {
                    imgui_frame.text("Sandtoy");
                    imgui_frame.text("Interactive pixel simulation");
                    imgui_frame.text("by svn <benjamin@benjaminblack.me>");
                    imgui_frame.text_colored([0.4, 0.4, 0.8, 1.0], "https://github.com/7Games/sandtoy");
                });
        }
    }
}
