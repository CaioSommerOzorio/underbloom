use eframe::egui;
use std::time::{Duration, Instant};

use crate::simulation::tick::tick;
use crate::simulation::world::World;

pub struct Underbloom {
  label: String,
  value: f32,

  world: World,
  last_tick: Instant,
  tick_accumulator: Duration,
}

impl Default for Underbloom {
  fn default() -> Self {
    Self {
      label: String::new(),
      value: 0.0,

      world: World {
        time: 0,
        areas: Vec::new(),
      },
      last_tick: Instant::now(),
      tick_accumulator: Duration::ZERO,
    }
  }
}

impl eframe::App for Underbloom {
  fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    let now = Instant::now();
    let elapsed = now - self.last_tick;

    self.last_tick = now;
    self.tick_accumulator += elapsed;

    let tick_duration = Duration::from_millis(500);

    while self.tick_accumulator >= tick_duration {
      tick(&mut self.world);
      self.tick_accumulator -= tick_duration;
    }

    ui.ctx()
      .request_repaint_after(tick_duration - self.tick_accumulator);
    egui::CentralPanel::default().show(ui, |ui| {
      ui.heading("Underbloom");

      ui.horizontal(|ui| {
        ui.label("Write something: ");
        ui.text_edit_singleline(&mut self.label);
      });

      ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));

      if ui.button("hi").clicked() {
        self.value += 1.0;
      }

      ui.label(format!("Hello '{}', value: {}", self.label, self.value));
      ui.label(format!("Ticks: {}", self.world.time));
    });
  }
}
