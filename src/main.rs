// TODO: scale up UI
// TODO: add on the plot the point where we are
// TODO: Add Axes desc
// TODO: make polish symbols

use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};
use macroquad::prelude::*; // Import necessary components

//thrs=4.0;
//x=(($1+$2+$3+$4)/4.0);
//kasa=150.0*( (x - thrs) > 0 ? (x- thrs): 0);
fn calculate_allowance(pol: f32, ang: f32, mat: f32, inf: f32) -> f64 {
    let threshold = 4.0;
    let average = calculate_average(pol, ang, mat, inf);
    let allowance = 150.0
        * if average > threshold {
            average - threshold
        } else {
            0.0
        };
    allowance
}

fn calculate_average(pol: f32, ang: f32, mat: f32, inf: f32) -> f64 {
    ((pol + ang + mat + inf) / 4.0).into()
}

#[macroquad::main("egui with macroquad")]
async fn main() {
    let mut pol_value: f32 = 3.0;
    let mut ang_value: f32 = 3.0;
    let mut mat_value: f32 = 3.0;
    let mut inf_value: f32 = 3.0;
    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            egui_macroquad::egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                // jezyk polski
                ui.horizontal(|ui| {
                    ui.label("jezyk polski:   ");
                    ui.add(egui_macroquad::egui::Slider::new(&mut pol_value, 1.0..=6.0));
                });
                // matematyka
                ui.horizontal(|ui| {
                    ui.label("matematyka:     ");
                    ui.add(egui_macroquad::egui::Slider::new(&mut mat_value, 1.0..=6.0));
                });
                // jezyk angielski
                ui.horizontal(|ui| {
                    ui.label("jezyk angielski:");
                    ui.add(egui_macroquad::egui::Slider::new(&mut ang_value, 1.0..=6.0));
                });
                // informatyka
                ui.horizontal(|ui| {
                    ui.label("informatyka:    ");
                    ui.add(egui_macroquad::egui::Slider::new(&mut inf_value, 1.0..=6.0));
                });
                ui.horizontal(|ui| {
                    let avg = calculate_average(pol_value, ang_value, mat_value, inf_value);
                    ui.label(egui_macroquad::egui::RichText::new(format!("Srednia: ")).size(24.0));
                    ui.label(
                        egui_macroquad::egui::RichText::new(format!("{:.2}", avg))
                            .color(if avg <= 4.0 {
                                egui_macroquad::egui::Color32::RED
                            } else {
                                egui_macroquad::egui::Color32::YELLOW
                            })
                            .size(24.0), // rozmiar czcionki w punktach
                    );
                });
                ui.horizontal(|ui| {
                    ui.label(
                        egui_macroquad::egui::RichText::new(format!("Kieszonkowe: "))
                            .color(egui_macroquad::egui::Color32::BLUE)
                            .size(24.0), // rozmiar czcionki w punktach
                    );
                    ui.label(
                        egui_macroquad::egui::RichText::new(format!(
                            "{:.2} zl",
                            calculate_allowance(pol_value, ang_value, mat_value, inf_value)
                        ))
                        .color(egui_macroquad::egui::Color32::GREEN)
                        .size(24.0), // rozmiar czcionki w punktach
                    );
                });

                // Process keys, mouse etc.
                let points = PlotPoints::from_iter(vec![
                    [1.0, calculate_allowance(1.0, 1.0, 1.0, 1.0)],
                    [2.0, calculate_allowance(2.0, 2.0, 2.0, 2.0)],
                    [3.0, calculate_allowance(3.0, 3.0, 3.0, 3.0)],
                    [4.0, calculate_allowance(4.0, 4.0, 4.0, 4.0)],
                    [5.0, calculate_allowance(5.0, 5.0, 5.0, 5.0)],
                    [6.0, calculate_allowance(6.0, 6.0, 6.0, 6.0)],
                ]);
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
