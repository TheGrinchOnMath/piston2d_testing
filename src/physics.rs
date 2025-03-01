use piston_window::types::{Color, ColorComponent};
use rand::prelude::*;

// structures to wrap return statements that need a success and data iff success
struct IntersectResult {
    success: bool,
    position: [f64;2]
}

impl IntersectResult {
    fn empty() -> IntersectResult {
        IntersectResult {
            success: false,
            position: [f64::infinity();2]
        }
    }
}

// structures for Ray and Mirror
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub(crate) start_pos:[f64;2],
    pub(crate) vector:[f64;2],
    pub(crate) color:[ColorComponent;4],
}


impl Ray {
    // generate new ray object from input variables
    fn new(start_pos: [f64;2], vector: [f64;2], color: [ColorComponent;4]) -> Ray {
        Ray {
            start_pos,
            vector,
            color,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Mirror {
    pub(crate) start_pos: [f64;2],
    pub(crate) end_pos: [f64;2],
    pub(crate) color: [ColorComponent;4]
}

impl Mirror {
    // intersection function for a ray as input
    fn intersect() -> IntersectResult {
        

        IntersectResult::empty()
    }
}

pub fn generate_mirrors(mirror_count: i32) -> Vec<Mirror> {
    // for now the function spits out some pregenerated stuff
    let min_rand = 10;
    let max_rand = 700;

    //
    let black:[ColorComponent;4] = [0.0, 0.0, 0.0, 1.0];
    let mut mirrors:Vec<Mirror> = Vec::new();

    let mut rng = rand::rng();
    for n in 0..mirror_count {
        let start_pos = [
            rng.random_range(min_rand..=max_rand) as f64,
            rng.random_range(min_rand..=max_rand) as f64
        ];
        let end_pos = [
            rng.random_range(min_rand..=max_rand) as f64,
            rng.random_range(min_rand..=max_rand) as f64
        ];

        mirrors.push(
            Mirror {
                start_pos,
                end_pos,
                color:black
            }
        )
    }

    mirrors
}

pub fn generate_rays(ray_count: f64, start: [f64;2]) -> Vec<Ray> {
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
        let vector:[f64;2] = [angle.cos(), angle.sin()];
        // create ray obj
        let ray = Ray {
            start_pos: start,
            vector,
            color: yellow
        };
        // add ray to vec
        result.push(ray);
    }

    // return final vec
    result
}