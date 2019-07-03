extern crate crossterm;

use crossterm::{Color, Crossterm};

// use the `Crossterm` to get an instance to the cursor module | demonstration.
pub fn main() {
    // Create the crossterm type to access different modules.
    let crossterm = Crossterm::new();

    // pass a reference to the current screen.
    let cursor = crossterm.cursor();
    let color = crossterm.color();
    let terminal = crossterm.terminal();
    let input = crossterm.input();
    let style = crossterm
        .style("Black text on green background")
        .with(Color::Black)
        .on(Color::Green);

    let line = input.read_line().unwrap();
    terminal.write(line).unwrap();
    terminal.write('\n').unwrap();
}
