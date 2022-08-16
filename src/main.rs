use macroquad::prelude as mq;
mod context;
mod player;
mod graphics;

fn print_player_name(ctx: &context::Context, player: usize, x: f32, y: f32, fsize: f32, color: mq::Color) {
    match ctx.get_player(player) {
        Some(player_obj) => {
            mq::draw_text(&format!("Player {}: {}", player, player_obj.get_name())[..], x, y, fsize, color);
        }
        None => {
            mq::draw_text(&format!("No Player {}", player)[..], x, y, fsize, color);
        }
    }
}

fn init(ctx: &mut context::Context) {
    ctx.set_player(0, "John".to_owned());
    ctx.set_player(1, "Lennon".to_owned());
}

fn frame(ctx: &mut context::Context) {
    mq::clear_background(mq::BLACK);
    print_player_name(ctx, 0, 30f32, 48f32, 48f32, mq::WHITE);
    print_player_name(ctx, 1, 30f32, 96f32, 48f32, mq::WHITE);
    graphics::scale(0f32);
}

fn config_window() -> mq::Conf {
    mq::Conf {
        window_title: "TicTacTest".to_owned(),
        high_dpi: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(config_window)]
async fn main() {
    graphics::event_loop(init, frame).await
}
