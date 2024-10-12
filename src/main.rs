use app::App;

pub mod app;
pub mod function;
pub mod function_builder;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Spring simulation",
        native_options,
        Box::new(|_| Ok(Box::new(App::new()))),
    )
}
