use fire::Fire;
use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

mod fire;

fn main() {
    let mut window = Window::new(
        "Doom Fire",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .expect("Cannot create the window");
    window.set_target_fps(60);

    let mut f = Fire::new(WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        f.update();
        window
            .update_with_buffer(&f.bytes(), WIDTH, HEIGHT)
            .expect("err");
    }
}
