*****CITATIONS*****

Original code by Chris Patton (https://github.com/crispyDyne/bevy_car_demo)

Alterations made using the reference of https://bevy-cheatbook.github.io/3d/gltf.html

As well as https://bevyengine.org/examples/3D%20Rendering/load-gltf/

*****BACKGROUND*****
</br>
This is a demo of importing photogrammetry models into the Bevy game engine. Specifically, we're using a car simulation created and provided to us by Chris Patton.</br>
Doing this manually informs the team of how to design our photogrammetry pipeline, as well as a small glimmer into what our finished program is aiming for.</br>
All changes are within the "car" folder. This includes the entire "assets" subfolder, as well as some code within environment.rs.</br>
Within the code, there exists a commented out command to import a file called "pom.gltf". This worked, the issue is just that the file "pom.bin", which is needed for importing into the simulation,
is too big for GitHub. </br>


*****HOW TO RUN*****
The demo can be ran by entering this into a Linux command line or an equivalent:

cargo run --example car
