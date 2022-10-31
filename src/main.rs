mod grid;

use egui::text::LayoutJob;
use egui::FontId;
use egui::Frame;
use egui::RichText;
use egui::TextFormat;
use grid::Grid;
use pathfinding::astar;
use pathfinding::manhattan_distance;

use eframe;
use eframe::egui;

fn get_grid_pos(pos: egui::Pos2, grid_size: f32, offset: (f32, f32)) -> (usize, usize) {
    let (x, y) = (pos.x - offset.0, pos.y - offset.1);
    let (r, c) = (y / grid_size, x / grid_size);
    (r as usize, c as usize)
}

#[derive(PartialEq)]
enum PaintTile {
    Nothing,
    ObstaclePlacement,
    Start,
    End,
}

struct MyApp {
    grid: Grid<i32>,
    start: (i32, i32),
    end: (i32, i32),
    path: Option<Vec<(i32, i32)>>,
    paint_mode: PaintTile,
    highlited: Option<(usize, usize)>,
}

impl Default for MyApp {
    fn default() -> Self {
        let grid = Grid::from_vec(vec![vec![0; 10]; 10]);
        let start = (1, 1);
        let end = (8, 8);
        let path = None;
        MyApp {
            grid,
            start,
            end,
            path,
            paint_mode: PaintTile::Nothing,
            highlited: None,
        }
    }
}

impl MyApp {
    fn find_path(&mut self) {
        self.path = astar(
            self.start,
            self.end,
            &self.grid.to_vec(),
            manhattan_distance,
            |row, col, grid| grid[row][col] == 1,
        );
    }

    fn ui_control(&mut self, ui: &mut egui::Ui) {
        ui.heading("A* algorithm visualisation");
        ui.horizontal(|ui| {
            ui.label("Start:");
            ui.colored_label(egui::Color32::GRAY, "row");
            ui.add(egui::DragValue::new(&mut self.start.1).speed(1.0));
            ui.colored_label(egui::Color32::GRAY, "col");
            ui.add(egui::DragValue::new(&mut self.start.0).speed(1.0));
        });
        ui.horizontal(|ui| {
            ui.label("End:");
            ui.colored_label(egui::Color32::GRAY, "row");
            ui.add(egui::DragValue::new(&mut self.end.1).speed(1.0));
            ui.colored_label(egui::Color32::GRAY, "col");
            ui.add(egui::DragValue::new(&mut self.end.0).speed(1.0));
        });
        ui.horizontal(|ui| {
            if ui.button("Find path").clicked() {
                self.find_path();
            }
            ui.colored_label(egui::Color32::TRANSPARENT, " ");
            if ui.button("Clear path").clicked() {
                self.path = None;
            }
        });
    }

    fn ui_grid_canvas(&mut self, offset: (f32, f32), ui: &mut egui::Ui) {
        let (offset_x, offset_y) = offset;
        let grid_size = 20;
        let painter = ui.painter();
        for row in 0..self.grid.height {
            for col in 0..self.grid.width {
                let mut color = egui::Color32::from_rgb(255, 255, 255);
                if self.grid[row][col] == 1 {
                    color = egui::Color32::from_rgb(0, 0, 0);
                }
                if let Some(path) = &self.path {
                    if path.contains(&(row as i32, col as i32)) {
                        color = egui::Color32::from_rgb(0, 0, 255);
                    }
                }
                if self.start == (row as i32, col as i32) {
                    color = egui::Color32::from_rgb(0, 255, 0);
                }
                if self.end == (row as i32, col as i32) {
                    color = egui::Color32::from_rgb(255, 0, 0);
                }
                if let Some(highlited) = self.highlited {
                    if highlited == (row, col) {
                        let tmp = color.to_array();
                        color = egui::Color32::from_rgb(
                            (tmp[0] as f32 * 0.5) as u8,
                            (tmp[1] as f32 * 0.5) as u8,
                            (tmp[2] as f32 * 0.5) as u8,
                        );
                    }
                }
                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::Pos2::new(
                            col as f32 * grid_size as f32 + offset_x,
                            row as f32 * grid_size as f32 + offset_y,
                        ),
                        egui::Vec2::new(grid_size as f32, grid_size as f32),
                    ),
                    0.0,
                    color,
                );
            }
        }
    }

    fn handle_canvas_response(
        &mut self,
        response: egui::Response,
        ui: &mut egui::Ui,
        offset: (f32, f32),
    ) {
        if response.clicked() || response.dragged() || response.double_clicked() {
            let mouse_pos = ui.input().pointer.interact_pos();
            match mouse_pos {
                Some(pos) => {
                    let grid_size = 20.;
                    let (row, col) = get_grid_pos(pos, grid_size, offset);
                    if row < self.grid.height && col < self.grid.width {
                        match self.paint_mode {
                            PaintTile::Start => self.start = (row as i32, col as i32),
                            PaintTile::End => self.end = (row as i32, col as i32),
                            PaintTile::ObstaclePlacement => {
                                self.grid[row][col] = if self.grid[row][col] == 0 { 1 } else { 0 }
                            }
                            PaintTile::Nothing => {}
                        }
                        self.find_path();
                    }
                }
                None => {}
            }
        }

        if response.hovered() {
            let pos = response.hover_pos();
            if pos.is_some() {
                self.highlited = Some(get_grid_pos(pos.unwrap(), 20., offset));
            } else {
                self.highlited = None;
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_control(ui);
            ui.separator();
            ui.horizontal(|ui| {
                if ui
                    .button(RichText::new("Start").color(egui::Color32::GREEN))
                    .clicked()
                {
                    self.paint_mode = PaintTile::Start;
                }
                if ui
                    .button(RichText::new("End").color(egui::Color32::RED))
                    .clicked()
                {
                    self.paint_mode = PaintTile::End;
                }
                // make string "Obstacle/Empty" where Obstacle is Black, Empty is White
                let mut text = LayoutJob::default();
                text.append(
                    "Obstacle",
                    0.0,
                    TextFormat {
                        font_id: FontId::new(14., egui::FontFamily::Proportional),
                        color: egui::Color32::BLACK,
                        ..Default::default()
                    },
                );
                text.append(
                    "/",
                    0.0,
                    TextFormat {
                        font_id: FontId::new(14., egui::FontFamily::Proportional),
                        color: egui::Color32::GRAY,
                        ..Default::default()
                    },
                );
                text.append(
                    "Empty",
                    0.0,
                    TextFormat {
                        font_id: FontId::new(14., egui::FontFamily::Proportional),
                        color: egui::Color32::WHITE,
                        ..Default::default()
                    },
                );

                if ui.button(text).clicked() {
                    self.paint_mode = PaintTile::ObstaclePlacement;
                }
            });
            ui.horizontal(|ui| {
                let path_state = if let Some(_) = self.path {
                    RichText::new("SUCCESS").underline()
                } else {
                    RichText::new("FAIL").underline()
                };
                ui.label(path_state);
                if ui.button("Clear grid").clicked() {
                    self.grid.fill(0);
                    self.find_path()
                }
            });
            let mut offset = (0.0, 0.0);
            let canvas = Frame::canvas(ui.style())
                .show(ui, |ui| {
                    let (_, rect) = ui.allocate_space(ui.available_size());
                    let margin = 10.0;
                    offset = (rect.min.x + margin, rect.min.y + margin);
                    self.ui_grid_canvas(offset, ui);
                })
                .response;
            let response = canvas.interact(egui::Sense::click());
            self.handle_canvas_response(response, ui, offset);
        });
    }
}

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::Vec2::new(240.0, 370.0));
    options.resizable = false;
    options.always_on_top = false;
    let mut app = MyApp::default();
    app.find_path();
    eframe::run_native(
        "A* algorithm visualisation",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}
