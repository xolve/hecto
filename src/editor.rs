use log::info;
use std::{io, path::Path};
use termion::{color, event::Key};

use crate::document::{Document, Row};
use crate::terminal::{Size, Terminal};

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
                panic!("{}", error);
            }
        }
    }

    pub fn process_keypress(&mut self) -> Result<(), io::Error> {
        let pressed_key = self.terminal.read_key()?;
        match pressed_key {
            Key::Ctrl('q') => {
                self.should_quit = true;
            }

            Key::Ctrl('s') => {
                self.document.save()?;
            }

            Key::Up
            | Key::Right
            | Key::Down
            | Key::Left
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            | Key::End => self.move_cursor(pressed_key),

            Key::Char(c) => {
                if c == '\n' {
                    self.document.insert_newline(&self.cursor_position);
                    self.move_cursor(Key::Down);
                    self.move_cursor(Key::Home);
                } else {
                    self.document.insert(&self.cursor_position, c);
                    self.move_cursor(Key::Right);
                }
            }

            Key::Backspace => {
                if self.cursor_position.x == 0 && self.cursor_position.y == 0 {
                    ()
                } else {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }

            Key::Delete => {
                self.document.delete(&self.cursor_position);
            }

            _ => (),
        }

        self.scroll();

        Ok(())
    }

    fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let end = self.offset.x + self.document_viewport_size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    pub fn draw_rows(&self) {
        info!("Called draw rows");
        self.terminal.cursor_position(&Position { x: 0, y: 0 });
        let Size {
            height: m,
            width: n,
        } = self.document_viewport_size();

        for i in 0..m {
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

        let p = self.document.filename().map(|p| {
            std::fs::canonicalize(Path::new(&p))
                .unwrap()
                .as_os_str()
                .to_owned()
        });
        let status_messgae = format!("{:?}", p);
        self.set_status_message(&status_messgae);

        info!(
            "Size: {:?}. Cursor Position: {:?}. Offset: {:?}.\r",
            self.document_viewport_size(),
            self.cursor_position,
            self.offset,
        );
    }

    pub fn set_status_message(&self, message: &str) {
        let w = self.terminal.size().width as usize;
        let h = self.terminal.size().height as usize;
        self.terminal.cursor_position(&Position { x: 0, y: h });
        self.terminal.clear_current_line();
        print!("{}{}", color::Bg(color::LightCyan), message);
        if message.len() < w {
            print!(
                "{}{}",
                color::Bg(color::LightCyan),
                " ".repeat(w - message.len())
            );
        }

        print!("{}", color::Bg(termion::color::Reset));
    }

    fn refresh_screen(&mut self) {
        self.terminal.hide_cursor();
        if self.should_quit {
            self.prompt_save_before_exit();
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
        let height = self.document_viewport_size().height as usize;
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
                } else if y > 0 {
                    y -= 1;
                    x = self.document.row(y).map_or(0, |r| r.len());
                }
            }
            Key::PageUp => y = if y > height { y - height } else { 0 },
            Key::PageDown => {
                y = if y.saturating_add(height) < self.document.len() {
                    y + height
                } else {
                    self.document.len()
                }
            }
            Key::Home => x = 0,
            Key::End => {
                x = self.document.row(y).map_or(0, |r| r.len());
            }
            _ => (),
        }
        let width = self.document.row(y).map_or(0, |r| r.len());
        if x > width {
            x = width;
        }
        self.cursor_position = Position { x, y }
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.document_viewport_size().width as usize;
        let height = self.document_viewport_size().height as usize;
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

    fn document_viewport_size(&self) -> Size {
        let terminal_size = self.terminal.size();
        Size {
            width: terminal_size.width,
            height: terminal_size.height - 1,
        }
    }

    fn prompt_save_before_exit(&mut self) -> Result<(), io::Error> {
        if self.document.get_modified() {
            let answer = self.prompt("Unsaved, wanna save (y/n): ")?;
            if answer.starts_with("y") {
                self.document.save()?
            }
        }
        Ok(())
    }

    fn prompt(&self, message: &str) -> Result<String, io::Error> {
        let mut result = String::new();
        loop {
            let status_message = format!("{}{}", message, result);
            self.set_status_message(&status_message);
            self.terminal.show_cursor();
            if let Key::Char(c) = self.terminal.read_key()? {
                if c == '\n' {
                    break;
                } else if !c.is_control() {
                    result.push(c)
                }
            }
        }

        Ok(result)
    }
}
