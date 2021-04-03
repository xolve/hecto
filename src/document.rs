use crate::editor::Position;
use std::io::{self, Write};
use std::{fs, path::Path};

#[derive(Default)]
pub struct Row {
    data: String,
    len: usize,
}

#[derive(Default)]
pub struct Document {
    filename: Option<String>,
    rows: Vec<Row>,
    is_modified: bool,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Row {
        Self {
            data: String::from(slice),
            len: slice.chars().count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(self.data.len(), end);
        let start = std::cmp::min(start, end);
        self.data.get(start..end).unwrap_or_default().to_string()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, xpos: usize, c: char) {
        self.data.insert(xpos, c);
        self.update_len();
    }

    pub fn delete(&mut self, xpos: usize) {
        let before = self.data.chars().take(xpos);
        let after = self.data.chars().skip(xpos + 1);
        self.data = before.chain(after).collect();
        self.update_len();
    }

    pub fn append(&mut self, other: &Self) {
        self.data.push_str(&other.data);
        self.update_len();
    }

    fn update_len(&mut self) {
        self.len = self.data.chars().count();
    }
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let path = Path::new(filename);

        if path.exists() {
            let contents = std::fs::read_to_string(filename)?;
            let rows = contents
                .lines()
                .into_iter()
                .map(|l| Row::from(l))
                .collect::<Vec<Row>>();
            Ok(Self {
                filename: Some(filename.to_owned()),
                rows,
                is_modified: false,
            })
        } else {
            Ok(Self {
                filename: Some(filename.to_owned()),
                rows: Vec::new(),
                is_modified: false,
            })
        }
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn insert_newline(&mut self, pos: &Position) {
        let up_down = self.rows.get(pos.y).map(|row| {
            let (up, down) = row.data.split_at(pos.x);
            println!("->{} {}<-", up, down);
            (Row::from(up), Row::from(down))
        });

        up_down.map(|(up, down)| {
            self.rows.remove(pos.y);
            self.rows.insert(pos.y, up);
            self.rows.insert(pos.y.saturating_add(1), down);
        });
        self.is_modified = true;
    }

    pub fn insert(&mut self, pos: &Position, c: char) {
        if c == '\n' {
            if pos.y == self.len() {
                let row = Row::default();
                self.rows.push(row);
            } else {
            }
        }
        let row = self.rows.get_mut(pos.y).unwrap();
        row.insert(pos.x, c);
        self.is_modified = true;
    }

    pub fn delete(&mut self, pos: &Position) {
        let row_len_opt = self.rows.get(pos.y).map(|r| r.len);
        if let Some(row_len) = row_len_opt {
            if row_len <= pos.x {
                let next_row = if self.rows.get(pos.y + 1).is_some() {
                    self.rows.remove(pos.y + 1)
                } else {
                    Row::default()
                };
                let row = self.rows.get_mut(pos.y).unwrap();
                row.append(&next_row);
            } else {
                let row = self.rows.get_mut(pos.y).unwrap();
                row.delete(pos.x);
            }
        }
        self.is_modified = true;
    }

    pub fn save(&mut self) -> io::Result<()> {
        if let Some(filename) = &self.filename {
            let mut fp = fs::File::create(filename)?;
            for row in &self.rows {
                fp.write_all(row.data.as_bytes())?;
                fp.write_all(b"\n")?;
            }
        };
        self.is_modified = false;
        Ok(())
    }

    pub fn get_modified(&mut self) -> bool {
        self.is_modified
    }

    pub fn filename(&self) -> Option<&String> {
        self.filename.as_ref()
    }
}
