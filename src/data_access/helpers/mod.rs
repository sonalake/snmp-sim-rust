pub mod agents;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) use agents::*;

use crate::data_access::entity::agents::*;

impl From<ActiveModel> for Model {
    fn from(am: ActiveModel) -> Self {
        Self {
            id: am.id.unwrap(),
            created_at: am.created_at.unwrap(),
            modified_at: am.modified_at.unwrap(),
            name: am.name.unwrap(),
        }
    }
}
