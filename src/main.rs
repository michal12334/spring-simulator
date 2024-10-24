use app::App;

pub mod app;
pub mod forces_plot;
pub mod function;
pub mod function_builder;
pub mod position_plot;
pub mod trajectory_plot;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 750.0])
            .with_min_inner_size([800.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Spring simulation",
        native_options,
        Box::new(|_| Ok(Box::new(App::new()))),
    )
}
