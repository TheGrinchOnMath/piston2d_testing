// file containing the structs and functions used to handle the intersections and other logic



// this is the global reflection handler for now.
pub fn find_closest_mirror_reflections(
    rays: &Vec<Ray>,
    mirrors: &Vec<Mirror>,
) -> ReflectionHandlerResult {
    let mut result: ReflectionHandlerResult = ReflectionHandlerResult::empty();

    for ray in rays {
        // initialize tracking variables
        // distance will track the distance value of the closest mirror so far
        let mut distance = f64::MAX;
        // closest position is the closest position
        let mut closest_position: [f64; 2] = [f64::MAX; 2];
        let mut _mirror = Mirror::empty();

        for mirror in mirrors {
            let intersect = mirror.intersect(ray.clone());
            if intersect.success {
                let position = intersect.position;
                let current_distance = ray.distance(position);
                if current_distance < distance && current_distance > 10f64.powi(-10) {
                    // if the current intersection is the closest so far,
                    // store all the interesting info
                    distance = current_distance;
                    closest_position = position;
                    _mirror = mirror.clone();
                }
            }
        }

        if closest_position[0] < f64::MAX && closest_position[1] < f64::MAX {
            let draw_line = [
                ray.start_pos[0],
                ray.start_pos[1],
                closest_position[0],
                closest_position[1],
            ];
            result.draw_line.push(draw_line);
            // FIXME the normal vector is wrong for some reason
            let new_ray = ray.reflect(closest_position, _mirror.normal());
            result.reflected_rays.push(new_ray);
            // println!(
            //     "closest_position: {:?} ray: {:?}, mirror : {:?}, normalVector: {:?} -> {:?} ",
            //     closest_position,
            //     ray,
            //     _mirror,
            //     _mirror.normal(),
            //     new_ray
            // );

        }
    }
    //println!("\n\n\n");
    result
}
