use tic_tac::game::App;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([280.0, 340.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Tic Tac Toe",
        native_options,
        Box::new(|_cc| Box::new(App::default())),
    )
}
