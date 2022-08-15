mod graphics;
use fermium::video::SDL_WINDOWPOS_CENTERED;

fn main() {
    let window = graphics::Window::new(String::from("TicTacTest"), 800, 600, SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED);
    println!("Hello, world!");
}
