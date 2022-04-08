use super::JsonError;
use actix_web::web::{FormConfig, JsonConfig, PathConfig, QueryConfig};

macro_rules! extractor_config {
    ($vi:vis fn $id:ident() -> $extractor:ty) => {
        $vi fn $id() -> $extractor {
            <$extractor>::default().error_handler(|err, _| JsonError::from(err).into())
        }
    };
}

extractor_config!(pub fn path_extractor_config() -> PathConfig);
extractor_config!(pub fn json_extractor_config() -> JsonConfig);
extractor_config!(pub fn form_extractor_config() -> FormConfig);
extractor_config!(pub fn query_extractor_config() -> QueryConfig);
