use std::fmt::Display;

struct TextParser<'t> {
    text: &'t str,
}

impl<'t> TextParser<'t> {
    fn new(text: &'t str) -> Self {
        TextParser { text }
    }

    // all text before space
    fn first_word(&self) -> &str {
        if let Some(idx) = self.text.find(' ') {
            &self.text[..idx]
        } else {
            self.text
        }
    }

    fn between(&self, start: char, end: char) -> Option<&str> {
        // let start_idx = self.text.find(start);
        // let end_idx = self.text.find(end);
        if let Some(start_idx) = self.text.find(start) {
            if let Some(end_idx) = self.text.find(end) {
                return Some(&self.text[start_idx + 1..end_idx]);
            }
        }
        return None;
    }
}

fn main() {
    let mut tp = TextParser::new("hello there random stranger");

    assert_eq!(tp.first_word(), "hello");
    assert_eq!(tp.between('h', 'o'), Some("ell"));

    tp.text = "hello (world) foo";
    assert_eq!(tp.between('(', ')'), Some("world"));
}
