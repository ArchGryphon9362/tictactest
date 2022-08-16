use macroquad::prelude as mq;

pub async fn event_loop(init: fn(), frame: fn(&mut i32), frame_count: &mut i32) {
    init();
    loop {
        frame(frame_count);
        mq::next_frame().await
    }
}
