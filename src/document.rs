use std::usize;

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
}