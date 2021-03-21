use std::io::{self, Seek};
use termion::event::Key;

use crate::terminal::{Size, Terminal};
use crate::document::{Document, Row};

#[derive(Debug, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn default() -> Self {
        let filename = std::env::args().into_iter().skip(1).next();
        let document = if let Some(fname) = filename {
            Document::open(&fname).unwrap()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            cursor_position: Position::default(),
            offset: Position::default(),
            terminal: Terminal::default().expect("Unable to instantiate terminal."),
            document,
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
            }
            Key::Up
            | Key::Right
            | Key::Down
            | Key::Left
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            | Key::End => self.move_cursor(pressed_key),
            _ => (),
        }

        self.scroll();

        Ok(())
    }

    fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let end = self.offset.x + self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    pub fn draw_rows(&self) {
        self.terminal.cursor_position(&Position { x: 0, y: 0 });
        let Size { height: m, width: n } = self.terminal.size();

        for i in 0..(*m - 1) {
            self.terminal.clear_current_line();
            if let Some(row) = self.document.row(i as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && i == m / 2 {
                println!(
                    "~{}Hecto Version - {}\r",
                    " ".repeat((n / 2 - 10).into()),
                    env!("CARGO_PKG_VERSION")
                );
            } else {
                println!("~\r");
            }
        }

        let Position { x: _, y } = self.cursor_position;
        self.terminal.clear_current_line();
        print!(
            "{}Size: {:?}. Cursor Position: {:?}. Offset: {:?}. doc len: {}, line len: {}\r",
            termion::color::Fg(termion::color::LightRed),
            self.terminal.size(),
            self.cursor_position,
            self.offset,
            self.document.len(),
            self.document.row(y).map_or(0, |r| r.len())
        );
        print!("{}", termion::color::Fg(termion::color::Reset));
    }

    fn refresh_screen(&self) {
        self.terminal.hide_cursor();
        if self.should_quit {
            self.terminal.clear_screen();
            println!("Goodbye");
        } else {
            self.draw_rows();
            let cursor_position = Position {
                x: self.cursor_position.x - self.offset.x,
                y: self.cursor_position.y - self.offset.y,
            };
            self.terminal.cursor_position(&cursor_position);
        }
        self.terminal.show_cursor();
        self.terminal.flush();
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.terminal.size().height as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Right => {
                let width = self.document.row(y).map_or(0, |r| r.len());
                if x < width {
                    x = x.saturating_add(1);
                } else if y < self.document.len() {
                    y += 1;
                    x = 0
                }
            }
            Key::Down => {
                if y < self.document.len() {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0{
                    y -= 1;
                    x = self.document.row(y).map_or(0, |r| r.len());
                }
            },
            Key::PageUp => {
                y = if y > height {
                    y - height
                } else {
                    0
                }
            },
            Key::PageDown => {
                y = if y.saturating_add(height) < self.document.len() {
                    y + height
                } else {
                    self.document.len()
                }
            },
            Key::Home => x = 0,
            Key::End => {
                x = self.document.row(y).map_or(0, |r| r.len());
            },
            _ => (),
        }
        let width = self.document.row(y).map_or(0, |r| r.len());
        if x > width {
            x = width;
        }
        self.cursor_position = Position { x, y }
    }

    fn scroll(&mut self) {
        let Position { x, y} = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = (self.terminal.size().height as usize);//.saturating_sub(1);
        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x > offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }
}
