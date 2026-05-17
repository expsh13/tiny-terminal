use crate::dot_pattern_for;

const FONT_WIDTH: usize = 4;
const FONT_HEIGHT: usize = 5;
const GAP: usize = 1;

pub fn draw_cells_to_pixels(cells: &Vec<Vec<Option<char>>>) -> Vec<Vec<bool>> {
    let cell_width = cells.iter().map(Vec::len).max().unwrap_or(0);
    let cell_height = cells.len();
    let pixel_width = pixel_size_for_cells(cell_width, FONT_WIDTH);
    let pixel_height = pixel_size_for_cells(cell_height, FONT_HEIGHT);
    let mut pixels = vec![vec![false; pixel_width]; pixel_height];

    for (cell_y, line) in cells.iter().enumerate() {
        for (cell_x, cell) in line.iter().enumerate() {
            if let Some(char) = cell {
                let char_dot = dot_pattern_for(*char);
                let start_x = cell_x * (FONT_WIDTH + GAP);
                let start_y = cell_y * (FONT_HEIGHT + GAP);

                for dot_y in 0..FONT_HEIGHT {
                    for dot_x in 0..FONT_WIDTH {
                        pixels[start_y + dot_y][start_x + dot_x] = char_dot[dot_y][dot_x];
                    }
                }
            }
        }
    }

    pixels
}

pub fn bool_pixels_to_rgb_buffer(pixels: &Vec<Vec<bool>>) -> Vec<u32> {
    let mut buffer = Vec::new();

    for line in pixels {
        for pixel in line {
            if *pixel {
                buffer.push(0xFFFFFF);
            } else {
                buffer.push(0x000000);
            }
        }
    }

    buffer
}

fn pixel_size_for_cells(cell_count: usize, font_size: usize) -> usize {
    if cell_count == 0 {
        0
    } else {
        cell_count * font_size + (cell_count - 1) * GAP
    }
}

#[cfg(test)]
mod tests {
    use super::{bool_pixels_to_rgb_buffer, draw_cells_to_pixels};

    #[test]
    fn draws_cells_to_pixels_with_one_pixel_gap_between_columns_and_rows() {
        let cells = vec![vec![Some('h'), Some('e')], vec![Some('w'), None]];

        let pixels = draw_cells_to_pixels(&cells);

        assert_eq!(pixels[0][0], true);
        assert_eq!(pixels[0][4], false);
        assert_eq!(pixels[0][5], true);

        assert_eq!(pixels[5][0], false);
        assert_eq!(pixels[6][0], true);
    }

    #[test]
    fn converts_bool_pixels_to_rgb_buffer() {
        let pixels = vec![vec![true, false], vec![false, true]];

        let buffer = bool_pixels_to_rgb_buffer(&pixels);

        assert_eq!(buffer, vec![0xFFFFFF, 0x000000, 0x000000, 0xFFFFFF]);
    }
}
