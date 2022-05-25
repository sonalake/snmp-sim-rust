use crate::parser::ParserError;
use crate::{parser::SnmpDataItems, property::Property, VALUE_TYPE_DELIMITER};
use std::collections::BTreeMap;
use std::str::FromStr;
use strum_macros::EnumString;

/// An SNMP file data
#[derive(Debug, Default)]
pub struct SnmpData {
    pub values: BTreeMap<String, (DataType, String)>,
}

#[derive(Debug, PartialEq, EnumString)]
pub enum DataType {
    #[strum(serialize = "String", serialize = "STRING")]
    String,
    #[strum(serialize = "OID")]
    Oid,
    #[strum(serialize = "INTEGER")]
    Integer,
    #[strum(serialize = "Integer32")]
    Integer32,
    #[strum(serialize = "Timeticks")]
    Timeticks,
    #[strum(serialize = "Counter32")]
    Counter32,
    #[strum(serialize = "Counter64")]
    Counter64,
    #[strum(serialize = "Gauge32")]
    Gauge32,
    #[strum(serialize = "IpAddress")]
    IpAddress,
    #[strum(serialize = "Hex-STRING")]
    HexString,
    #[strum(serialize = "Network Address")]
    NetworkAddress,
    #[strum(serialize = "Bits", serialize = "BITS")]
    Bits,
    #[strum(serialize = "Null")]
    Null,
    #[strum(serialize = "Opaque")]
    Opaque,
    #[strum(serialize = "UInteger32")]
    UInteger32,
    #[strum(serialize = "OctetString")]
    OctetString,
}

impl SnmpData {
    pub fn new() -> SnmpData {
        SnmpData {
            values: BTreeMap::new(),
        }
    }
}

impl SnmpDataItems for SnmpData {
    fn add_data(&mut self, property: Property) -> Result<(), ParserError> {
        let value_type_index = property
            .value
            .find(VALUE_TYPE_DELIMITER)
            .unwrap_or(usize::max_value());

        let mut data_type = DataType::String;
        let property_value: String;
        if value_type_index != usize::max_value() && value_type_index != 0 {
            let (data_type_raw, value) = property.value.split_at(value_type_index);
            data_type = DataType::from_str(data_type_raw)
                .map_err(|err| ParserError::UnrecognozedDataType(data_type_raw.to_string(), err))?;
            property_value = value
                .trim_matches(' ')
                .trim_start_matches(VALUE_TYPE_DELIMITER)
                .trim_matches(' ')
                .to_string();
        } else {
            property_value = property.value.trim_matches(' ').to_string();
        }

        self.values
            .insert(property.name, (data_type, property_value));

        Ok(())
    }
}
