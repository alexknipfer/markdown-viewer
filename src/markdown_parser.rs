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
            self.handle_whitespace();
            if self.is_end_of_line() {
                break;
            } else {
                result.push_str(&self.parse_line());
                result.push_str(&self.create_break());
            }
        }

        result
    }

    fn parse_line(&mut self) -> String {
        let next_char = self.get_next_character();

        console::log_2(&"Next CHAR: ".into(), &next_char.to_string().into());

        match next_char {
            '#' => self.create_title(),
            _ => self.parse_text(),
        }
    }

    fn is_end_of_line(&self) -> bool {
        self.position >= self.input.len()
    }

    fn get_next_character(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    fn create_break(&self) -> String {
        String::from("<br />")
    }

    fn create_title(&mut self) -> String {
        let pound = self.consume_while(|c| c == '#');
        self.handle_whitespace();
        let text = self.parse_text();

        create_html_element(format!("h{}", pound.len()), text)
    }

    fn parse_text(&mut self) -> String {
        self.consume_while(|c| !is_newline(c))
    }

    fn handle_whitespace(&mut self) -> String {
        self.consume_while(char::is_whitespace)
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

fn create_html_element(tag_name: String, text: String) -> String {
    format!("<{}>{}</{}>", tag_name, text, tag_name)
}

fn is_newline(c: char) -> bool {
    c == '\n'
}
