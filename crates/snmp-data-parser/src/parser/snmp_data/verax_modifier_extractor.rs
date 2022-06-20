use crate::parser::snmp_data::modifier_extractor::ModifierExtractor;

#[derive(Clone)]
pub struct VeraxModifierExtractor;
impl ModifierExtractor for VeraxModifierExtractor {
    fn process_post_loaded_modifier(&self, data_value: &str) -> Option<(String, String)> {
        self.process_modifier(data_value, r"//\$([\W\w]*)")
    }

    fn process_pre_loaded_modifier(&self, data_value: &str) -> Option<(String, String)> {
        self.process_modifier(data_value, r"//\^([\W\w]*)\^//")
    }
}

impl VeraxModifierExtractor {
    fn process_modifier(&self, data_value: &str, regex_pattern: &str) -> Option<(String, String)> {
        if data_value.contains("//") {
            let pre_loaded_re = regex::Regex::new(regex_pattern).unwrap();
            if let Some(caps) = pre_loaded_re.captures(data_value) {
                if caps.len() == 2 {
                    let property_value = data_value.replace(caps.get(0).unwrap().as_str(), "");
                    let post_loaded_modifier = caps.get(1).unwrap().as_str().to_string();

                    return Some((property_value, post_loaded_modifier));
                } else {
                    tracing::error!("Found more captures than expected {:?}", caps);
                }
            }
        }
        None
    }
}
