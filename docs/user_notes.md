# User Notes
An overview of anything that could help a user.

## Controls
- W = Go Foward
- S = Brake
- A = Turn Left
- D = Turn Right

## How to Run
- Go into the "src" folder in your command terminal of choice
- Run the command "cargo run --example car"

## How to Change Environment (For Intermediate Level Users)
- Follow the code model in src/car/src/enviroment.rs
```
commands.spawn(SceneBundle {
        scene: asset_server.load("3D_Example/Example.gltf#Scene0"),
        transform: Transform::from_xyz(-5.0, 20.0, -1.0)
        .with_scale(Vec3::new(5.0, 5.0, 5.0))
        .with_rotation(Quat::from_rotation_x(0.5 * std::f32::consts::PI)),
        ..default()
    });
```
- asset_server.load("3D_Example/Example.gltf#Scene0")
  - Replace the quote with the directory of a 3D model within the src/car/assets folder, followed by "#Scene0" without quotes
- Comment out "grid_terrain.build_meshes(&mut commands, &mut meshes, &mut materials, empty_parent);" if you want to not spawn the default environment
