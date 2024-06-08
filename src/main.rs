#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::num::ParseIntError;

mod crc;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 380.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CRC Paweł Perek",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_zoom_factor(2.0);

            Box::<MyApp>::default()
        }),
    )
}

const MAX_ITERATIONS: usize = 1_000_000_000;

struct MyApp {
    input: String,
    iterations: usize,
    result: Option<String>,
    error: Option<String>,
    execution_time: Option<std::time::Duration>,
    iteration_time: Option<std::time::Duration>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input: "10000111".to_string(),
            iterations: 1,
            result: None,
            error: None,
            execution_time: None,
            iteration_time: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Licznik CRC");
            ui.horizontal(|ui| {
                let name_label = ui.label("Bity: ");
                ui.text_edit_singleline(&mut self.input)
                    .labelled_by(name_label.id);
            });

            ui.horizontal(|ui| {
                let slider_label = ui.label("Liczba powtórzeń: ");
                ui.add(
                    egui::Slider::new(&mut self.iterations, 1..=MAX_ITERATIONS).logarithmic(true),
                )
                .labelled_by(slider_label.id);
            });

            if ui.button("Oblicz").clicked() {
                let clean_input = self.input.replace(" ", "");


                if clean_input.len() > 96 {
                    self.error = Some("Maksymalna długość ciągu to 96 bitów".to_string());
                    return;
                }
                
                let iterations = self.iterations;


                let parse_result: Result<Vec<usize>, ParseIntError> = clean_input
                    .as_str()
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(8)
                    .map(|chunk| {
                        let chunk_str: String = chunk.iter().collect();
                        usize::from_str_radix(&chunk_str, 2)
                    })
                    .collect();

                if parse_result.is_err() {
                    self.error = Some(format!("Błąd parsowania: {:?}", parse_result.err().unwrap()));
                    return;
                }

                let bytes = parse_result.unwrap();

                let mut output: usize = 0;

                let execution_timer = std::time::Instant::now();

                for _ in 0..iterations {
                    output = crc::CRC::calculate(&bytes);
                }

                let elapsed = execution_timer.elapsed();

                self.execution_time = Some(elapsed);
                self.iteration_time = Some(elapsed / iterations as u32);
                self.result = Some(format!("{:04X}", output));
                self.error = None;
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Wynik: ");

                if let Some(result) = &self.result {
                    ui.label(result);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Czas wykonania: ");

                if let Some(execution_time) = &self.execution_time {
                    ui.label(format!("{:?}", execution_time));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Czas iteracji: ");

                if let Some(iteration_time) = &self.iteration_time {
                    ui.label(format!("{:?}", iteration_time));
                }
            });

            if let Some(error) = &self.error {
                ui.label(eframe::egui::RichText::new(error).color(egui::Color32::RED));
            }
        });
    }
}
