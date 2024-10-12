use egui_plot::{Legend, Line, Plot};

#[derive(Debug, Default)]
pub struct ForcesPlot {
    f: Vec<[f64; 2]>,
    g: Vec<[f64; 2]>,
    h: Vec<[f64; 2]>,
    w: Vec<[f64; 2]>,
}

impl ForcesPlot {
    pub fn reset(&mut self) {
        self.f.clear();
        self.g.clear();
        self.h.clear();
        self.w.clear();
    }

    pub fn add(&mut self, t: f64, f: f64, g: f64, h: f64, w: f64) {
        self.f.push([t, f]);
        self.g.push([t, g]);
        self.h.push([t, h]);
        self.w.push([t, w]);
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        let plot = Plot::new("forces_plot")
            .legend(Legend::default())
            .show_axes(true)
            .show_grid(true);

        plot.show(ui, |ui| {
            ui.line(Line::new(self.f.clone()).name("f(t)"));
            ui.line(Line::new(self.g.clone()).name("g(t)"));
            ui.line(Line::new(self.h.clone()).name("h(t)"));
            ui.line(Line::new(self.w.clone()).name("w(t)"));
        });
    }
}
