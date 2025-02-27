

mod render;
mod physics;
mod io;

extern crate piston_window;

use std::process::exit;
use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("shapes", [512; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);
    // init mouse pos by setting it to the middle of the window
    let mut mouse_pos:[f64;2] = [window.size().width, window.size().height];
    // init reset variable. this will be used to trigger the rays be deleted
    let mut reset = true;
    // init rays, mirrors vectors. they contain the rays and mirrors.
    // mirrors are loaded from a json file, rays are generated from cursor position
    let mut rays:Vec<physics::Ray>;
    let mirrors = io::read_json("./assets/file.json");
    let ray_count = 11;
    // this might be stupid, and it probably is, but hey! 
    // big vec of all the generated rays after each reset. might make a lot of memory be used.
    let mut big_ray_buffer:Vec<physics::Ray> = vec![];
    // generate very first set of rays
    rays = physics::generate_rays(physics::Vector2::new_from_array(mouse_pos), ray_count);

    // main event loop
    while let Some(e) = window.next() {
        // add code here to update mouse position
        if reset {
            rays = physics::generate_rays(physics::Vector2::new_from_array(mouse_pos), ray_count);
            big_ray_buffer.clear();
            big_ray_buffer = rays.iter().map(|ray| ray.clone()).collect();
        }

        if let Some(ref args) = e.press_args() {
            use piston_window::Button::Keyboard;

            if *args == Keyboard(Key::Escape) {
                // kill the process
                exit(0)
            }
            if *args == Keyboard(Key::Space) {
                reset = true;
                // do something here that reloads the keys
            }

        }
        if let Some(ref args) = e.mouse_cursor_args() {
            // update mouse pos
            mouse_pos = *args;
        }
        // draw stuff
        window.draw_2d(&e, |c: Context, g: &mut G2d, _| {
            clear([1.0; 4], g);
            let black = [0.0, 0.0, 0.0, 1.0];

            // draw rays
            for ray in &rays {
                let line_coords = [ray.start_pos.x, ray.start_pos.y, ray.start_pos.x + ray.vector.x, ray.start_pos.y + ray.vector.y];
                let color = black;
                let thickness = 3.0;
                line(black, thickness, line_coords, c.transform, g);
            }
            // draw mirrors
                
            
            
            line(black, 2.0, [100.0, 200.0, 300.0, 400.0], c.transform, g);

            /*

            for i in 0..5 {

                let c = c.trans(0.0, i as f64 * 100.0);
                let red = [1.0, 0.0, 0.0, 1.0];
                let rect = math::margin_rectangle([20.0, 20.0, 60.0, 60.0], i as f64 * 5.0);
                //rectangle(red, rect, c.transform, g);
                //Rectangle::new_border(black, 2.0).draw(rect, &c.draw_state, c.transform, g);
                let green = [0.0, 1.0, 0.0, 1.0];
                let h = 60.0 * (1.0 - i as f64 / 5.0);
                let rect = [120.0, 50.0 - h / 2.0, 60.0, h];
                //ellipse(green, rect, c.transform, g);
                //Ellipse::new_border(black, 2.0).draw(rect, &c.draw_state, c.transform, g);
                let blue = [0.0, 0.0, 1.0, 1.0];
                // circle_arc(blue, 10.0, 0.0, f64::_360() - i as f64 * 1.2, [230.0, 30.0, 40.0, 40.0], c.transform, g);
                let orange = [1.0, 0.5, 0.0, 1.0];
                line(orange, 5.0, [320.0 + i as f64 * 15.0, 20.0, 380.0 - i as f64 * 15.0, 80.0],
                     c.transform, g);
                let magenta = [1.0, 0.0, 0.5, 1.0];
                // polygon(magenta, &[[420.0, 20.0], [480.0, 20.0], [480.0 - i as f64 * 15.0, 80.0] ], c.transform, g);
            }*/
        });
    }
}
