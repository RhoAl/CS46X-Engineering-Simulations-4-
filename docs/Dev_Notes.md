# Developer Notes

This document covers important details about regarding the underlying systems of the car simulator. It will list out important details about how various aspects of the simulator works, as well as how to change their properties.

## Raycast Collision

Arguably the main feature added by our team. Using a Bevy raycast addon (https://crates.io/crates/bevy_mod_raycast), we're able to cast a ray that sends back location data. This information is sent back to the car's location
system to simulate collision. Doing this allows for a collision system that works indepedent of if a model has a collision system applied to it, allowing for the insertion of 3D models into the simulator without needing to
do addition collision work.
-  The main collision system the program uses can be found in src/car/src/tire.rs, specifically within the call of the point_tire_system function. The call to use the raycast library can be found at the top of
the file ("use bevy_mod_raycast::prelude::*;").
- Documentation regarding the library can be found here: https://docs.rs/bevy_mod_raycast/latest/bevy_mod_raycast/
- The are two key pieces of code that the function declares for the raycast:
  - The declaration that actually casts the ray: let hits = raycast.cast_ray(Ray3d::new(pos, dir), &default());
  - The code that pushes the raycast data into the tire contact system: contacts.push((contact, point_abs, active));
- Raycasting is notoriously resource intensive, so the casting of additional rays will negatively affect performance
  - Because of that, there hasn't yet been any implemented collision on the front and back of the car, as additional rays would likely tank the program without further optimization. The four rays casted are at the bottom points where the tire models would normally be.   

## Enviroment Maps 
The raycast collision system allows for the user to insert any 3D model (that the engine can handle) into the simulation, and explore it without any additional work.
- The file for environment calls is found in: src/car/src/enviroment.rs
- The folder for the environment models are found in src/car/assets
- Currently there are two types of enviroments: imported glTFs and models made by the pre-existing car simulator
- The current method of importing models is command spawning them in the build_environment function via commands.spawn(SceneBundle{});
- Imported models can only use 8-bit RGB textures; 16-bit RGB textures cause Bevy to crash and have to be converted to 8-bit
- This line controls the building of the non-imported meshes: grid_terrain.build_meshes(&mut commands, &mut meshes, &mut materials, empty_parent);
  - Can be commented out to prevent the non-imported model from spawning
- glTFs have a limit to how complicated they can be; a too complicated surface will cause hitching from the amount of raycast activity
- The team made glTFs were made using photogrammetry in Meshroom, and the Meshroom_Documentation file provides a guide in how they were created

## Wheel Model/Wheel Filter
Currently the wheel models on the car are set to be invisible. There is a reason for this.
- When the wheel models are spawned, they occasionaly get in the way of the rays, which can cause the car to explode
- The solution to this is using the filter option in the raycast library (https://docs.rs/bevy_mod_raycast/latest/bevy_mod_raycast/immediate/struct.RaycastSettings.html)
  - Unfortunately the filter was unable to be implemented by the end of development
  - The draft of the filter solution relied on a query (https://docs.rs/bevy/latest/bevy/prelude/struct.Query.html) to check if there was NoRaycast entity in any model; if so, then the raycast would not collect data from it
- The code that controls the wheel model can be found in src/car/src/build.rs, specifically the MeshDef section of the wheel_e spawn declaration, found in the implementation of the Wheel component.

## UI
The UI of the simulator can be found in the upper left corner. Its current features are tracking a set of physic statistics and an unimplemented map change feature.
- This uses the Bevy EGUI crate (https://crates.io/crates/bevy-inspector-egui)
- Documentation for the crate can be found here: https://docs.rs/bevy_egui/latest/bevy_egui/
- The statistics element tracks the speed and throttle of the car
- The file controling it can be found at src/car/src/ui.rs
- 
