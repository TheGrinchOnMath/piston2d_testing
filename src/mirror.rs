//use pathlib::Path;
use crate::io;
use crate::ray::Ray;
use graphics::color;
use rand::prelude::*;
use std::ops::{Mul, Range};
use vector2d::Vector2D;

extern crate graphics;

#[derive(Debug, Copy, Clone)]
pub struct Mirror {
    pub(crate) start_pos: Vector2D<f64>,
    pub(crate) end_pos: Vector2D<f64>,
    pub(crate) unit_vec: Vector2D<f64>,
    pub(crate) color: graphics::types::Color,
}

// mirror implementations. This is where the Mirror class methods go.
impl Mirror {
    // intersection function. Returns an Option based on the outcome of the intersection check
    pub fn intersect(&self, ray: &Ray) -> Option<Vector2D<f64>> {
        // initialize equation variables.
        // end point 1 of mirror segment
        let a: Vector2D<f64> = self.start_pos;
        // end point .y of mirror segment
        let b: Vector2D<f64> = self.end_pos;
        // ray start position (origin)
        let p: Vector2D<f64> = ray.start_pos;
        // ray direction vector
        let v: Vector2D<f64> = ray.vector;

        let denominator = v.y * (b.x - a.x) - v.x * (b.y - a.y);

        if denominator == 0.0 {
            None
        } else {
            // calculate the factors for both vectors (in parametric line representation)
            let m = ((b.y - a.y) * (p.x - a.x) - (b.x - a.x) * (p.y - a.y)) / denominator;
            let n = (v.y * (p.x - a.x) - v.x * (p.y - a.y)) / denominator;
            // if these factors are within the definition bounds for segment and ray
            if 0.0 < n && 1.0 > n && m > 0.0 {
                // set the result to intersection
                // this line works since vector arithmetic is implemented here
                let result: Vector2D<f64> = a + (b - a).mul(n);

                Some(result)
            } else {
                None
            }
        }
    }
}

// generate collection of random mirrors.
pub fn generate_rand(count: i32, window_dimensions: Vector2D<f64>) -> Vec<Mirror> {
    let mut mirrors: Vec<Mirror> = Vec::with_capacity(count as usize);

    // get Seed
    let mut rng = rand::rng();

    // create ranges
    let range_x = 10.0..window_dimensions.x;
    let range_y = 10.0..window_dimensions.y;

    // create all mirrors
    for _ in 0..count {
        // start pos "vector" (Point)
        let start_pos = Vector2D {
            x: rng.random_range::<f64, Range<f64>>(range_x.clone()),
            y: rng.random_range::<f64, Range<f64>>(range_y.clone()),
        };
        // end pos "vector" (Point)
        let end_pos = Vector2D {
            x: rng.random_range::<f64, Range<f64>>(range_x.clone()),
            y: rng.random_range::<f64, Range<f64>>(range_y.clone()),
        };
        // create the unit vec using a vec created from the ends of the mirror
        // normalize() returns the unit vector corresponding to the one just created.
        let unit_vec = (&end_pos - &start_pos).normalise();
        let color = graphics::color::WHITE;
        mirrors.push(Mirror {
            start_pos,
            end_pos,
            unit_vec,
            color,
        })
    }

    mirrors
}

pub fn generate_json(path: &str) -> Vec<Mirror> {
    // for now this only accepts pixel dimensions
    let mirrors: Vec<Mirror>;

    let json_data = io::read_json(path);

    let coord_format = "pixels";

    let mirrors_from_json = &json_data.mirrors;

    mirrors = mirrors_from_json
        .iter()
        .map(|mirror| Mirror {
            start_pos: Vector2D {
                x: mirror.start_pos[0],
                y: mirror.start_pos[1],
            },
            end_pos: Vector2D {
                x: mirror.end_pos[0],
                y: mirror.end_pos[1],
            },
            unit_vec: Vector2D {
                x: mirror.end_pos[0] - mirror.start_pos[0],
                y: mirror.end_pos[1] - mirror.start_pos[1],
            },
            color: color::WHITE,
        })
        .collect();

    mirrors
}

pub fn generate_window_edges() {}

// FIXME: ADD NEW STRUCT FIELDS
/* pub fn generate_json(path: &Path) -> Vec<Mirror> {
    // for now this only accepts pixel dimensions
    let mut mirrors: Vec<Mirror> = Vec::new();

    let json_data = io::read_json(path);

    let coord_format = "pixels";

    let mirrors_from_json = &json_data.mirrors;

    mirrors = mirrors_from_json
        .iter()
        .map(|mirror| Mirror {
            start_pos: [mirror.start_pos[0], mirror.start_pos[1]],
            end_pos: [mirror.end_pos[0], mirror.end_pos[1]],
            color: [1.0; 4],
        })
        .collect();

    mirrors
}
 */
