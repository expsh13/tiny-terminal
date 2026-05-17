use minifb::{Key, Scale, Window, WindowOptions};
use tiny_terminal::{bool_pixels_to_rgb_buffer, render_text_to_pixels};

fn main() -> Result<(), minifb::Error> {
    let pixels = render_text_to_pixels("hello\nworld");
    let height = pixels.len();
    let width = pixels[0].len();
    let buffer = bool_pixels_to_rgb_buffer(&pixels);

    let mut window = Window::new(
        "tiny-terminal",
        width,
        height,
        WindowOptions {
            scale: Scale::X8,
            ..WindowOptions::default()
        },
    )?;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height)?;
    }

    Ok(())
}
