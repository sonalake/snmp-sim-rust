const VALUE_DELIMITER: char = '=';
const VALUE_TYPE_DELIMITER: char = ':';

pub mod parser;
pub use crate::parser::snmp_data::SnmpDataParser;

pub mod property;
pub use crate::property::PropertyParser;

pub mod line;
pub use crate::line::LineReader;
