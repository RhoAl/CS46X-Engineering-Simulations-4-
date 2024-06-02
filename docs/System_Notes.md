# Program Structure

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
  - Because of that, we haven't yet implemented collision on the front and back of the car, as additional rays would likely tank the program without further optimization. The four rays casted are at the bottom points where the tire models would normally be.   

## Enviroment Maps 
The raycast collision system allows for the user to insert any 3D model (that the engine can handle) into the simulation, and explore it without any additional work.
- The file for environment calls is found in: src/car/src/enviroment.rs
