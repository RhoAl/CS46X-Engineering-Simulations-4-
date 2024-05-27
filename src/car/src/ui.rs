use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::build::CarDefinition;
use crate::control::CarControl;
use rigid_body::{
    joint::Joint,
    sva::{Force, Vector},
};
use crate::tire::PointTire;

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

pub fn stat_ui(mut contexts: EguiContexts, mut car:ResMut<CarDefinition>, carcontrol:ResMut<CarControl>, mut tire_query: Query<&mut PointTire>, mut query_joints: Query<&mut Joint>) {
    egui::Window::new("Stats").show(contexts.ctx_mut(), |ui|{
        ui.horizontal(|ui|{
            let mut RoundSpeedValue = 0.0;
            ui.label("Speed: ");
            for mut tire in tire_query.iter_mut() {
               if let Ok([mut joint]) =
                    query_joints.get_many_mut([tire.joint_entity()])
                {
                let mut SpeedValue = joint.qd.abs();
                RoundSpeedValue = (SpeedValue * 10.0).round() / 10.0;

                }
            }
            ui.label(RoundSpeedValue.to_string());

            //ui.label(SpeedValue.to_string());
            ui.label("Throttle: ");
            let mut Throttle = carcontrol.throttle;
            let mut RoundThrottleValue = (Throttle * 100.0).round() / 100.0;
            ui.label(RoundThrottleValue.to_string());

        });
    });
}