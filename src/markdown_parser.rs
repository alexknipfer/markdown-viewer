struct MarkdownParser {
    position: usize,
    input: String,
}

impl MarkdownParser {
    fn end_of_line(&self) -> bool {
        self.position >= self.input.len()
    }

    fn starts_with(&self, &s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }

    fn next_character(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    fn read_character(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, current_char) = iter.next().unwrap();
        let (next_position, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_position;
        current_char
    }
}
