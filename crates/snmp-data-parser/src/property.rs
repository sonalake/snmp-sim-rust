use crate::VALUE_DELIMITER;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::BufRead;
use std::iter::Iterator;

use crate::line::{Line, LineReader};

#[derive(Debug, thiserror::Error)]
pub enum PropertyError {
    #[error("Line {}: Missing name.", line)]
    MissingName { line: usize },
    #[error("Line {}: Missing value.", line)]
    MissingValue { line: usize },
}

/// SNMP data property.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Property {
    /// Property name.
    pub name: String,

    /// Property value.
    pub value: String,
}

impl Property {
    /// Return a new `Property` object.
    pub fn new() -> Property {
        Property {
            name: String::new(),
            value: String::new(),
        }
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}\nvalue: {:?}", self.name, self.value)
    }
}

/// Take a `LineReader` and return a list of `Property`.
#[derive(Debug, Clone)]
pub struct PropertyParser<B> {
    line_reader: LineReader<B>,
}

impl<B: BufRead> PropertyParser<B> {
    /// Return a new `PropertyParser` from a `LineReader`.
    pub fn new(line_reader: LineReader<B>) -> PropertyParser<B> {
        PropertyParser { line_reader }
    }

    /// Return a new `PropertyParser` from a `Reader`.
    pub fn from_reader(reader: B) -> PropertyParser<B> {
        let line_reader = LineReader::new(reader);

        PropertyParser { line_reader }
    }

    fn parse(&self, line: Line) -> Result<Property, PropertyError> {
        let mut property = Property::new();
        let mut to_parse = line.as_str();

        // Parse name.
        let value_index = to_parse.find(VALUE_DELIMITER).unwrap_or(usize::max_value());
        let end_name_index = if value_index != usize::max_value() && value_index != 0 {
            value_index
        } else {
            return Err(PropertyError::MissingName { line: line.number() });
        };

        {
            let split = to_parse.split_at(end_name_index);
            property.name = split.0.trim_matches(' ').to_string();
            to_parse = split.1;
        }

        // Parse value
        to_parse = to_parse.trim_start_matches(VALUE_DELIMITER);
        if to_parse.is_empty() {
            return Err(PropertyError::MissingValue { line: line.number() });
        } else {
            property.value = to_parse.trim_matches(' ').to_string();
        }

        Ok(property)
    }
}

impl<B: BufRead> Iterator for PropertyParser<B> {
    type Item = Result<Property, PropertyError>;

    fn next(&mut self) -> Option<Result<Property, PropertyError>> {
        self.line_reader.next().map(|line| self.parse(line))
    }
}
