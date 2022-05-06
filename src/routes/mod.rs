pub mod agents;
pub mod managed_devices;

pub use agents::*;
pub use managed_devices::*;

use paperclip::actix::Apiv2Schema;
use serde::Deserialize;

pub(crate) fn first() -> Option<usize> {
    Some(1)
}

pub(crate) fn twenty() -> Option<usize> {
    Some(20)
}

#[derive(Debug, Deserialize, Apiv2Schema)]
pub struct PageQuery {
    #[serde(default = "first")]
    /// Page index starts from one, default value is 1.
    pub page: Option<usize>,

    /// Number of results on a page, default value is 20.
    #[serde(default = "twenty")]
    pub page_size: Option<usize>,
}
