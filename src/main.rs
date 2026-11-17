mod app;
mod biology;
mod environment;
mod simulation;

fn main() -> eframe::Result {
  let options = eframe::NativeOptions::default();

  eframe::run_native(
    "Underbloom",
    options,
    Box::new(|_cc| Ok(Box::new(app::Underbloom::default()))),
  )
}
