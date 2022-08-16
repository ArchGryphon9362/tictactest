use macroquad::prelude as mq;
mod context;
mod player;
mod graphics;

fn init(ctx: &mut context::Context) {
    ctx.set_player(0, String::from("John"));
    ctx.set_player(1, String::from("Lennon"));
}

fn frame(ctx: &mut context::Context) {
    mq::clear_background(mq::BLACK);
    mq::draw_text(&format!("Hey, {}", ctx.get_frame())[..], 30f32, 30f32, 48f32, mq::WHITE);
}

#[macroquad::main("TicTacTest")]
async fn main() {
    graphics::event_loop(init, frame).await
}
