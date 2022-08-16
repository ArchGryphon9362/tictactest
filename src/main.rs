use macroquad::prelude as mq;
mod graphics;

fn init() {
    
}

fn frame(frame_count: &mut i32) {
    mq::clear_background(mq::BLACK);
    mq::draw_text(&format!("Hey, {}", frame_count)[..], 30f32, 30f32, 48f32, mq::WHITE);
    *frame_count += 1;
}

#[macroquad::main("TicTacTest")]
async fn main() {
    let mut frame_count = 0;
    graphics::event_loop(init, frame, &mut frame_count).await
}
