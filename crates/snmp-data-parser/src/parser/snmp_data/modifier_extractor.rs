pub trait ModifierExtractor {
    fn process_post_loaded_modifier(&self, data_value: &str) -> Option<(String, String)>;
    fn process_pre_loaded_modifier(&self, data_value: &str) -> Option<(String, String)>;
}
