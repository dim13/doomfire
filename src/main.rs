use fire::{Fire, BLACK, WHITE};
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use toggle::Toggle;

mod fire;
mod toggle;

fn main() {
    let mut f = Fire::new();
    let mut t = Toggle::new(BLACK, WHITE);
    f.seed(WHITE);
    let (width, height) = f.size();
    let mut window = Window::new(
        "Doom Fire",
        width,
        height,
        WindowOptions {
            scale: Scale::X2,
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Cannot create the window");
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            f.seed(t.toggle());
        }
        f.update();
        window
            .update_with_buffer(&f.bytes(), width, height)
            .expect("Cannot update buffer");
    }
}
