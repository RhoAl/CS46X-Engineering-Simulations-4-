use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_mod_raycast::prelude::*;
use car::tire::PointTire;
use rigid_body::{
    joint::Joint,
    sva::{Force, Vector},
};

//use std::env;
use bevy_integrator::{SimTime, Solver};
use car::{
    build::{build_car, car_startup_system},
    environment::build_environment,
    setup::{camera_setup, simulation_setup},
    ui::{ui_example_system, stat_ui},
};
use rigid_body::plugin::RigidBodyPlugin;

// Main function
fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let car_definition = build_car();
    // Create App
    App::new()
        .add_plugins(RigidBodyPlugin {
            time: SimTime::new(0.01, 0.0, None),
            solver: Solver::RK4,
            simulation_setup: vec![simulation_setup],
            environment_setup: vec![camera_setup],
            name: "car_demo".to_string(),
        })
        .add_plugins(EguiPlugin)
        .insert_resource(car_definition)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, stat_ui)
        .add_systems(Startup, car_startup_system)
        .add_systems(Startup, build_environment)
        // .add_systems(Update, raycast)
        .run();
}

const DIST: Vec3 = Vec3::new(-10.0, 30.0, 0.0);

// This is all that is needed to raycast into the world! You can also use the normal, non-debug
// version (raycast.cast_ray) when you don't need to visualize the ray or intersections.
fn raycast(
    mut raycast: Raycast, 
    mut gizmos: Gizmos, 
    time: Res<Time>, 
    mut query_joints: Query<&mut Joint>, 
    mut tire_query: Query<&mut PointTire>,
) {
    let dir = Vec3::new(0.0, 0.0, -1.0);

    for mut tire in tire_query.iter_mut() {
        if let Ok([mut joint, parent]) =
            query_joints.get_many_mut([tire.joint_entity(), tire.joint_parent()])
        {
            let xp0 = parent.x.inverse(); // spatial transform from the parent joint to absolute coordinates
            let center_abs = xp0.transform_point(Vector::zeros()); // center of the tire in absolute coordinates

            let pos = Vec3::new(center_abs[0] as f32, center_abs[1] as f32, (center_abs[2] as f32) - 0.3);
            let hits = raycast.debug_cast_ray(Ray3d::new(pos, dir), &default(), &mut gizmos,);

            // for &(entity, ref intersection_data) in hits {
            //     // Access the IntersectionData for each hit
            //     println!("{:?}", intersection_data);
            // }
        }
    }
}



