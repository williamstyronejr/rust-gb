use crate::graphics::window::AppWindow;

mod graphics;

fn main() {
    println!("Running Application Window!");
    let mut window = AppWindow::new(160*3, 144*3, String::from("RustBoy"));

    window.start();
}
