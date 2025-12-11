use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};
use macroquad::prelude::*; // Import necessary components

#[macroquad::main("egui with macroquad")]
async fn main() {
    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            egui_macroquad::egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.label("Test");

                // Process keys, mouse etc.
                let points = PlotPoints::from_iter(vec![[0.0, 1.0], [1.0, 3.0], [2.0, 2.0]]);
                let line = Line::new("linie", points);

                Plot::new("my_plot").show(ui, |plot_ui| {
                    // Customize axes, legends, etc. here
                    plot_ui.line(line);
                });
            });
        });

        // Draw things before egui

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
