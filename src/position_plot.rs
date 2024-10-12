use egui_plot::{Legend, Line, Plot};

#[derive(Debug, Default)]
pub struct PositionPlot {
    x: Vec<[f64; 2]>,
    x_t: Vec<[f64; 2]>,
    x_tt: Vec<[f64; 2]>,
}

impl PositionPlot {
    pub fn reset(&mut self) {
        self.x.clear();
        self.x_t.clear();
        self.x_tt.clear();
    }

    pub fn add(&mut self, t: f64, x: f64, x_t: f64, x_tt: f64) {
        self.x.push([t, x]);
        self.x_t.push([t, x_t]);
        self.x_tt.push([t, x_tt]);
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        let plot = Plot::new("position_plot")
            .legend(Legend::default())
            .show_axes(true)
            .show_grid(true);

        plot.show(ui, |ui| {
            ui.line(Line::new(self.x.clone()).name("x(t)"));
            ui.line(Line::new(self.x_t.clone()).name("x_t(t)"));
            ui.line(Line::new(self.x_tt.clone()).name("x_tt(t)"));
        });
    }
}
