use crate::models::TourStep;
use serde::{Deserialize, Serialize};
use yew::Properties;

/// Configuration for a tour.
///
/// `TourConfig` holds all the necessary information to define and run a tour,
/// including an optional identifier and a list of steps.
///
/// # Fields
///
/// * `id` - An optional unique identifier for the tour.
/// * `steps` - A vector of `TourStep`s that define the content and order of the tour.
///
/// # Examples
///
/// ```
/// use yew_tou_rs::prelude::{TourConfig, TourStep};
///
/// let config = TourConfig {
///     id: Some("main-tour".to_string()),
///     steps: vec![
///         TourStep {
///             selector: ".intro-element".to_string(),
///             content: "Welcome to the tour!".to_string(),
///         },
///         TourStep {
///             selector: "#feature-button".to_string(),
///             content: "Click here to see our main feature.".to_string(),
///         },
///     ],
/// };
/// ```
///
/// # Yew Properties
///
/// This struct derives `Properties`, allowing it to be used as properties for Yew components.
///
/// # Serialization
///
/// `TourConfig` can be serialized and deserialized, which is useful for storing configurations
/// or passing them between different parts of your application.
///
/// # Clone and Debug
///
/// This struct can be cloned and easily printed for debugging purposes.
#[derive(Properties, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TourConfig {
    /// An optional unique identifier for the tour.
    ///
    /// This can be used to distinguish between different tours in your application
    /// or to persist tour state. If not provided, a default value may be used.
    ///
    /// The `#[prop_or_default]` attribute means this field will use its default value (None)
    /// if not explicitly set when creating a `TourConfig` instance in a Yew component.
    #[prop_or_default]
    pub id: Option<String>,

    /// A vector of steps that define the content and order of the tour.
    ///
    /// Each step is represented by a `TourStep` struct, which includes information
    /// about what element to highlight and what content to display for that step.
    pub steps: Vec<TourStep>,
}
