mod path;
mod shell;

extern crate crossterm;

use crossterm::RawScreen;
use shell::Shell;

pub fn main() {
    if let Ok(_) = RawScreen::into_raw_mode() {
        let input = crossterm::input();
        let mut shell = Shell::new(String::from("rsh>"));

        input.disable_mouse_mode().unwrap();

        let mut stdin = input.read_sync();

        shell.init();

        loop {
            let event = stdin.next();
            if let Some(input_event) = event {
                shell.process_key_event(input_event);
            }
        }
    }
}
