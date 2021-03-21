use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::editor::Position;

#[derive(Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1 - 1,
            },
            _stdout: io::stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn cursor_position(&self, position: &Position) {
        let x = position.x.saturating_add(1) as u16;
        let y = position.y.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn clear_screen(&self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    }

    pub fn read_key(&self) -> Result<Key, io::Error> {
        io::stdin().keys().next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "Some err occured reading input key.")
        })?
    }

    pub fn flush(&self) -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn hide_cursor(&self) {
        print!("{}", termion::cursor::Hide);
    }

    pub fn show_cursor(&self) {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line(&self) {
        print!("{}", termion::clear::CurrentLine);
    }
}
