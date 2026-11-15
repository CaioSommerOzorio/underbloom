use eframe::egui;

mod biology;
mod simulation;
mod environment;


fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Underbloom",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    label: String,
    value: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: String::new(),
            value: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Underbloom");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));

            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.label(format!("Hello '{}', value: {}", self.label, self.value));
        });
    }
}
