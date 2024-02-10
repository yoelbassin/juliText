use std::io::Write;

use crate::Position;
use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
    dirty: bool,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in content.lines() {
            // \n \r etc are stripped
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            filename: Some(filename.to_string()),
            dirty: false,
        })
    }
    pub fn save(&mut self) -> Result<(), std::io::Error> {
        if let Some(filename) = &self.filename {
            let mut file = std::fs::File::create(filename)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn insert(&mut self, at: &Position, c: char) {
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if at.y < self.len() {
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }
    }
    fn insert_newline(&mut self, at: &Position) {
        if at.y >= self.len() {
            self.rows.push(Row::default());
        } else {
            let new_row = self.rows[at.y].split(at.x);
            self.rows.insert(at.y + 1, new_row);
        }
    }
    pub fn delete(&mut self, at: &Position) {
        if at.y >= self.len() {
            return;
        }
        self.dirty = true;
        if at.x == self.rows[at.y].len() && at.y < self.len() - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
        }
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
