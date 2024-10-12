use egui_plot::{Legend, Line, Plot};

#[derive(Debug, Default)]
pub struct TrajectoryPlot {
    t: Vec<[f64; 2]>,
}

impl TrajectoryPlot {
    pub fn reset(&mut self) {
        self.t.clear();
    }

    pub fn add(&mut self, x: f64, x_t: f64) {
        self.t.push([x, x_t]);
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        let plot = Plot::new("trajectory_plot")
            .legend(Legend::default())
            .show_axes(true)
            .show_grid(true)
            .data_aspect(1.0)
            .x_axis_label("x")
            .y_axis_label("x_t");

        plot.show(ui, |ui| {
            ui.line(Line::new(self.t.clone()));
        });
    }
}
