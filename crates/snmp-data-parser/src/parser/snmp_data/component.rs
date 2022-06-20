use crate::parser::snmp_data::modifier_extractor::ModifierExtractor;
use crate::parser::ParserError;
use crate::{parser::SnmpDataItems, property::Property, VALUE_TYPE_DELIMITER};

use rasn::prelude::ObjectIdentifier;
use std::collections::BTreeMap;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Clone)]
pub struct SnmpDataItem {
    pub data_type: DataType,
    pub data_value: String,
    pub pre_loaded_mod: Vec<String>,
    pub post_loaded_mod: Option<String>,
}

impl SnmpDataItem {
    pub fn new<Extractor: ModifierExtractor>(extractor: &Extractor, data_type: DataType, data_value: &str) -> Self {
        SnmpDataItem {
            data_type,
            data_value: data_value.to_string(),
            pre_loaded_mod: vec![],
            post_loaded_mod: None,
        }
        .process_data_value_modifiers(extractor)
    }

    fn process_data_value_modifiers<Extractor: ModifierExtractor>(&mut self, extractor: &Extractor) -> Self {
        if let Some((property_value, pre_loaded_modifier)) = extractor.process_pre_loaded_modifier(&self.data_value) {
            // TODO implement value pre-loaded modifiers support #35
            self.pre_loaded_mod.push(pre_loaded_modifier);
            self.data_value = property_value;
        }

        if let Some((property_value, post_loaded_modifier)) = extractor.process_post_loaded_modifier(&self.data_value) {
            // TODO implement value post-loaded modifiers support #36
            self.post_loaded_mod = Some(post_loaded_modifier);
            self.data_value = property_value;
        }

        // TODO validate data_value against the expected data type #38

        self.clone()
    }
}

/// An SNMP file data
#[derive(Debug, Clone, Default)]
pub struct SnmpData {
    pub data: BTreeMap<ObjectIdentifier, SnmpDataItem>,
}
impl std::ops::Deref for SnmpData {
    type Target = BTreeMap<ObjectIdentifier, SnmpDataItem>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for SnmpData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Clone, Debug, PartialEq, EnumString)]
pub enum DataType {
    #[strum(serialize = "String", serialize = "STRING")]
    String,
    #[strum(serialize = "OID")]
    Oid,
    #[strum(serialize = "INTEGER", serialize = "Integer32")]
    Integer,
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
    pub fn new() -> Self {
        SnmpData { data: BTreeMap::new() }
    }
}

impl SnmpDataItems for SnmpData {
    fn add_data<Extractor: ModifierExtractor>(
        &mut self,
        extractor: &Extractor,
        property: Property,
    ) -> Result<(), ParserError> {
        let value_type_index = property
            .value
            .find(VALUE_TYPE_DELIMITER)
            .unwrap_or(usize::max_value());

        let mut data_type = DataType::String;
        let data_value: &str;
        if value_type_index != usize::max_value() && value_type_index != 0 {
            let (data_type_raw, value) = property.value.split_at(value_type_index);
            data_type = DataType::from_str(data_type_raw)
                .map_err(|err| ParserError::UnrecognozedDataType(data_type_raw.to_string(), err))?;
            data_value = value.trim().trim_start_matches(VALUE_TYPE_DELIMITER).trim();
        } else {
            data_value = property.value.trim();
        }

        let snmp_data_item = SnmpDataItem::new(extractor, data_type, data_value);

        // TODO implement OID pre-loaded modifiers support #37
        self.insert(string_to_oid(&property.name), snmp_data_item);

        Ok(())
    }
}

pub fn oid_to_string(oid: ObjectIdentifier) -> String {
    oid.iter().map(|&id| format!(".{id}")).collect()
}

pub fn string_to_oid(oid: &str) -> ObjectIdentifier {
    rasn::types::ObjectIdentifier::new(
        oid.trim_matches('.')
            .split('.')
            .map(|val| val.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .to_vec(),
    )
    .unwrap()
}
