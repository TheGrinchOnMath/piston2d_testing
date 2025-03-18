use std::thread;

use ggez::{winit::event_loop, *};


fn main() {
    println!("hello world!");

    let state = State {dt: std::time::Duration::new(0, 0)};

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello ggez!", "Gronk")
    .default_conf(c)
    .build()
    .unwrap();

    event::run(ctx, event_loop, state);
}

struct State{
    dt: std::time::Duration,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        
        let dt = self.dt.as_secs_f64();

        let wait_time = 1./30. - dt;

        if wait_time > 0. {
            thread::sleep(std::time::Duration::from_secs_f64(wait_time));
        }

        let fps = 1. / dt;
        
        println!("hello ggez!\tfps = {}",fps);
        Ok(())
    }
  }