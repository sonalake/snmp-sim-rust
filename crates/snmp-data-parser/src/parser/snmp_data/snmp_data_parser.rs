use crate::parser::snmp_data::SnmpData;
use crate::parser::ModifierExtractor;
use std::cell::RefCell;
use std::io::BufRead;

use crate::parser::{ParserError, SnmpDataItems};
use crate::property::PropertyParser;

/// Reader returning `SnmpData` object from a `BufRead`.
pub struct SnmpDataParser<B, Extractor> {
    line_parser: RefCell<PropertyParser<B>>,
    extractor: Extractor,
}

impl<B: BufRead, Extractor: ModifierExtractor> SnmpDataParser<B, Extractor> {
    /// Return a new `SnmpDataParser` from a `Reader`.
    pub fn new(reader: B, extractor: Extractor) -> Self {
        let line_parser = PropertyParser::from_reader(reader);

        SnmpDataParser {
            line_parser: RefCell::new(line_parser),
            extractor,
        }
    }
}

impl<B: BufRead, Extractor: ModifierExtractor> Iterator for SnmpDataParser<B, Extractor> {
    type Item = Result<SnmpData, ParserError>;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let mut snmp_data = SnmpData::new();
        let result = match snmp_data.parse(&self.extractor, &self.line_parser) {
            Ok(_) if snmp_data.is_empty() => return None,
            Ok(_) => Ok(snmp_data),
            Err(err) => Err(err),
        };

        Some(result)
    }
}
