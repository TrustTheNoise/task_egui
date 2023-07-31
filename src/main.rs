mod app;
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "task_egui",
        native_options,
        Box::new(|_cc| Box::new(app::TaskApp::default())),
    )
    .unwrap();
}

