mod graphics;

fn main() {
    let window = graphics::Window::new(String::from("TicTacTest"), 800, 600);
    println!("Hello, world!");
    println!("{}", window.get_name());
    println!("{}", window.get_width());
    println!("{}", window.get_height());
}
