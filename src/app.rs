use eframe::egui;
use std::time::{Duration, Instant};

use crate::simulation::tick::tick;
use crate::simulation::world::World;

#[derive(PartialEq)]
enum Menu {
  Home,
  Pot,
}

pub struct Underbloom {
  label: String,
  value: f32,

  world: World,
  last_tick: Instant,
  tick_accumulator: Duration,
  current_menu: Menu,
}

impl Default for Underbloom {
  fn default() -> Self {
    Self {
      label: String::new(),
      value: 0.0,

      world: World {
        time: 0,
        pots: Vec::new(),
      },

      last_tick: Instant::now(),
      tick_accumulator: Duration::ZERO,
      current_menu: Menu::Home,
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

    egui::Panel::bottom("footer").show(ui, |ui| {
      ui.horizontal(|ui| {
        ui.label(format!("World age: {}h", self.world.time));
      });
    });

    if self.current_menu == Menu::Home {
      egui::Panel::top("header").show(ui, |ui| {
        ui.horizontal(|ui| {
          ui.heading("Underbloom");
          ui.separator();
          ui.heading("Pots");
        });
      });

      egui::CentralPanel::default().show(ui, |ui| {
        for i in 0..self.world.pots.len() {
          ui.horizontal(|ui| {
            ui.label(format!("Pot {}", i));
          });
        }
        if ui.add(egui::Button::new("Add a pot")).clicked() {
          self.world.add_pot();
        }
      });
    }
  }
}
