use macroquad::prelude as mq;
use crate::context::Context;

pub async fn event_loop(init: fn(&mut Context), frame: fn(&mut Context)) {
    let mut context = Context::new();
    init(&mut context);
    loop {
        frame(&mut context);
        context.frame();
        mq::next_frame().await
    }
}

pub fn scale(unscaled: f32) -> f32 {
    println!("{}x{}", mq::screen_width(), mq::screen_height());
    0f32
}

fn unscale(scaled: f32) -> f32 {
    panic!("graphics::unscale(scaled: f32) -> f32 not yet implemented!");
    0f32
}
