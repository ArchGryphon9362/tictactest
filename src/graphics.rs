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
