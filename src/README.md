## CITATIONS

Original code by Chris Patton (https://github.com/crispyDyne/bevy_car_demo)


## BACKGROUND
This is a modified car simulation program, with added features and an altered collision system. The original car simulation we're using as a base was created and provided to us by Chris Patton.</br>
The specific new features are a new UI to track statics and maps, the ability to import environment models as explorable levels, and a new collision system.</br>

## TECHNICAL BACKGROUND
- The car simulation is run in Bevy, which is a rust programming language based game engine.
- The collision system uses a raycasting system to detect solid objects, allow for collision to be applied to imported 3D models without prior collision work being done. Raycasting itself is essentially another form of raytracing, both of which are a 3D modeling technique which relies on casting out a virtual ray that detects information from a larger environment.
- Pre-made environment models, aside from the default one, are made using Meshroom and Blender, using Meshroom to make a 3D model from a set of photos (photogrammetry), and then using Blender to simplify and convert the models into a compatible format.  </br>

## FUTURE WORK
- Changes to the collision system by adding more rays. This would allow for more accurate collision, but could tank performance if not carefully optimized.
- Hallway environment model. We created a second showcase environment model, but we weren't able to get in running stabely yet.
- Ray filters / working wheel models. Currently in development is a filter for our raycast system. This would allow us to pick and choose what models cause collsion. The primary goal of this was to allow us to use the car's 3D wheel models without activating the raycast. As such, the wheels are turned off by default, but can be turned on. Do be warned it causes instability issues.
- Level select. Currently changes to the environment have to be hard coded in. Eventually we'd like to code this feature to buttons on the UI.
- A greater number of physics statistics. Currently we only have speed and throttle of the car. In the future, we'd like to implement more statistics.

## FOLDER STRUCTURE
- The car folder contains the 

## HOW TO RUN
The demo can be ran by entering this into a Linux command line or an equivalent:

cargo run --example car
