use std::io;
use termion::event::Key;

use crate::terminal::{Size, Terminal};

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            cursor_position: Position {
                x: 0,
                y: 0,
            },
            terminal: Terminal::default().expect("Unable to instantiate terminal."),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                panic!(error);
            }
        }
    }

    pub fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = self.terminal.read_key()?;
        match pressed_key {
            Key::Ctrl('q') => {
                self.should_quit = true;
            },
            Key::Up | Key::Right | Key::Down | Key::Left | Key::PageUp | Key::PageDown | Key::Home | Key::End => self.move_cursor(pressed_key),
            _ => (),
        }

        Ok(())
    }

    pub fn draw_rows(&self) {
        self.terminal.cursor_position(&Position { x: 0, y: 0 });
        let Size { row: m, col: n,} = self.terminal.size();
        println!("Size: {:?}. Position: {:?}.\r", self.terminal.size(), self.cursor_position);

        for i in 1..(*m - 1) {
            self.terminal.clear_current_line();
            if i == m / 2 {
                println!("~{}Hecto Version - {}\r", " ".repeat((n / 2 - 10).into()), env!("CARGO_PKG_VERSION"));
            } else {
                println!("{}\r", i);
            }
        }
        print!("~");
    }

    fn refresh_screen(&self) {
        self.terminal.hide_cursor();
        if self.should_quit {
            self.terminal.clear_screen();
            println!("Goodbye");
        } else {
            self.draw_rows();
            self.terminal.cursor_position(&self.cursor_position);
        }
        self.terminal.show_cursor();
        self.terminal.flush();
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Right => {
                x = std::cmp::min((self.terminal.size().col - 1) as usize, x.saturating_add(1));
            },
            Key::Down => {
		y = std::cmp::min((self.terminal.size().row - 1) as usize,  y.saturating_add(1));
	    },
	    Key::Left => x = x.saturating_sub(1),
	    Key::PageUp => y = 0,
	    Key::PageDown => y = (self.terminal.size().row - 1) as usize,
	    Key::Home => x = 0,
	    Key::End => x = (self.terminal.size() .col - 1) as usize,
	    _ => (),
	}
	self.cursor_position = Position { x, y }
    }
}
