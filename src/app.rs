use chrono::{DateTime, Local};
use egui::{Layout, Vec2, Widget};
use egui_plot::{Legend, Line, Plot, PlotPoints};

use crate::{function::{ConstFunction, Function}, function_builder::FunctionBuilder};

#[derive(Debug)]
pub struct App {
    speed: f64,
    previous_time: DateTime<Local>,
    x: f64,
    v: f64,
    x_0: f64,
    v_0: f64,
    m: f64,
    c: f64,
    k: f64,
    delta_t: f64,
    tick: f64,
    w: Function,
    h: Function,
    w_builder: FunctionBuilder,
    h_builder: FunctionBuilder,
    run: bool,
    time_points: Vec<f64>,
    x_points: Vec<f64>,
}

impl App {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
            previous_time: Local::now(),
            x: 1.0,
            v: 0.0,
            x_0: 0.0,
            v_0: 1.0,
            m: 1.0,
            c: 0.1,
            k: 0.1,
            delta_t: 20.0 / 1000.0,
            tick: 0.0,
            w: Function::Const(ConstFunction::new(0.0)),
            h: Function::Const(ConstFunction::new(0.0)),
            w_builder: FunctionBuilder::default(),
            h_builder: FunctionBuilder::default(),
            run: false,
            time_points: vec![0.0],
            x_points: vec![0.0],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_time = Local::now();
        let delta = current_time - self.previous_time;
        self.previous_time = current_time;

        if self.run {
            self.tick += delta.num_milliseconds() as f64 / 1000.0;
            while self.tick >= self.delta_t / self.speed {
                self.tick -= self.delta_t / self.speed;

                let x = self.x;
                let t = *self.time_points.last().unwrap();

                self.x += self.v * self.delta_t;
                self.v += self.delta_t * (self.c * (self.w.get_value(t) - x) - self.k * self.v + self.h.get_value(t)) / self.m;

                let new_time_point = t + self.delta_t;
                self.time_points.push(new_time_point);
                self.x_points.push(self.x);
            }
        }

        egui::SidePanel::left("settings_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("run").clicked() {
                    self.run = true;
                }
    
                if ui.button("stop").clicked() {
                    self.run = false;
                }

                if ui.button("reset").clicked() {
                    self.run = false;
                }
            });

            egui::Slider::new(&mut self.speed, 0.1..=10.0).text("speed").ui(ui);
            egui::Slider::new(&mut self.delta_t, 0.01..=0.5).step_by(0.01).text("delta_t").ui(ui);
            egui::Slider::new(&mut self.x_0, 0.01..=0.5).text("x_0").ui(ui);
            egui::Slider::new(&mut self.v_0, 0.01..=0.5).text("v_0").ui(ui);
            egui::Slider::new(&mut self.m, 1.0..=50.0).text("m").ui(ui);
            egui::Slider::new(&mut self.c, 0.1..=1.0).text("c").ui(ui);
            egui::Slider::new(&mut self.k, 0.1..=1.0).text("k").ui(ui);

            ui.add_space(15.0);

            ui.label("w(t):");
            if self.w_builder.show(ui) {
                self.w = self.w_builder.build();
            }

            ui.add_space(15.0);

            ui.label("h(t):");
            if self.h_builder.show(ui) {
                self.h = self.h_builder.build();
            }

            ui.with_layout(Layout::bottom_up(egui::Align::Min), |ui| {
                let fps = 1000.0 / delta.num_milliseconds() as f64;
                ui.label(format!("FPS: {:.1}", fps,));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let cell_size = {
                let size = ui.available_size();
                Vec2 {
                    x: size.x / 2.0,
                    y: size.y / 2.0,
                }
            };
            let current_size = ui.min_rect();
            egui::Grid::new("main_grid").show(ui, |ui| {
                ui.allocate_ui(cell_size, |ui| {
                    ui.centered_and_justified(|ui| {
                        let y = cell_size.y / 2.0;
                        let x_space = cell_size.x - 40.0;
                        let x_min = 0.2 * x_space;
                        let x = x_min + (self.x as f32 + 1.0) * 0.5 * (x_space - x_min);

                        let start = egui::pos2( 20.0 + current_size.x_range().min, y + current_size.y_range().min);
                        let end = egui::pos2(20.0 + x + current_size.x_range().min, y + current_size.y_range().min);

                        let painter = ui.painter();
                        painter.line_segment([start, end], egui::Stroke::new(2.0, egui::Color32::RED));
                    });
                });
                
                ui.allocate_ui(cell_size, |ui| {
                    ui.centered_and_justified(|ui| {
                        let plot = Plot::new("plot1")
                            .legend(Legend::default())
                            .show_axes(true)
                            .show_grid(true)
                            .data_aspect(1.0);

                        plot.show(ui, |ui| {
                            let points: PlotPoints = self.time_points
                                .iter()
                                .zip(self.x_points.iter())
                                .map(|(&x, &y)| {
                                    [x, y]
                                })
                                .collect();
                            ui.line(Line::new(points));
                        });
                    });
                });
                ui.end_row();

                ui.allocate_ui(cell_size, |ui| {
                    ui.centered_and_justified(|ui| {
                        let plot = Plot::new("plot2")
                            .legend(Legend::default())
                            .show_axes(true)
                            .show_grid(true)
                            .data_aspect(1.0);

                        plot.show(ui, |ui| {
                            let points: PlotPoints = self.time_points
                                .iter()
                                .zip(self.x_points.iter())
                                .map(|(&x, &y)| {
                                    [x, y]
                                })
                                .collect();
                            ui.line(Line::new(points));
                        });
                    });
                });
                
                ui.allocate_ui(cell_size, |ui| {
                    ui.centered_and_justified(|ui| {
                        let plot = Plot::new("plot3")
                            .legend(Legend::default())
                            .show_axes(true)
                            .show_grid(true)
                            .data_aspect(1.0);

                        plot.show(ui, |ui| {
                            let points: PlotPoints = self.time_points
                                .iter()
                                .zip(self.x_points.iter())
                                .map(|(&x, &y)| {
                                    [x, y]
                                })
                                .collect();
                            ui.line(Line::new(points));
                        });
                    });
                });
                ui.end_row();
            });
        });

        ctx.request_repaint();
    }
}
