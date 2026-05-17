pub fn place_chars_in_cells(
    chars: &Vec<char>,
    width: usize,
    height: usize,
) -> Vec<Vec<Option<char>>> {
    let mut line = 0;
    let mut row = 0;
    let mut cells = vec![vec![None; width]; height];

    for c in chars {
        if *c == '\n' {
            line += 1;
            row = 0;

            continue;
        }

        cells[line][row] = Some(*c);
        row += 1;
    }

    cells
}

#[cfg(test)]
mod tests {
    use super::place_chars_in_cells;
    use crate::read_chars;

    #[test]
    fn places_chars_into_cells_and_moves_to_next_line_on_newline() {
        let chars = read_chars("hello\nworld");
        let cells = place_chars_in_cells(&chars, 10, 2);

        assert_eq!(cells[0][0], Some('h'));
        assert_eq!(cells[0][4], Some('o'));
        assert_eq!(cells[0][5], None);
        assert_eq!(cells[1][0], Some('w'));
        assert_eq!(cells[1][4], Some('d'));
    }
}
