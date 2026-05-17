pub fn read_chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::read_chars;

    #[test]
    fn reads_fixed_text_one_char_at_a_time() {
        let chars = read_chars("hello\nworld");

        assert_eq!(
            chars,
            vec!['h', 'e', 'l', 'l', 'o', '\n', 'w', 'o', 'r', 'l', 'd']
        );
    }
}
