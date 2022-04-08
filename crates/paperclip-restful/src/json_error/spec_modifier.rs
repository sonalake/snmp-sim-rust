use paperclip::v2::models::DefaultApiRaw;

use paperclip::v2::schema::Apiv2Schema;

use crate::JsonErrorResponse;

pub fn add_json_error(spec: &mut DefaultApiRaw) {
    for resp in spec
        .paths
        .values_mut()
        .flat_map(|pt| pt.methods.values_mut())
        .flat_map(|m| &mut m.responses)
        .filter(|(k, _)| k.starts_with('4') || k.starts_with('5'))
        .map(|(_, v)| v)
    {
        if resp.schema.is_none() {
            resp.schema = Some(JsonErrorResponse::raw_schema());
        }
    }
}
