use piston_window::types::{Color, ColorComponent};
use rand::prelude::*;
use std::cell::Ref;

// structures to wrap return statements that need a success and data iff success
#[derive(Debug, Copy, Clone)]
struct IntersectResult {
    success: bool,
    position: [f64; 2],
}

impl IntersectResult {
    fn empty() -> IntersectResult {
        IntersectResult {
            success: false,
            position: [f64::INFINITY; 2],
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ReflectResult {
    success: bool,
    start_pos: [f64; 2],
    end_pos: [f64; 2],
}

impl ReflectResult {
    fn empty() -> ReflectResult {
        ReflectResult {
            success: false,
            start_pos: [f64::INFINITY; 2],
            end_pos: [f64::INFINITY; 2],
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReflectionHandlerResult {
    pub draw_line: Vec<[f64; 4]>,
    pub reflected_rays: Vec<Ray>,
}

impl ReflectionHandlerResult {
    fn empty() -> ReflectionHandlerResult {
        ReflectionHandlerResult {
            draw_line: vec![],
            reflected_rays: vec![],
        }
    }
}

// structures for Ray and Mirror
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub(crate) start_pos: [f64; 2],
    pub(crate) vector: [f64; 2],
    pub(crate) color: [ColorComponent; 4],
}

impl Ray {
    // generate new ray object from input variables
    fn new(start_pos: [f64; 2], vector: [f64; 2], color: [ColorComponent; 4]) -> Ray {
        Ray {
            start_pos,
            vector,
            color,
        }
    }

    // find distance between origin and given point
    fn distance(&self, point: [f64; 2]) -> f64 {
        // by creating a vector from both points, then apply pythagora's theorem to the x & y coords

        ((self.start_pos[0] - point[0]).powi(2) + (self.start_pos[1] - point[1]).powi(2)).sqrt()
    }

    fn reflect(&self, position: [f64; 2], normal: [f64; 2]) -> Ray {
        // split the vector reflection equation into multiple parts to make things readable
        let norm_squared: f64 = normal[0] * normal[0] + normal[1] * normal[1];
        let dot_product: f64 = self.vector[0] * normal[0] + self.vector[1] * normal[1];

        let result: [f64; 2] = [
            self.vector[0] - 2f64 * dot_product / norm_squared * normal[0],
            self.vector[1] - 2f64 * dot_product / norm_squared * normal[1],
        ];

        // create new object ray and return it. for now colors are preserved.
        Ray {
            start_pos: position,
            vector: result,
            color: self.color,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Mirror {
    pub(crate) start_pos: [f64; 2],
    pub(crate) end_pos: [f64; 2],
    pub(crate) color: [ColorComponent; 4],
}

impl Mirror {
    // create an empty mirror object with color black
    fn empty() -> Mirror {
        Mirror {
            start_pos: [f64::INFINITY; 2],
            end_pos: [f64::INFINITY; 2],
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
    // intersection function for a ray as input
    fn intersect(&self, ray: Ray) -> IntersectResult {
        // initialize equation variables.
        // end point 1 of mirror segment
        let a1: f64 = self.start_pos[0];
        let a2: f64 = self.start_pos[1];
        // end point 2 of mirror segment
        let b1: f64 = self.end_pos[0];
        let b2: f64 = self.end_pos[1];
        // ray start position (origin)
        let p1: f64 = ray.start_pos[0];
        let p2: f64 = ray.start_pos[1];
        // ray direction vector
        let v1: f64 = ray.vector[0];
        let v2: f64 = ray.vector[1];

        // result, will modify if successful
        let mut result = IntersectResult::empty();
        // calculate common denominator for the fractions
        let denominator = v2 * (b1 - a1) - v1 * (b2 - a2);

        // this checks for the situation where the denominator is zero
        // (and therefore the lines are parralel)
        if denominator != 0.0 {
            // calculate the factors for both vectors (in parametric line representation)
            let m = ((b2 - a2) * (p1 - a1) - (b1 - a1) * (p2 - a2)) / denominator;
            let n = (v2 * (p1 - a1) - v1 * (p2 - a2)) / denominator;
            // if these factors are within the definition bounds for segment and ray
            if 0.0 < n && 1.0 > n && m > 0.0 {
                // set the result to intersection
                result.position[0] = a1 + n * (b1 - a1);
                result.position[1] = a2 + n * (b2 - a2);
                result.success = true;
            }
        }
        // since the result will be a fail unless specified above, we are clear to return here only
        result
    }

    fn normal(&self) -> [f64; 2] {
        [
            self.end_pos[0] - self.end_pos[1],
            self.start_pos[1] - self.start_pos[0],
        ]
    }
}

pub fn generate_mirrors(mirror_count: i32) -> Vec<Mirror> {
    // for now the function spits out some pregenerated stuff
    let min_rand = 10;
    let max_rand = 1100;

    //
    let _black: [ColorComponent; 4] = [0.0, 0.0, 0.0, 1.0];
    let white: [ColorComponent; 4] = [1.0, 1.0, 1.0, 1.0];
    let mut mirrors: Vec<Mirror> = Vec::new();

    let mut rng = rand::rng();
    for n in 0..mirror_count {
        let start_pos = [
            rng.random_range(min_rand..=max_rand) as f64,
            rng.random_range(min_rand..=max_rand) as f64,
        ];
        let end_pos = [
            rng.random_range(min_rand..=max_rand) as f64,
            rng.random_range(min_rand..=max_rand) as f64,
        ];

        mirrors.push(Mirror {
            start_pos,
            end_pos,
            color: white,
        })
    }

    mirrors
}

pub fn generate_rays(ray_count: f64, start: [f64; 2]) -> Vec<Ray> {
    // color of the ray. for now yellow
    let yellow = [1.0, 1.0, 0.0, 1.0];

    // vec that will contain the rays
    let mut result: Vec<Ray> = Vec::new();

    // init pi, to make code more readable
    let pi = std::f64::consts::PI;

    // iter through the ray count to make rays
    for mut n in 0..ray_count as i32 {
        // angle: use fraction of 2pi
        let m = n as f64;
        let angle = 2f64 * pi * m / ray_count;
        // vector is simplified trigonometry
        let vector: [f64; 2] = [angle.cos(), angle.sin()];
        // create ray obj
        let ray = Ray {
            start_pos: start,
            vector,
            color: yellow,
        };
        // add ray to vec
        result.push(ray);
    }

    // return final vec
    result
}

// check intersections for every mirror. for 1 ray.
// for now we return a simple result, since we are not doing reflections just yet
/*pub fn find_closest_mirror_no_reflections(ray: &Vec<Ray>, mirrors: &Vec<Mirror>) -> [[f64;2];2] {
    // initialize tracking variables
    // distance will track the distance value of the closest mirror so far
    let mut distance = f64::MAX;
    // closest position is the closest position
    let mut closest_position:[f64;2] = [f64::MAX;2];


    for mirror in mirrors {
        let intersect = mirror.intersect(ray);
        if intersect.success {
            let position = intersect.position;
            let current_distance = ray.distance(position);
            if current_distance < distance {
                // if the current intersection is the closest so far,
                // store all the interesting info
                distance = current_distance;
                closest_position = position;
            }
        }
    }

    // check if any intersection has been found
    // TODO add condition that makes the ray stop at edge of screen and no longer reflect if this happens
    // currently we are doing this in main. FIXME add proper stuff
    /*if (closest_position[0] < f64::MAX) && (closest_position[1] < f64::MAX) {
        //     result.success = true;
        //     result.start_pos = ray.start_pos;
        //     result.end_pos = closest_position;
        // }
        // result

    }*/
    [ray.start_pos, closest_position]
}
*/
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
            println!(
                "closest_position: {:?} ray: {:?}, mirror : {:?}, normalVector: {:?} -> {:?} ", 
                closest_position, 
                ray, 
                _mirror,
                _mirror.normal(),
                new_ray 
            );
        
        }
    }
    println!("\n\n\n");
    result
}
