use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;


// This is all that is needed to raycast into the world! You can also use the normal, non-debug
// version (raycast.cast_ray) when you don't need to visualize the ray or intersections.
fn raycast(
    mut raycast: Raycast, 
    mut gizmos: Gizmos, 
    time: Res<Time>, 
    mut query_joints: Query<&mut Joint>, 
    mut tire_query: Query<&mut PointTire>,
    
) {
    for mut tire in tire_query.iter_mut() {
        if let Ok([mut joint, parent]) =
            query_joints.get_many_mut([tire.joint_entity(), tire.joint_parent()])
        {
            let xp0 = parent.x.inverse(); // spatial transform from the parent joint to absolute coordinates
            let center_abs = xp0.transform_point(Vector::zeros()); // center of the tire in absolute coordinates

            let pos = Vec3::new((center_abs[0] as f32), (center_abs[1] as f32), (center_abs[2] as f32) - 0.1);
            let dir = Vec3::new(0.0, 0.0, -1.0);
            let hits = raycast.debug_cast_ray(Ray3d::new(pos, dir), &default(), &mut gizmos,);

            for &(entity, ref intersection_data) in hits {
                // Access the IntersectionData for each hit
                println!("{:?}", intersection_data);
            }
        }
    }
}