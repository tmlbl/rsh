use crossterm::{InputEvent, KeyEvent};

pub struct Shell {
    prompt: String,
    term: crossterm::Terminal,
    line_buf: Vec<char>,
}

impl Shell {
    pub fn new(prompt: String) -> Shell {
        Shell {
            prompt,
            term: crossterm::terminal(),
            line_buf: Vec::new(),
        }
    }

    fn update_line(&mut self) {
        // Re-set the current line
        self.term.write('\r').unwrap();
        self.term.write(self.prompt.clone()).unwrap();
        self.term.write(" ").unwrap();

        let line: String = self.line_buf.clone().into_iter().collect();
        self.term.write(line).unwrap();
    }

    fn is_end_of_line(&mut self) -> bool {
        self.line_buf.last().unwrap_or(&'x') == &'\n'
    }

    pub fn process_key_event(&mut self, input_event: InputEvent) {
        match input_event {
            InputEvent::Keyboard(k) => match k {
                KeyEvent::Ctrl(c) => match c {
                    'c' => std::process::exit(1),
                    _ => (),
                },
                KeyEvent::Char(c) => self.line_buf.push(c),
                KeyEvent::Backspace => {
                    self.line_buf.pop();
                },
                _ => (),
            },
            _ => (),
        }
        self.update_line();
        if self.is_end_of_line() {
            println!("End of the line...");
        }
    }
}
