use std::usize;

use crate::editor::Position;

#[derive(Default)]
pub struct Row {
    data: String,
    len: usize,
}

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
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

    pub fn insert (&mut self, xpos: usize, c: char) {
        self.data.insert(xpos, c);
        self.len = self.data.chars().count();
    }
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(filename)?;
        let rows = contents.lines().into_iter().map(|l| Row::from(l)).collect::<Vec<Row>>();
        Ok(Self {
            rows
        })
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

    pub fn insert(&mut self, pos: &Position, c: char) {
        if pos.y == self.len() {
        let row = Row::default();
            self.rows.push(row);
        }
        let row = self.rows.get_mut(pos.y).unwrap();
        row.insert(pos.x, c);
    }
}