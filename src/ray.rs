use std::ops::Mul;

use graphics::color;
use vector2d::Vector2D;

extern crate graphics;

// Ray struct. Having a Struct for Ray is not as bad as in Python,
// since the performance cost is minimal due to differing design.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub(crate) start_pos: Vector2D<f64>,
    pub(crate) vector: Vector2D<f64>,
    pub(crate) color: graphics::types::Color,
}

impl Ray {
    // get distance from Ray origin to any given point.
    pub fn distance_to_point(&self, point: Vector2D<f64>) -> f64 {
        (point - self.start_pos).length()
    }

    // generates reflection vector based on the normal of the surface to be reflected from
    pub fn reflect(&self, point: Vector2D<f64>, normal: Vector2D<f64>) -> Ray {
        // https://math.stackexchange.com/a/4325839
        let result: Vector2D<f64> = self.vector
            - normal
                .mul((Vector2D::<f64>::dot(self.vector, normal) * 2.0) / normal.length_squared());

        Ray {
            start_pos: point,
            vector: result,
            color: self.color,
        }
    }

    // generate rays radially around cursor, based on ray count.
    pub fn generate_radial(ray_count: i32, origin: Vector2D<f64>) -> Vec<Ray> {
        let mut res: Vec<Ray> = Vec::with_capacity(ray_count as usize);
        let pi = std::f64::consts::PI;

        for n in 0..ray_count {
            let angle = (n as f64 / ray_count as f64) * 2.0 * pi;

            let vector: Vector2D<f64> = Vector2D::from((angle.cos(), angle.sin()));

            // add ray to vector
            res.push(Ray {
                start_pos: origin,
                vector,
                color: graphics::color::YELLOW,
            })
        }
        res
    }
}
