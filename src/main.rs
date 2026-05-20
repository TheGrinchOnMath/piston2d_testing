mod mirror;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate vector2d;

use glutin_window::GlutinWindow as Window;
use graphics::math::Matrix2d;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use vector2d::Vector2D;

pub struct App {
    gl: GlGraphics,
    reflection_counter: i32,
    mirrors: Vec<mirror::Mirror>,
    mouse_pos: Vector2D<f64>,
    rays: Vec<mirror::Ray>,
    clear_window: bool,
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

        // draw call
        self.gl
            .draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {
                let transform = c.transform;

                // clear screen?
                if self.clear_window {
                    clear(color::BLACK, gl);

                    line_from_to(color::WHITE, 3.0, [1.0, 1.0], [100.0, 100.0], transform, gl);
                }
            })
    }

    // main update function.
    fn update(&mut self, args: &UpdateArgs) {}
}

fn main() {
    // change to OpenGL::V2_1 if no workey
    let gl = OpenGL::V3_2;

    // create Glutin Window
    let mut window: Window = WindowSettings::new("test123", [200; 2])
        .graphics_api(gl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // create a new App instance.
    let mut app = App {
        gl: GlGraphics::new(gl),
        reflection_counter: 0,
        mirrors: vec![mirror::Mirror {}],
        mouse_pos: vector2d::Vector2D::<f64> { x: 0f64, y: 0f64 },
        rays: vec![mirror::Ray {}],
        clear_window: true,
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
    }
}
