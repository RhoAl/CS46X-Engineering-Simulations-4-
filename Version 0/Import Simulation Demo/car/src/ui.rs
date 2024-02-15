use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::build::CarDefinition;
use crate::control::CarControl;

pub fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Dev Tools").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui|{
            if ui.button("Reset").clicked(){
                
            }
        });
        ui.horizontal(|ui|{
            if ui.button("Map 1").clicked(){
                
            }
            if ui.button("Map 2").clicked(){
                
            }
        });
    });
}

pub fn stat_ui(mut contexts: EguiContexts, mut car:ResMut<CarDefinition>, mut carcontrol:ResMut<CarControl>) {
    egui::Window::new("Stats").show(contexts.ctx_mut(), |ui|{
        ui.horizontal(|ui|{
            ui.add(egui::DragValue::new(&mut car.wheel.coefficient_of_friction)
                .speed(0.1)
                .clamp_range(0..=10)
            );
            ui.label("Wheel Friction Coefficient: ");
            ui.label(car.wheel.coefficient_of_friction.to_string());
            ui.label("Throttle: ");
            ui.label(carcontrol.throttle.to_string());
            
        });
    });
}