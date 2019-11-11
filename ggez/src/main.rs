//#![allow(unused)]
//#![allow(dead_code)]

use ggez::*;
use ggez::event::EventHandler;
use std::time::Duration;

struct State {
    delta_time: Duration
}

impl State {
    fn new() -> Self {
        Self {
            delta_time: Duration::new(0, 0)
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.delta_time = timer::delta(ctx);
        return Ok(());
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        println!("Hello ggez!!, Delta time is: {}", self.delta_time.subsec_nanos());
        return Ok(());
    }
}

fn main() {
    let state = &mut State { delta_time: Duration::new(0, 0) };
    let conf = conf::Conf::new();
    let (ref mut context, ref mut evt_loop) = ContextBuilder::new("pickup_stick", "Nor Mohd Sobri").conf(conf).build().unwrap();
    event::run(context, evt_loop, state).unwrap();
}
