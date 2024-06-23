use crate::step::TourStep;
use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Properties, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TourConfig {
    pub steps: Vec<TourStep>,
}
