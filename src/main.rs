// TODO: scale up UI
// TODO: add on the plot the point where we are
// TODO: Add Axes desc
// TODO: make polish symbols

use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};
use macroquad::prelude::*; // Import necessary components

// all ui got the same width

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
    let mut avg: f64 = 0.0;
    let mut allowance: f64 = 0.0;
    let mut initialization = true;
    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            // enter key will act a TAB e.g. cycle through widgets
            egui_macroquad::egui::CentralPanel::default().show(egui_ctx, |ui| {
                let window_width = ui.available_width();
                let window_height = ui.available_height();
                let widget_width = window_width / 5.0;
                let widget_height = window_height / 10.0;
                let font_size = widget_height / 4.0;

                ui.style_mut().spacing.slider_width = widget_width;
                ui.style_mut().spacing.interact_size.y = widget_height;

                ui.style_mut().text_styles.insert(
                    egui_macroquad::egui::TextStyle::Body,
                    egui_macroquad::egui::FontId::new(
                        font_size,
                        egui_macroquad::egui::FontFamily::Proportional,
                    ),
                );

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        // język polski
                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [widget_width, widget_height],
                                egui_macroquad::egui::Label::new(
                                    egui_macroquad::egui::RichText::new(format!(
                                        "Język polski:   "
                                    ))
                                    .size(font_size),
                                ),
                            );
                            let pol_slider = ui.add_sized(
                                [widget_width, widget_height * 0.5],
                                egui_macroquad::egui::Slider::new(&mut pol_value, 1.0..=6.0)
                                    .step_by(0.01)
                                    .show_value(false),
                            );
                            ui.add(egui_macroquad::egui::Label::new(
                                egui_macroquad::egui::RichText::new(format!("{:.2}", pol_value))
                                    .size(font_size),
                            ));
                            if initialization {
                                pol_slider.request_focus();
                                initialization = false;
                            }
                        });
                        // matematyka
                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [widget_width, widget_height],
                                egui_macroquad::egui::Label::new(
                                    egui_macroquad::egui::RichText::new(format!(
                                        "Matematyka:     "
                                    ))
                                    .size(font_size),
                                ),
                            );
                            ui.add_sized(
                                [widget_width, widget_height],
                                egui_macroquad::egui::Slider::new(&mut mat_value, 1.0..=6.0)
                                    .step_by(0.01)
                                    .show_value(false),
                            );
                            ui.add(egui_macroquad::egui::Label::new(
                                egui_macroquad::egui::RichText::new(format!("{:.2}", mat_value))
                                    .size(font_size),
                            ));
                        });
                        // język angielski
                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [widget_width, widget_height],
                                egui_macroquad::egui::Label::new(
                                    egui_macroquad::egui::RichText::new(format!(
                                        "Język angielski:"
                                    ))
                                    .size(font_size),
                                ),
                            );
                            ui.add_sized(
                                [widget_width, widget_height],
                                egui_macroquad::egui::Slider::new(&mut ang_value, 1.0..=6.0)
                                    .step_by(0.01)
                                    .show_value(false),
                            );
                            ui.add(egui_macroquad::egui::Label::new(
                                egui_macroquad::egui::RichText::new(format!("{:.2}", ang_value))
                                    .size(font_size),
                            ));
                        });
                        // informatyka
                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [widget_width, widget_height],
                                egui_macroquad::egui::Label::new(
                                    egui_macroquad::egui::RichText::new(format!(
                                        "Informatyka:    "
                                    ))
                                    .size(font_size),
                                ),
                            );
                            ui.add_sized(
                                [widget_width, widget_height],
                                egui_macroquad::egui::Slider::new(&mut inf_value, 1.0..=6.0)
                                    .step_by(0.01)
                                    .show_value(false),
                            );
                            ui.add(egui_macroquad::egui::Label::new(
                                egui_macroquad::egui::RichText::new(format!("{:.2}", inf_value))
                                    .size(font_size),
                            ));
                        });
                        ui.horizontal(|ui| {
                            avg = calculate_average(pol_value, ang_value, mat_value, inf_value);
                            ui.label(
                                egui_macroquad::egui::RichText::new(format!("Średnia: "))
                                    .size(font_size),
                            );
                            ui.label(
                                egui_macroquad::egui::RichText::new(format!("{:.2}", avg))
                                    .color(if avg <= 4.0 {
                                        egui_macroquad::egui::Color32::RED
                                    } else {
                                        egui_macroquad::egui::Color32::YELLOW
                                    })
                                    .size(font_size), // rozmiar czcionki w punktach
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(
                                egui_macroquad::egui::RichText::new(format!("Kieszonkowe: "))
                                    .color(egui_macroquad::egui::Color32::BLUE)
                                    .size(font_size), // rozmiar czcionki w punktach
                            );

                            allowance =
                                calculate_allowance(pol_value, ang_value, mat_value, inf_value);

                            ui.label(
                                egui_macroquad::egui::RichText::new(format!("{:.2} zł", allowance))
                                    .color(egui_macroquad::egui::Color32::GREEN)
                                    .size(font_size), // rozmiar czcionki w punktach
                            );
                        });
                    });

                    // Process keys, mouse etc.
                    //ui.memory_mut(|mem| mem.request_focus(egui_macroquad::egui::Id::new(_)));

                    let points = PlotPoints::from_iter(vec![
                        [1.0, calculate_allowance(1.0, 1.0, 1.0, 1.0)],
                        [2.0, calculate_allowance(2.0, 2.0, 2.0, 2.0)],
                        [3.0, calculate_allowance(3.0, 3.0, 3.0, 3.0)],
                        [4.0, calculate_allowance(4.0, 4.0, 4.0, 4.0)],
                        [5.0, calculate_allowance(5.0, 5.0, 5.0, 5.0)],
                        [6.0, calculate_allowance(6.0, 6.0, 6.0, 6.0)],
                    ]);
                    let line = Line::new("linie", points);
                    let avg_point = PlotPoints::from_iter(vec![[avg, allowance]]);
                    Plot::new("my_plot")
                        .x_axis_label("Średnia")
                        .y_axis_label("Kasa")
                        .show(ui, |plot_ui| {
                            // Customize axes, legends, etc. here
                            //println!("GUI X width: {}", ui.available_width());
                            plot_ui.line(line);
                            plot_ui
                                .points(egui_plot::Points::new("średnia", avg_point).radius(7.0));
                        });
                });
            });
        });

        // Draw things before egui

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
