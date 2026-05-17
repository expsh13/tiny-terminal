pub fn dot_pattern_for(char: char) -> [[bool; 4]; 5] {
    match char {
        'h' => [
            [true, false, false, true],
            [true, false, false, true],
            [true, true, true, true],
            [true, false, false, true],
            [true, false, false, true],
        ],
        'e' => [
            [true, true, true, true],
            [true, false, false, false],
            [true, true, true, false],
            [true, false, false, false],
            [true, true, true, true],
        ],
        'l' => [
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
            [true, true, true, true],
        ],
        'o' => [
            [false, true, true, false],
            [true, false, false, true],
            [true, false, false, true],
            [true, false, false, true],
            [false, true, true, false],
        ],
        'w' => [
            [true, false, false, true],
            [true, false, false, true],
            [true, false, false, true],
            [true, true, true, true],
            [false, true, true, false],
        ],
        'r' => [
            [true, true, true, false],
            [true, false, false, true],
            [true, true, true, false],
            [true, false, true, false],
            [true, false, false, true],
        ],
        'd' => [
            [true, true, true, false],
            [true, false, false, true],
            [true, false, false, true],
            [true, false, false, true],
            [true, true, true, false],
        ],
        _ => panic!("not supported"),
    }
}

#[cfg(test)]
mod tests {
    use super::dot_pattern_for;

    #[test]
    fn converts_h_to_dot_font_pattern() {
        let pattern = dot_pattern_for('h');

        assert_eq!(
            pattern,
            [
                [true, false, false, true],
                [true, false, false, true],
                [true, true, true, true],
                [true, false, false, true],
                [true, false, false, true],
            ]
        );
    }

    #[test]
    fn supports_chars_used_in_hello_world() {
        for c in ['h', 'e', 'l', 'o', 'w', 'r', 'd'] {
            dot_pattern_for(c);
        }
    }
}
