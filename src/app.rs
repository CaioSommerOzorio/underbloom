use eframe::egui;
use egui_autocomplete::AutoCompleteTextEdit;
use std::time::{Duration, Instant};

use crate::biology::species::all_species_names;
use crate::simulation::tick::tick;
use crate::simulation::world::World;

#[derive(PartialEq)]
enum Menu {
  Home,
  Pot,
  SpeciesInfo,
}

pub struct Underbloom {
  world: World,
  last_tick: Instant,
  tick_accumulator: Duration,
  current_menu: Menu,
  pot_inspect: usize,
  species_inspect: String,
  buffer: String,
}

impl Default for Underbloom {
  fn default() -> Self {
    Self {
      world: World {
        time: 0,
        pots: Vec::new(),
      },

      last_tick: Instant::now(),
      tick_accumulator: Duration::ZERO,
      current_menu: Menu::Home,
      pot_inspect: 0,
      species_inspect: String::new(),
      buffer: String::new(),
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
            ui.collapsing(format!("Pot {}", i), |ui| {
              ui.label(format!("Temperature: {}", &self.world.pots[i].temp));
              ui.label(format!("Moisture: {}", &self.world.pots[i].moisture));
              if ui.add(egui::Button::new("View pot")).clicked() {
                self.pot_inspect = i;
                self.current_menu = Menu::Pot;
              }
            });
          });
        }
        ui.add_space(12.0);
        if ui.add(egui::Button::new("Add a pot")).clicked() {
          self.world.add_pot();
        }
      });
    } else if self.current_menu == Menu::Pot {
      egui::Panel::top("header").show(ui, |ui| {
        ui.horizontal(|ui| {
          ui.heading("Underbloom");
          ui.separator();
          ui.heading(format!("Pot {}", self.pot_inspect));
        });
      });

      egui::CentralPanel::default().show(ui, |ui| {
        ui.heading("Temperature");
        ui.add(egui::Slider::new(
          &mut self.world.pots[self.pot_inspect].temp,
          -50.0..=50.0,
        ));
        ui.heading("Moisture");
        ui.add(egui::Slider::new(
          &mut self.world.pots[self.pot_inspect].moisture,
          -0.0..=100.0,
        ));

        let species = all_species_names();
        egui::ComboBox::from_label("Select one!")
          .selected_text(format!("{:?}", self.buffer))
          .show_ui(ui, |ui| {
            for i in 0..species.len() {
              ui.selectable_value(&mut self.buffer, species[i].clone(), &species[i]);
            }
          });

        if ui.add(egui::Button::new("Add a colony")).clicked() {
          self.world.pots[self.pot_inspect].add_colony(&self.buffer);
        }
        ui.heading("Colonies");
        let colonies = &self.world.pots[self.pot_inspect].colonies;
        for colony in colonies {
          ui.collapsing(&colony.species.name, |ui| {
            ui.label(format!("Biomass: {}g", colony.biomass));
            ui.label(format!("Age: {}", colony.age));
            if ui.add(egui::Button::new("Inspect Species")).clicked() {
              self.current_menu = Menu::SpeciesInfo;
              self.species_inspect = colony.species.name.clone();
            }
          });
        }
        if ui.add(egui::Button::new("Return to main menu")).clicked() {
          self.current_menu = Menu::Home;
        }
      });
    }
  }
}
