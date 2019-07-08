use crossterm::{InputEvent, KeyEvent};
use std::process::Command;

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

    pub fn init(&self) {
        self.term.clear(crossterm::ClearType::All).unwrap();
        self.update_line();
    }

    fn line_buf_as_string(&self) -> String {
        self.line_buf.clone().into_iter().collect()
    }

    fn update_line(&self) {
        // Re-set the current line
        self.term.write('\r').unwrap();
        self.term.write(self.prompt.clone()).unwrap();
        self.term.write(" ").unwrap();

        self.term.write(self.line_buf_as_string()).unwrap();
    }

    fn is_end_of_line(&mut self) -> bool {
        self.line_buf.last().unwrap_or(&'x') == &'\n'
    }

    fn write_string(&self, s: String) {
        self.term.write('\r').unwrap();
        for line in s.split('\n') {
            if line.is_empty() {
                return;
            }
            self.term.write(line).unwrap();
            self.term.write("\r\n").unwrap();
        }
    }

    fn execute_line(&mut self) {
        let line = self.line_buf_as_string();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mut cmd = Command::new(tokens.first().unwrap());
        if tokens.len() > 1 {
            for (i, token) in tokens.iter().enumerate() {
                if i != 0 {
                    cmd.arg(token);
                }
            }
        }
        match cmd.output() {
            Ok(out) => {
                self.write_string(String::from_utf8(out.stdout).unwrap());
                self.write_string(String::from_utf8(out.stderr).unwrap());
            }
            Err(e) => {
                self.term.write(e).unwrap();
            }
        }
        self.line_buf.clear();
        self.update_line();
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
                }
                _ => (),
            },
            _ => (),
        }
        self.update_line();
        if self.is_end_of_line() {
            self.execute_line();
        }
    }
}
