use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo, style::SetBackgroundColor, terminal::{Clear, ClearType}, QueueableCommand
};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout
            .queue(SetBackgroundColor(crossterm::style::Color::Green))
            .unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout
            .queue(SetBackgroundColor(crossterm::style::Color::Black))
            .unwrap();
    }
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);

            }
            
        }
        
    }
    stdout.flush().unwrap();
}
