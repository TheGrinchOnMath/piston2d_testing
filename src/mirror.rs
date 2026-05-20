use vector2d::Vector2D;

extern crate graphics;

// mirror struct. equivalent of the Mirror class in Python.
#[derive(Debug, Copy, Clone)]

pub struct Ray {
    start_pos: Vector2D<f64>,
    vector: Vector2D<f64>,
}

pub struct Mirror {
    pub(crate) start_pos: Vector2D<f64>,
    pub(crate) end_pos: Vector2D<f64>,
    pub(crate) unit_vec: Vector2D<f64>,
    pub(crate) color: graphics::types::Color,
}

// mirror implementations. This is where the Mirror class methods go.
impl Mirror {
    // intersection function. Returns an Option based on the outcome of the intersection check
    fn intersect(&self, ray: Ray) -> Option<Vector2D<f64>> {
        // initialize equation variables.
        // end point 1 of mirror segment
        let a: Vector2D<f64> = self.start_pos;
        // end point .y of mirror segment
        let b: Vector2D<f64> = self.end_pos;
        // ray start position (origin)
        let p: Vector2D<f64> = ray.start_pos;
        // ray direction vector
        let v: Vector2D<f64> = ray.vector;

        // initiate empty result
        let mut result: Vector2D<f64> = Vector2D {
            x: f64::INFINITY,
            y: f64::INFINITY,
        };

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
                result.x = a.x + n * (b.x - a.x);
                result.y = a.y + n * (b.y - a.y);
                Some(result)
            } else {
                None
            }
        }
    }
}
