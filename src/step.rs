use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TourStep {
    pub selector: String,
    pub content: String,
}
