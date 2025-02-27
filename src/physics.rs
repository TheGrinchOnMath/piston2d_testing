
// structs for ray and mirror




struct Mirror {
    start_pos: Vector2,
    end_pos: Vector2,
    absorption_factor: f32, // 0 = perfect mirror, 1 = nothing gets reflected
}

impl Mirror {
    fn intersect(&self, ray: &Ray) -> IntersectionResult {
        // skip intensity for now, needlessly complex
        /* function description:
        intersect self with ray using math. return the Result struct (cuz easier that way)

         */

        {
            let a1: f64 = self.start_pos.x;
            let a2: f64 = self.start_pos.y;
            let b1: f64 = self.end_pos.x;
            let b2: f64 = self.end_pos.y;
            let p1: f64 = ray.start_pos.x;
            let p2: f64 = ray.start_pos.y;
            let v1: f64 = ray.vector.x;
            let v2: f64 = ray.vector.y;

            // result, will modify if successful
            let mut result = IntersectionResult { success: false, position: Vector2::empty() };
            // common denominator
            let denominator = v2 * (b1 - a1) - v1 * (b2 - a2);
            if denominator > 0.0 {
                let m = ((b2 - a2) * (p1 - a1) - (b1 - a1) * (p2 - a2)) / denominator;
                let n = (v2 * (p1 - a1) - v2 * (p2 - a2)) / denominator;
                if 0.0 < n && 1.0 > n && m > 0.0 {
                    result.position.x = a1 + n * (b1 - a1);
                    result.position.y = a2 + n * (b2 - a2);
                    result.success = true;

                    // returns a successful collision
                    result
                } else {
                    // result that returns false
                    result
                }
            } else {
                // result that returns false
                result
            }
        }
    }

    fn normal_vector(&self) -> Vector2 {
        // return the normal vector. we are assuming all mirrors are line mirrors, and that this is fine.
        Vector2 {
            x: self.end_pos.y - self.start_pos.x,
            y: self.end_pos.x - self.start_pos.y,
        }
    }
}

struct IntersectionHandlerResult {
    success: bool,
    new_rays: Vec<Ray>,
    draw_rays: Vec<Ray>,
}

struct IntersectionResult {
    success: bool,
    position: Vector2,
}

struct ReflectionResult {
    success: bool,
    old_ray: Ray,
    new_ray: Ray,
}
impl ReflectionResult {
    fn empty() -> ReflectionResult {
        ReflectionResult {
            success: false,
            old_ray: Ray::empty(),
            new_ray: Ray::empty(),
        }
    }
}
pub(crate) struct Vector2 {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Vector2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    fn empty() -> Self {
        Self{x: f64::INFINITY, y: f64::INFINITY}
    }
    fn dot(&self, vector:&Vector2) -> f64 {
        let vector = vector.clone();
        self.x * vector.x + self.y * vector.y
    }

    fn reflect(&self, normal:&Vector2) -> Vector2 {
        let factor = 2.0 * self.dot(normal) / (normal.norm().powi(2));
        let x:f64 = self.x - normal.x * factor;
        let y:f64 = self.y - normal.y * factor;
        Vector2::new(x, y)
    }

    fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn clone(&self) -> Vector2 {
        Vector2::new(self.x.clone(), self.y.clone())
    }
    pub(crate) fn new_from_array(arr:[f64;2]) -> Vector2 {
        Vector2::new(arr[0], arr[1])
    }
}

pub(crate) struct Ray {
    pub(crate) start_pos: Vector2,
    pub(crate) vector: Vector2,
    pub(crate) intensity: f64, // below thresh: dead, above : alive (for example 0.1)
}

impl Ray {
    fn new(start_pos: Vector2, vector: Vector2, intensity: f64) -> Ray {
        Ray {
            start_pos,
            vector,
            intensity,
        }
    }

    fn empty() -> Ray {
        Ray {
            start_pos: Vector2::empty(),
            vector: Vector2::empty(),
            intensity: 0.0,
        }
    }


    fn distance(&self, position:Vector2) -> f64 {
        ((self.start_pos.x - position.x).powi(2) + (self.start_pos.y - position.y).powi(2)).sqrt()
    }

    fn reflect(&self, position: Vector2, normal:Vector2) -> Ray {
        // use vector math here (done)
        Ray {start_pos: position, vector: self.vector.reflect(&normal), intensity: 1.0}
    }
    pub(crate) fn clone(&self) -> Ray {
        Ray {
            start_pos: self.start_pos.clone(),
            vector: self.vector.clone(),
            intensity: self.intensity.clone()}
    }
}

fn find_closest_mirror(mut ray: Ray, mirrors: &Vec<Mirror>) -> ReflectionResult {
    let mut closest_distance = f64::MAX;
    let mut intersection_result = IntersectionResult { success: false, position: Vector2::empty()};
    let mut reflection_result = ReflectionResult::empty();
    let mut normal = Vector2::empty();
    let mut intensity = 1.0;
    let closest_position = Vector2::empty();

    for mirror in mirrors {
        intersection_result = mirror.intersect(&ray);
        if intersection_result.success {
            let collision = intersection_result.position;
            let distance = ray.distance(collision);
            if distance < closest_distance {
                closest_distance = distance;
                normal = mirror.normal_vector();
                if mirror.absorption_factor == 1.0 {
                    intensity = 0.0
                } else {
                    intensity = 1.0 
                }; 
            }

        }
    }
    if closest_distance < f64::MAX {
        reflection_result.success = true;
        reflection_result.new_ray = ray.reflect(closest_position, normal);
        reflection_result.new_ray.intensity = intensity;
        reflection_result.old_ray = ray.clone();

    }
    // implicit return
    reflection_result
}



pub fn intersection_handler(rays:Vec<Ray>, mirrors:Vec<Mirror>, reset:bool) -> IntersectionHandlerResult {
    let mut new_rays:Vec<Ray> = vec![];
    let mut draw_rays: Vec<Ray> = vec![];

    
        // there has to be a nicer way of iterating through rays, but eh for now
        for ray in rays {
            let reflection_result = find_closest_mirror(ray, &mirrors);
            if reflection_result.success {
                new_rays.push(reflection_result.new_ray);
                draw_rays.push(reflection_result.old_ray);
            }
        }
        IntersectionHandlerResult {
            success: true,
            new_rays,
            draw_rays,
        
    }
}

pub fn generate_rays (start: Vector2, ray_count: i32) -> Vec<Ray> {
    let mut new_rays:Vec<Ray> = vec![];
    for n in 0..ray_count {
        let angle:f64 = 2.0 * std::f64::consts::PI * n as f64 / ray_count as f64;
        let vector:Vector2 = Vector2::new(angle.cos(), angle.sin());
        new_rays.push(Ray::new(start.clone(), vector, 0.0));
    }
    new_rays
}