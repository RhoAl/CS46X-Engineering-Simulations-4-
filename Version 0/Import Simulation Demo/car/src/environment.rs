//Original code by Chris Patton (https://github.com/crispyDyne/bevy_car_demo)
//Alterations made using the reference of https://bevy-cheatbook.github.io/3d/gltf.html
//As well as https://bevyengine.org/examples/3D%20Rendering/load-gltf/

/*
Sources for mesh transformations:
https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html#method.from_rotation
https://bevyengine.org/examples/Transforms/transform/
https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html
*/

use std::f32::consts::PI;

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};

use grid_terrain::{
    examples::{steps, table_top, wave},
    GridTerrain,
};

pub fn build_environment(
    mut commands: Commands, asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    //Water Bottle model
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("WaterBottle/WaterBottleGLTFSeparated.gltf#Scene0"),
    //     transform: Transform::from_scale(Vec3::splat(50.0))
    //     .with_rotation(Quat::from_rotation_x(0.5 * std::f32::consts::PI)),
    //     ..default()
    // });

    // pomegranate model
    // Works, but the assets are currently too big for a GitHub upload

    commands.spawn(SceneBundle {
        scene: asset_server.load("Fruit/Fruit.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 11.0)
        .with_scale(Vec3::new(50.0, 50.0, 50.0))
        .with_rotation(Quat::from_rotation_x(0.5 * std::f32::consts::PI)),
        ..default()
    });
    
    

    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.9, 0.9, 1.0),
        brightness: 0.4,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0, // lux
            shadow_depth_bias: 0.3,
            shadow_normal_bias: 1.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 10.0),
            rotation: Quat::from_rotation_x(-PI / 4.) * Quat::from_rotation_y(-PI / 4.),

            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 1.,
            maximum_distance: 300.0,
            first_cascade_far_bound: 5.0,
            overlap_proportion: 0.3,
        }
        .into(),

        ..default()
    });

    commands.insert_resource(DirectionalLightShadowMap { size: 4 * 1024 });

    let size = 20.0; // must be the same for all grid elements

    let height = 2.;
    let table_elements = table_top(size, height);

    let height = 0.3;
    let wave_length = 4.;
    let wave_elements = wave(size, height, wave_length);

    let step_elements = steps(size, vec![0.2, 0.4, 0.6]);

    // merge the two grid terrains
    let mut elements = table_elements;
    elements.extend(wave_elements);
    elements.extend(step_elements);

    let grid_terrain = GridTerrain::new(elements, [size, size]);
    let empty_parent = commands.spawn(SpatialBundle::default()).id();

    grid_terrain.build_meshes(&mut commands, &mut meshes, &mut materials, empty_parent);
    commands.insert_resource(grid_terrain);
}
