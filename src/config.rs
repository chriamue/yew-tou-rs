use crate::step::TourStep;
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Properties, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TourConfig {
    #[prop_or_default]
    pub id: Option<String>,
    pub steps: Vec<TourStep>,
}
