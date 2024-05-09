#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, style::HandleShape};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // .with_close_button(false),
            // .with_transparent(true), // need to set the transparency in the eframe or change color wbeing written
            .with_inner_size([600.0, 300.0])
            .with_position((1100.0, 200.0)),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("(xp)\nHello World!\n(simple)");
        });

        let mut x = false; // I think this gets refdefined every time -- I need to see where the main loop is
        let mut text = "".to_string();
        egui::Window::new("My Window").show(ctx, |ui| {
            let mut uivar = ui.label(format!("Bap, Bap: {:?}", x));
            let uivar2 = ui.text_edit_singleline(&mut name);
            let x = if uivar2.has_focus() { true } else { false };
            uivar = ui.label(format!("Bap, Bap: {:?}", x));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });

            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(
                egui::Slider::new(&mut age, 0..=120_000_000)
                    .text("age")
                    .drag_value_speed(0.5)
                    .logarithmic(true)
                    .handle_shape(HandleShape::Rect {
                        aspect_ratio: (1.5),
                    }),
            );
            if ui.button("Increment").clicked() {
                age += 1;
            }
            ui.heading("My egui Application");
            ui.label(format!("Hello '{name}', age {age}"));
            ui.add(
                egui::Slider::new(&mut age, 0..=120_000_000)
                    .text("age")
                    .drag_value_speed(0.5)
                    .logarithmic(true)
                    .handle_shape(HandleShape::Rect {
                        aspect_ratio: (1.5),
                    }),
            );
        });
    })
}
