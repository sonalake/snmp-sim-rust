pub mod component;

use std::cell::RefCell;
use std::io::BufRead;

use crate::parser::{ParserError, SnmpDataItems};
use crate::property::PropertyParser;

/// Reader returning `SnmpData` object from a `BufRead`.
pub struct SnmpDataParser<B> {
    line_parser: RefCell<PropertyParser<B>>,
}

impl<B: BufRead> SnmpDataParser<B> {
    /// Return a new `SnmpDataParser` from a `Reader`.
    pub fn new(reader: B) -> SnmpDataParser<B> {
        let line_parser = PropertyParser::from_reader(reader);

        SnmpDataParser {
            line_parser: RefCell::new(line_parser),
        }
    }
}

impl<B: BufRead> Iterator for SnmpDataParser<B> {
    type Item = Result<component::SnmpData, ParserError>;

    fn next(&mut self) -> Option<Result<component::SnmpData, ParserError>> {
        let mut snmp_data = component::SnmpData::new();
        let result = match snmp_data.parse(&self.line_parser) {
            Ok(_) if snmp_data.values.is_empty() => return None,
            Ok(_) => Ok(snmp_data),
            Err(err) => Err(err),
        };

        Some(result)
    }
}
