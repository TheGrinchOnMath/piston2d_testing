// run one set of reflection calculations

use crate::mirror::Mirror;
use crate::ray::Ray;
use vector2d::Vector2D;

#[derive(Debug, Copy, Clone)]
pub struct Segment {
    pub(crate) start_pos: Vector2D<f64>,
    pub(crate) end_pos: Vector2D<f64>,
    pub(crate) color: graphics::types::Color,
}

/*
ray: Ray currently calculating intersections
mirrors: collection of mirrors to check for intersections
e: margin of error to avoid false positives

NOTE: an alternative approach could be to calculate the intersection (or lack therof),
    between each ray, mirror, then to find the closest mirrors from the resulting dataset

    dataset would be: Vec<Result> where Result: distance, mirror ID
    each ray has a Vec<Result>
    very memory-intensive
*/
pub fn get_intersections(
    ray: Ray,
    mirrors: Vec<Mirror>,
    e: f64,
) -> Option<(Mirror, Vector2D<f64>)> {
    // distance variable, to check closest mirror.
    let mut dist = f64::INFINITY;

    // res variable, which will track the closest mirror, intersection point and input Ray.
    // TBD: is returning the Ray really necessary?
    // initialize res as None, default to no intersection, correct for present intersection.
    let mut res: Option<(Mirror, Vector2D<f64>)> = None;

    // iterate over mirrors
    for mirror in mirrors {
        // get mirror's normal vector
        // get result from intersection check function
        if let Some(p) = mirror.intersect(&ray) {
            // distance from ray to mirror in current iteration
            let current_dist = ray.distance_to_point(p);
            // distance check condition, with small error compensation value
            if (current_dist < dist) && (current_dist > e) {
                // if the current mirror is closer than previous trigger of condition,
                // update distance & reassign values to res
                dist = current_dist;
                res = Some((mirror, p));
            }
        }
    }

    // return res. Since we assumed default-failure, this is ok to do in this way.
    res
}

/*
main physics function.
current job: calculate all intersections, then reflections for all rays.
this function can then be the arbiter for concurrent approaches.

Since it handles reflections too, it has the job of outputing data necessary for:
- new rendering passes
- the next set of rays (For reflection calculations).

*/
pub fn reflection_handler(rays: Vec<Ray>, mirrors: Vec<Mirror>, e: f64) -> Vec<Ray> {
    // vec of lines to draw
    let mut new_ray_vec: Vec<Ray> = Vec::with_capacity(rays.len());

    // iterate over rays
    for n in 0..rays.len() {
        let ray = rays[n];
        // evaluate option
        // FIXME: replace the clone() call with proper borrow semantics across caller and callee
        if let Some((mirror, point)) = get_intersections(ray, mirrors.clone(), e) {
            // if we have a good intersection, reflect the ray and add it to new list
            new_ray_vec.push(ray.reflect(point, mirror.unit_vec.normal()));
        } else {
            eprintln!("no reflections found for Ray: {:?}!", ray)
        }
    }

    new_ray_vec
}

// wrapper for reflection_handler
pub fn physics(rays: Vec<Ray>, mirrors: Vec<Mirror>, e: f64) -> (Vec<Ray>, Vec<Segment>) {
    let new_rays = reflection_handler(rays.clone(), mirrors, e);

    let segments = rays
        .iter()
        .zip(new_rays.iter())
        .map(|(cur, next)| Segment {
            start_pos: cur.start_pos,
            end_pos: next.start_pos,
            color: cur.color,
        })
        .collect();
    (new_rays, segments)
}
