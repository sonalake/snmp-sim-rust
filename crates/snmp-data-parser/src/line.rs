use std::fmt;
use std::io::BufRead;
use std::iter::Iterator;

/// An unfolded raw line.
///
/// Its inner is only a raw line from the file. No parsing or checking have
/// been made yet.
#[derive(Debug, Clone, Default)]
pub struct Line {
    inner: String,
    number: usize,
}

impl Line {
    /// Return a new `Line` object.
    pub fn new(line: String, line_number: usize) -> Line {
        Line {
            inner: line,
            number: line_number,
        }
    }

    /// Return a `&str`
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    /// Return the line number.
    pub fn number(&self) -> usize {
        self.number
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {}: {}", self.number, self.inner)
    }
}

/// A trait generic for implementing line reading use crate::by `PropertyParser`.
pub trait LineRead {
    /// Return the next line unwrapped and formated.
    fn next_line(&mut self) -> Option<Line>;
}

#[derive(Debug, Clone, Default)]
/// Take a `BufRead` and return the unfolded `Line`.
pub struct LineReader<B> {
    reader: B,
    saved: Option<String>,
    number: usize,
}

impl<B: BufRead> LineReader<B> {
    /// Return a new `LineReader` from a `Reader`.
    pub fn new(reader: B) -> LineReader<B> {
        LineReader {
            reader,
            saved: None,
            number: 0,
        }
    }
}

impl<B: BufRead> LineRead for LineReader<B> {
    fn next_line(&mut self) -> Option<Line> {
        let mut next_line = String::new();
        let mut line_number: usize = 0;

        if let Some(start) = self.saved.take() {
            // If during the last iteration a new line have been saved, start with.
            next_line.push_str(start.as_str());
            self.number += 1;
            line_number = self.number;
        } else {
            // This is the first iteration, next_start isn't been filled yet.
            for line in self.reader.by_ref().lines() {
                let line = line.unwrap();
                self.number += 1;

                if !line.is_empty() {
                    next_line = line.trim_end().to_string();
                    line_number = self.number;
                    break;
                }
            }
        }

        for line in self.reader.by_ref().lines() {
            let line = line.unwrap();

            if line.is_empty() {
                self.number += 1;
            } else if line.starts_with(' ') || line.starts_with('\t') {
                // This is a multi-lines attribute.

                // Remove the whitespace characters and join with the current line.
                next_line.push_str(line.trim());
                self.number += 1;
            } else {
                // This is a new attribute so it need to be saved it for
                // the next iteration.
                self.saved = Some(line.trim().to_string());
                break;
            }
        }

        if next_line.is_empty() {
            None
        } else {
            Some(Line::new(next_line, line_number))
        }
    }
}

impl<B: BufRead> Iterator for LineReader<B> {
    type Item = Line;

    fn next(&mut self) -> Option<Line> {
        self.next_line()
    }
}
