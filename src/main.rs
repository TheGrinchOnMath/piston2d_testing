mod io;
mod mirror;
mod physics;
mod ray;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate vector2d;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::*;
use vector2d::Vector2D;

/*
Note on the structure of rendered_generations:
the structure of rendered_generations is actually (pseudocode):
Vec <
    Vec <
        start_point
        end_point
        color
        >
    >

*/

const MAX_REFLECTIONS: i32 = 10;
const RAY_COUNT: i32 = 10000;

pub struct App {
    gl: GlGraphics,
    mirrors: Vec<mirror::Mirror>,
    mouse_pos: Vector2D<f64>,
    clear_window: bool,
    window_size: Size,
    randomize_mirrors: bool,
    reset_rays: bool,
    generations: Vec<Vec<ray::Ray>>,
    rendered_generations: Vec<Vec<physics::Segment>>,
    reflections: i32,
}

impl App {
    // main render function
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // get window dimensions
        let window_size: Vector2D<f64> = Vector2D {
            x: args.window_size[0],
            y: args.window_size[1],
        };
        self.window_size = args.window_size.into();

        // draw call
        self.gl
            .draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
                let transform = c.transform;

                clear(color::BLACK, gl);

                // render mirrors
                for mirror in self.mirrors.clone() {
                    let p1 = [mirror.start_pos.x, mirror.start_pos.y];
                    let p2 = [mirror.end_pos.x, mirror.end_pos.y];
                    line_from_to(mirror.color, 1.0, p1, p2, transform, gl);
                }

                // render rays
                for segment_gen in self.rendered_generations.clone() {
                    for segment in segment_gen {
                        let p1 = [segment.start_pos.x, segment.start_pos.y];
                        let p2 = [segment.end_pos.x, segment.end_pos.y];
                        line_from_to(segment.color, 0.02, p1, p2, transform, gl);
                    }
                }
            })
    }

    // main update function.
    fn update(&mut self, args: &UpdateArgs) {
        // randomize mirrors, using generate_rand function.
        if self.randomize_mirrors {
            self.randomize_mirrors = false;
            self.mirrors = mirror::generate_rand(
                10,
                Vector2D {
                    x: self.window_size.width,
                    y: self.window_size.height,
                },
            );
        }

        if self.reset_rays {
            self.reset_rays = false;
            // reset reflection count
            self.reflections = 0;
            // reset generations
            self.generations = vec![ray::Ray::generate_radial(RAY_COUNT, self.mouse_pos)];
            self.rendered_generations = Vec::new();
        } else if self.reflections >= MAX_REFLECTIONS {
            return;
        } else {
            self.reflections += 1;
            // get current iteration
            let rays = self.generations.last().unwrap().clone();

            // calculate next iteration, get current generation's segments to be rendered
            let (next_gen, segments) = physics::physics(rays, self.mirrors.clone(), 0.00001);
            self.generations.push(next_gen);
            self.rendered_generations.push(segments); // create new generation.
        }
    }
}

/* messy entrypoint function. currently contains logic that shouldn't live there.
Currently:
- runs initialization code
- has some state variables
- contains event loop


 */
fn main() {
    // change to OpenGL::V2_1 if no workey
    let gl = OpenGL::V3_2;

    // create Glutin Window
    let mut window: PistonWindow<GlutinWindow> = WindowSettings::new("test123", [1920, 1080])
        .graphics_api(gl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let window_size = window.window.draw_size();

    // create a new App instance.
    let mut app = App {
        gl: GlGraphics::new(gl),
        mirrors: mirror::generate_json("assets/mirrors.json"),
        mouse_pos: vector2d::Vector2D::<f64> { x: 0f64, y: 0f64 },
        clear_window: true,
        window_size,
        randomize_mirrors: false,
        reset_rays: true,
        generations: vec![ray::Ray::generate_radial(
            11,
            Vector2D {
                x: window_size.width / 2.0,
                y: window_size.height / 2.0,
            },
        )],
        rendered_generations: Vec::new(),
        reflections: 0,
    };

    let mut events = Events::new(EventSettings::new());
    // main event loop
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.press_args() {
            use piston_window::Button::Keyboard;

            if args == Keyboard(Key::Return) {
                //app.randomize_mirrors = true;
                app.reset_rays = true;
            }
            if args == Keyboard(Key::Space) {
                app.reset_rays = true;
            }
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.mouse_pos = Vector2D::from(args);
        }
    }
}
