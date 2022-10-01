use serde::{Deserialize, Serialize};

pub(crate) struct MappingWithKey {
    pub ecwid_id: String,
    pub details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Details {
    #[serde(rename(serialize = "d"))]
    design_id: String,
}

impl MappingWithKey {
    pub(crate) fn new(ecwid_id: &str, design_id: &str) -> Self {
        Self {
            ecwid_id: ecwid_id.to_string(),
            details: Details {
                design_id: design_id.to_string(),
            },
        }
    }
}
