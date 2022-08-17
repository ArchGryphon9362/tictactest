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

pub fn normalize(x: f32, y: f32, position: bool) -> (f32, f32) {
    let w = mq::screen_width();
    let h = mq::screen_height();
    let mut x_offset = 0.0;
    let mut y_offset = 0.0;
    let mut scale_x = 0.0;
    let mut scale_y = 0.0;
    if !position {
        if w / 800.0 * 600.0 <= h {
            return (w / 800.0 * x, w / 800.0 * y);
        } else {
            return (h / 600.0 * x, h / 600.0 * y);
        }
    } else if w / 800.0 * 600.0 <= h {
        y_offset = (h / 2.0) - (w / 800.0 * 600.0 / 2.0);
    } else {
        x_offset = (w / 2.0) - (h / 600.0 * 800.0 / 2.0);
    }
    if position {
        scale_x = (w / 800.0 * x) + x_offset;
        scale_y = (h / 600.0 * y) + y_offset;
    }
    (scale_x, scale_y)
}

fn unnormalize(x: f32, y: f32) -> (f32, f32) {
    panic!("graphics::unscale(scaled: f32) -> f32 not yet implemented!");
    (0.0, 0.0)
}
