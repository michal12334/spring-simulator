use egui::{Vec2, Widget};

#[derive(Debug)]
pub struct App {
    speed: f64,
}

impl App {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("settings_panel").show(ctx, |ui| {
            if ui.button("run").clicked() {

            }

            if ui.button("reset").clicked() {

            }

            egui::Slider::new(&mut self.speed, 0.1..=10.0).ui(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let cell_size = {
                let size = ui.available_size();
                Vec2 {
                    x: size.x / 2.0,
                    y: size.y / 2.0,
                }
            };
            egui::Grid::new("main_grid").show(ui, |ui| {
                ui.allocate_ui(cell_size, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Row 1, Column 1");
                    });
                });
                
                ui.allocate_ui(cell_size, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Row 1, Column 2");
                    });
                });
                ui.end_row();

                ui.allocate_ui(cell_size, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Row 2, Column 1");
                    });
                });
                
                ui.allocate_ui(cell_size, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Row 2, Column 2");
                    });
                });
                ui.end_row();
            });
        });
    }
}
