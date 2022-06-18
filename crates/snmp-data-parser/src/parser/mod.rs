pub mod snmp_data;

use crate::parser::snmp_data::ModifierExtractor;
use std::cell::RefCell;
use std::io::BufRead;

use crate::property::{Property, PropertyError, PropertyParser};

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("property error: {0}")]
    PropertyError(#[from] PropertyError),

    #[error("Unrecognozed data type: {0} {1}")]
    UnrecognozedDataType(String, strum::ParseError),
}

pub trait SnmpDataItems {
    /// Add the given property.
    fn add_data<Extractor: ModifierExtractor>(
        &mut self,
        extractor: &Extractor,
        property: Property,
    ) -> Result<(), ParserError>;

    /// Parse the content from `line_parser` and add the data.
    fn parse<B: BufRead, Extractor: ModifierExtractor>(
        &mut self,
        extractor: &Extractor,
        line_parser: &RefCell<PropertyParser<B>>,
    ) -> Result<(), ParserError> {
        loop {
            let line = match line_parser.borrow_mut().next() {
                Some(val) if val.is_ok() => val.unwrap(),
                // errors are ignored for now
                Some(_) => continue,
                None => return Ok(()),
            };

            self.add_data(extractor, line)?
        }
    }
}
