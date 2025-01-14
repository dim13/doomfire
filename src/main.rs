use fire::Fire;
use minifb::{Key, Scale, Window, WindowOptions};

mod fire;

fn main() {
    let mut f = Fire::new();
    f.seed();
    let (width, height) = f.size();
    let mut window = Window::new(
        "Doom Fire",
        width,
        height,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .expect("Cannot create the window");
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        f.update();
        window
            .update_with_buffer(&f.bytes(), width, height)
            .expect("Cannot update buffer");
    }
}
