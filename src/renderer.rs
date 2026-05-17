use crate::{draw_cells_to_pixels, place_chars_in_cells, read_chars};

pub fn render_text_to_pixels(input: &str) -> Vec<Vec<bool>> {
    let chars = read_chars(input);
    let (width, height) = cell_size_for_text(input);
    let cells = place_chars_in_cells(&chars, width, height);

    draw_cells_to_pixels(&cells)
}

fn cell_size_for_text(input: &str) -> (usize, usize) {
    if input.is_empty() {
        return (0, 0);
    }

    let mut width = 0;
    let mut current_width = 0;
    let mut height = 1;

    for char in input.chars() {
        if char == '\n' {
            width = width.max(current_width);
            current_width = 0;
            height += 1;
        } else {
            current_width += 1;
        }
    }

    (width.max(current_width), height)
}

#[cfg(test)]
mod tests {
    use super::render_text_to_pixels;

    #[test]
    fn renders_text_to_pixels_through_the_whole_pipeline() {
        let pixels = render_text_to_pixels("hello\nworld");

        assert_eq!(pixels.len(), 11);
        assert_eq!(pixels[0].len(), 24);

        assert_eq!(pixels[0][0], true);
        assert_eq!(pixels[0][5], true);
        assert_eq!(pixels[6][0], true);
    }
}
