mod physics;

use std::process::exit;
use piston::EventLoop;
use piston_window::*;

fn main() {
    // configure piston window
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston2d", [800; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);

    // configure "global" variables (cursor pos, etc)
    let mut mouse_pos = [window.size().width as f64, window.size().height as f64];

    // generate mirrors. these only need to generate once, so are outside of the while loop
    let mirrors = physics::generate_mirrors(5);

    // main draw loop, call draw() here
    while let Some(e) = window.next() {
        // process keyboard events
        if let Some(ref args) = e.press_args() {
            use piston_window::Button::Keyboard;

            if *args == Keyboard(Key::Escape) {
                exit(0);
            }
            // if *args == Keyboard(Key::Space) {
            //
            // }

        }

        // process mouse events
        if let Some(ref args) = e.mouse_cursor_args() {
            // update mouse pos every frame
            mouse_pos = *args;
        }

        // render
         window.draw_2d(&e, |c:Context, g:&mut G2d, _| {


             //clear screen
             let background_color = [1.0; 4];
             let black = [0.0, 0.0, 0.0, 1.0];

             clear(background_color, g);
             line(black,2f64 ,[100f64, 100f64, 200f64, 200f64], c.transform, g);


             // attempt drawing rays around the cursor?
             let rays = physics::generate_rays(11.0, mouse_pos);

             // process reflections:
             let line_coords = handle_ray_stuff(&rays, &mirrors);

             // iterate over ray vec
             /*for ray in rays {
                 // create array for draw
                 let draw_line = [
                     ray.start_pos[0],
                     ray.start_pos[1],
                     ray.start_pos[0] + ray.vector[0] * 10_000f64,
                     ray.start_pos[1] + ray.vector[1] * 10_000f64
                 ];
                 let color = ray.color;
                 // draw ray
                 line(color, 2.0, draw_line, c.transform, g);
             }*/

             for coords in line_coords {
                 let line_info = [
                     coords[0],
                     coords[1],
                     coords[2],
                     coords[3]
                 ];
                 line(
                     [0.0, 1.0, 0.0, 1.0],
                     2.0,
                     line_info,
                     c.transform,
                     g
                 );
             }

             // iterate over mirror vec
             for mirror in mirrors.clone() {
                 let draw_line = [
                     mirror.start_pos[0],
                     mirror.start_pos[1],
                     mirror.end_pos[0],
                     mirror.end_pos[1]
                 ];
                 let color = mirror.color;
                 line(color, 3.0, draw_line, c.transform, g);
             }
        });
    }

}

// use this function to simplify draw calls. maybe pass the draw args in and get em out?
fn render() {

}

fn handle_inputs() {

}

// FIXME remove this with something proper
fn handle_ray_stuff(rays:&Vec<physics::Ray>, mirrors:&Vec<physics::Mirror>) -> Vec<[f64;4]>{
    let mut result = Vec::new();
    for ray in rays {
        let _ray = ray.clone();
        let intersection = physics::find_closest_mirror(_ray, mirrors);
        //println!("intersection: {:?}", intersection);
        // since f64:MAX means no position was found we can compare to that
        if (intersection[1][0] < f64::MAX) && (intersection[1][1] < f64::MAX) {
            println!("intersection success");
            let line_coords = [intersection[0][0], intersection[0][1], intersection[1][0], intersection[1][1]];
            result.push(line_coords);
        } else {
            let line_coords = [_ray.start_pos[0], _ray.start_pos[1], _ray.start_pos[0] + 10_000f64*_ray.vector[0], _ray.start_pos[1] + 10_000f64 * ray.vector[1]];
            result.push(line_coords);
        }
    }
    //println!("result: {:?}", result);
    result
}