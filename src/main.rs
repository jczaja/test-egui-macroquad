// TODO: scale up UI
// TODO: add on the plot the point where we are
// TODO: Add Axes desc
// TODO: make polish symbols

use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};
use macroquad::prelude::*; // Import necessary components

/* 

To normalne zachowanie egui – domyślnie widgety takie jak slider czy label mają stały rozmiar i nie skalują się automatycznie przy zmianie rozmiaru okna. Plot jest wyjątkiem, bo domyślnie wypełnia dostępne miejsce.

**Jak sprawić, by slidery i labelki skalowały się razem z oknem?**

Musisz ręcznie ustawić ich szerokość na podstawie dostępnej szerokości okna. Możesz to zrobić tak:

```rust
let available_width = ui.available_width();

ui.add_sized([available_width, 24.0], egui_macroquad::egui::Slider::new(&mut pol_value, 1.0..=6.0));
```

Podobnie dla labeli, możesz użyć `ui.allocate_ui_with_layout` lub `ui.with_layout`, by rozciągnąć je na całą szerokość.

**Przykład dla kilku widgetów:**
```rust
let available_width = ui.available_width();

ui.horizontal(|ui| {
    ui.label("jezyk polski:");
    ui.add_sized([available_width - 100.0, 24.0], egui_macroquad::egui::Slider::new(&mut pol_value, 1.0..=6.0));
});
```
Odejmij od szerokości tyle, ile zajmuje labelka, żeby slider nie wychodził poza okno.

**Podsumowanie:**  
W egui musisz samodzielnie ustawiać rozmiary widgetów, jeśli chcesz, by skalowały się razem z oknem. Używaj `ui.add_sized` i `ui.available_width()`.


*/












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
    let mut allowance : f64 = 0.0;
    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            egui_macroquad::egui::CentralPanel::default().show(egui_ctx, |ui| {
                // język polski
                ui.horizontal(|ui| {
                    ui.label("język polski:   ");
                    ui.add(egui_macroquad::egui::Slider::new(&mut pol_value, 1.0..=6.0));
                });
                // matematyka
                ui.horizontal(|ui| {
                    ui.label("matematyka:     ");
                    ui.add(egui_macroquad::egui::Slider::new(&mut mat_value, 1.0..=6.0));
                });
                // język angielski
                ui.horizontal(|ui| {
                    ui.label("język angielski:");
                    ui.add(egui_macroquad::egui::Slider::new(&mut ang_value, 1.0..=6.0));
                });
                // informatyka
                ui.horizontal(|ui| {
                    ui.label("informatyka:    ");
                    ui.add(egui_macroquad::egui::Slider::new(&mut inf_value, 1.0..=6.0));
                });
                ui.horizontal(|ui| {
                    avg = calculate_average(pol_value, ang_value, mat_value, inf_value);
                    ui.label(egui_macroquad::egui::RichText::new(format!("Średnia: ")).size(24.0));
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

                    allowance = calculate_allowance(pol_value, ang_value, mat_value, inf_value);

                    ui.label(
                        egui_macroquad::egui::RichText::new(format!(
                            "{:.2} zł", allowance
                            
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
                let avg_point = PlotPoints::from_iter(vec![[avg, allowance]]);
                Plot::new("my_plot").x_axis_label("Średnia").y_axis_label("Kasa").show(ui, |plot_ui| {
                    // Customize axes, legends, etc. here
                    //println!("GUI X width: {}", ui.available_width());
                    plot_ui.line(line);
                    plot_ui.points(
                        egui_plot::Points::new("średnia", avg_point).radius(7.0),
                    );
                });
            });
        });

        // Draw things before egui

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
