use std::thread;
use std::time::Duration;

use ggez::{winit::event_loop, *};

fn main() {
    println!("hello world!");

    let fps_cap: f64 = 30.;
    let state: State = State {
        dt: Duration::new(0, 0),
        avg_fps: vec![fps_cap],
        fps_cap,
    };
    let c: conf::Conf = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello ggez!", "Gronk")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}

struct State {
    dt: Duration,
    avg_fps: Vec<f64>,
    fps_cap: f64,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let dt = self.dt.as_secs_f64();

        
        let fps = 1. / dt;

        self.avg_fps.push(fps);

        let avg_fps_notvec:f64 = self.avg_fps.iter().sum::<f64>() / self.avg_fps.len() as f64;

        if fps > self.fps_cap {
            let time = Duration::from_secs_f64(-1. /fps + 1./self.fps_cap);
            ggez::timer::sleep(time);
        }


        println!("hello ggez!\tfps = {} \t\t avg fps: {}", fps, avg_fps_notvec);
        Ok(())
    }
}
