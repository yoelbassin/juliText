use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        Self {
            string: s.to_string(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(end, self.string.len());
        let start = std::cmp::min(start, end);
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str("  ");
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }
    pub fn insert(&mut self, at: usize, c: char) {
        if at == self.len() {
            self.string.push(c);
        } else {
            let beggining: String = self.string[..].graphemes(true).take(at).collect();
            let remainder: String = self.string[..].graphemes(true).skip(at).collect();
            self.string = format!("{}{}{}", beggining, c, remainder);
        }
    }
    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        } else {
            let beggining: String = self.string[..].graphemes(true).take(at).collect();
            let remainder: String = self.string[..].graphemes(true).skip(at + 1).collect();
            self.string = format!("{}{}", beggining, remainder);
        }
    }
    pub fn append(&mut self, other: &Self) {
        self.string = format!("{}{}", self.string, other.string);
    }
    pub fn split(&mut self, at: usize) -> Self {
        let beggining: String = self.string[..].graphemes(true).take(at).collect();
        let remainder: String = self.string[..].graphemes(true).skip(at).collect();
        self.string = beggining;
        Self::from(&remainder[..])
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
    pub fn len(&self) -> usize {
        // We can optimize this by storing the length of the string in the struct
        self.string[..].graphemes(true).count()
    }
    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}
