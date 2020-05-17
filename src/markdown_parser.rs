use web_sys::console;

struct MarkdownParser {
    position: usize,
    input: String,
}

pub fn parse_input(source: String) -> String {
    MarkdownParser {
        position: 0,
        input: source,
    }
    .parse_lines()
}

impl MarkdownParser {
    fn parse_lines(&mut self) -> String {
        let mut result = String::new();

        loop {
            if self.is_end_of_line() {
                break;
            } else {
                result.push_str(&self.parse_line());
            }
        }

        result
    }

    fn parse_line(&mut self) -> String {
        match self.get_next_character() {
            '#' => String::new(),
            _ => self.parse_text(),
        }
    }

    fn is_end_of_line(&self) -> bool {
        self.position >= self.input.len()
    }

    fn get_next_character(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    fn parse_text(&mut self) -> String {
        self.consume_while(|c| !is_newline(c))
    }

    fn consume_while<F>(&mut self, func: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.is_end_of_line() && func(self.get_next_character()) {
            result.push(self.process_char());
        }

        result
    }

    fn process_char(&mut self) -> char {
        let mut iter = self.input[self.position..].char_indices();
        let (_, current_char) = iter.next().unwrap();
        let (next_position, _) = iter.next().unwrap_or((1, ' '));
        console::log_2(
            &"Current character: ".into(),
            &current_char.to_string().into(),
        );
        self.position += next_position;
        current_char
    }
}

fn is_newline(c: char) -> bool {
    c == '\n'
}
