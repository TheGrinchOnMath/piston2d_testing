mod physics;

use std::process::exit;
use piston::EventLoop;
use piston_window::*;
use rand::prelude::*;

fn main() {
    // configure piston window
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("piston2d", [800; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);

    // setup random float provider
    let mut rng = rand::rng();

    // configure "global" variables (cursor pos, etc)
    let mut mouse_pos = [window.size().width as f64, window.size().height as f64];

    // generate mirrors. these only need to generate once, so are outside of the while loop
    let mirrors = physics::generate_mirrors(10);

    // this counts the computed reflections (to be able to fix limits)
    let mut reflection_counter = 0;
    const MAX_REFLECTIONS: i32 = 3;

    // this lets us reset the sim
    let mut reset = true;

    // this lets us set the amount of rays
    const RAY_COUNT:f64 = 5f64;

    // keep track of all objects to draw

    let mut rays:Vec<physics::Ray> = physics::generate_rays(RAY_COUNT, mouse_pos);

    let mut clear_once = true;
    
    
    // main draw loop, call draw() here
    while let Some(e) = window.next() {
        // process keyboard events
        if let Some(ref args) = e.press_args() {
            use piston_window::Button::Keyboard;

            if *args == Keyboard(Key::Escape) {
                exit(0);
            }
            if *args == Keyboard(Key::Space) {
                reset = true;
            }

        }

        // process mouse events
        if let Some(ref args) = e.mouse_cursor_args() {
            // update mouse pos every frame
            mouse_pos = *args;
        }

        // render
         window.draw_2d(&e, |c:Context, g:&mut G2d, _| {

             let white = [1.0; 4];
             let black = [0.0, 0.0, 0.0, 1.0];
             // check for reset, if so regen the rays
             if clear_once {
             clear(black, g);
                 clear_once = false;
             }
                 
             let mut line_coords:Vec<[f64;4]> = Vec::new();
             if !reset && reflection_counter <= MAX_REFLECTIONS {
                 let result:physics::ReflectionHandlerResult = physics::find_closest_mirror_reflections(&rays, &mirrors);
                 // extract new rays
                 rays = result.reflected_rays;

                 line_coords = result.draw_line;

             } else if reset {
                 rays = physics::generate_rays(RAY_COUNT, mouse_pos);
                 println!("resetting...\n\n");
                 //clear screen
                 clear(black, g);
                 reflection_counter = 0;
                 reset = false;
             }
             



             reflection_counter += 1;


             // line(black,2f64 ,[100f64, 100f64, 200f64, 200f64], c.transform, g);




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

             let color = [
                 rng.random_range(0f32..=1f32),
                 rng.random_range(0f32..=1f32),
                 rng.random_range(0f32..=1f32),
                 1.0
             ];

             for coords in line_coords {
                 let line_info = [
                     coords[0],
                     coords[1],
                     coords[2],
                     coords[3]
                 ];
                 line(
                     color,
                     1.0,
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
/*fn render() {

}

fn handle_inputs() {

}*/
/*
// FIXME remove this with something proper
fn handle_ray_stuff(rays:&Vec<physics::Ray>, mirrors:&Vec<physics::Mirror>) -> Vec<[f64;4]>{
    let mut result = Vec::new();
    for ray in rays {
        let _ray = ray.clone();
        let intersection = physics::find_closest_mirror_no_reflections(_ray, mirrors);
        //println!("intersection: {:?}", intersection);
        // since f64:MAX means no position was found we can compare to that
        if (intersection[1][0] < f64::MAX) && (intersection[1][1] < f64::MAX) {
            //println!("intersection success");
            let line_coords = [intersection[0][0], intersection[0][1], intersection[1][0], intersection[1][1]];
            result.push(line_coords);
        } else {
            let line_coords = [_ray.start_pos[0], _ray.start_pos[1], _ray.start_pos[0] + 10_000f64*_ray.vector[0], _ray.start_pos[1] + 10_000f64 * ray.vector[1]];
            result.push(line_coords);
        }
    }
    //println!("result: {:?}", result);
    result
}*/