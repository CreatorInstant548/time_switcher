//! Positions of the cards in a file: RON gives no span for a parsed value, so a small scanner
//! finds where each element of the root list starts. Best effort, only for error messages:
//! it never fails and never changes what is loaded.

/// `(line, column)`, 1-based, of the first token of each element of the root `[...]` list.
pub(crate) fn element_starts(text: &str) -> Vec<(u32, u32)> {
    let mut s = Scanner {
        chars: text.chars().collect(),
        pos: 0,
        line: 1,
        col: 1,
    };
    let mut out = Vec::new();
    let mut depth: u32 = 0;
    let mut want = false;
    while let Some(c) = s.peek(0) {
        let at = (s.line, s.col);
        match c {
            c if c.is_whitespace() => s.bump(),
            '/' if s.peek(1) == Some('/') => s.skip_line(),
            '/' if s.peek(1) == Some('*') => s.skip_block_comment(),
            '#' => s.skip_attribute(),
            ',' => {
                want = depth == 1;
                s.bump();
            }
            ']' | ')' | '}' => {
                depth = depth.saturating_sub(1);
                want = false;
                s.bump();
            }
            _ => {
                if want && depth == 1 {
                    out.push(at);
                }
                want = false;
                match c {
                    '[' | '(' | '{' => {
                        want = depth == 0 && c == '[';
                        depth = depth.saturating_add(1);
                        s.bump();
                    }
                    '"' => s.skip_string(0),
                    'r' => s.skip_raw_string_or_ident(),
                    '\'' => s.skip_char(),
                    _ => s.bump(),
                }
            }
        }
    }
    out
}

struct Scanner {
    chars: Vec<char>,
    pos: usize,
    line: u32,
    col: u32,
}

impl Scanner {
    fn peek(&self, ahead: usize) -> Option<char> {
        self.chars.get(self.pos.saturating_add(ahead)).copied()
    }

    fn bump(&mut self) {
        if let Some(c) = self.peek(0) {
            self.pos = self.pos.saturating_add(1);
            if c == '\n' {
                self.line = self.line.saturating_add(1);
                self.col = 1;
            } else {
                self.col = self.col.saturating_add(1);
            }
        }
    }

    fn skip_line(&mut self) {
        while self.peek(0).is_some_and(|c| c != '\n') {
            self.bump();
        }
    }

    fn skip_block_comment(&mut self) {
        let mut nesting: u32 = 0;
        while let Some(c) = self.peek(0) {
            if c == '/' && self.peek(1) == Some('*') {
                nesting = nesting.saturating_add(1);
                self.bump();
            } else if c == '*' && self.peek(1) == Some('/') {
                nesting = nesting.saturating_sub(1);
                self.bump();
                if nesting == 0 {
                    self.bump();
                    return;
                }
            }
            self.bump();
        }
    }

    /// `#![enable(...)]`: skips up to the closing `]`.
    fn skip_attribute(&mut self) {
        let mut nesting: u32 = 0;
        while let Some(c) = self.peek(0) {
            self.bump();
            match c {
                '[' => nesting = nesting.saturating_add(1),
                ']' => {
                    nesting = nesting.saturating_sub(1);
                    if nesting == 0 {
                        return;
                    }
                }
                _ => {}
            }
        }
    }

    /// At an opening `"`, after `hashes` `#` already consumed: skips to the closing quote
    /// (followed by `hashes` `#`). Without hashes, a backslash escapes the next character.
    fn skip_string(&mut self, hashes: usize) {
        self.bump();
        while let Some(c) = self.peek(0) {
            if c == '\\' && hashes == 0 {
                self.bump();
            } else if c == '"' && (1..=hashes).all(|i| self.peek(i) == Some('#')) {
                self.bump();
                for _ in 0..hashes {
                    self.bump();
                }
                return;
            }
            self.bump();
        }
    }

    /// At an `r`: a raw string `r#"..."#` or a plain character of an identifier.
    fn skip_raw_string_or_ident(&mut self) {
        let mut hashes = 0;
        while self.peek(1usize.saturating_add(hashes)) == Some('#') {
            hashes = hashes.saturating_add(1);
        }
        if self.peek(1usize.saturating_add(hashes)) == Some('"') {
            self.bump();
            for _ in 0..hashes {
                self.bump();
            }
            self.skip_string(hashes);
        } else {
            self.bump();
        }
    }

    fn skip_char(&mut self) {
        self.bump();
        if self.peek(0) == Some('\\') {
            self.bump();
        }
        self.bump();
        if self.peek(0) == Some('\'') {
            self.bump();
        }
    }
}
