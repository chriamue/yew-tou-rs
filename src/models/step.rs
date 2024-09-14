use serde::{Deserialize, Serialize};

/// Represents a single step in a tour.
///
/// Each `TourStep` defines a specific element to highlight and the content to display
/// for that step in the tour.
///
/// # Fields
///
/// * `selector` - A CSS selector string used to identify the element to highlight.
/// * `content` - The text content to display for this step of the tour.
///
/// # Examples
///
/// ```
/// use yew_tou_rs::prelude::TourStep;
///
/// let step = TourStep {
///     selector: ".intro-element".to_string(),
///     content: "This is the first step of the tour.".to_string(),
/// };
/// ```
///
/// # Serialization
///
/// This struct derives `Serialize` and `Deserialize`, allowing it to be easily
/// converted to and from various data formats like JSON.
///
/// # Clone and Debug
///
/// `TourStep` can be cloned and printed for debugging purposes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TourStep {
    /// A CSS selector string used to identify the element to highlight for this step.
    ///
    /// This should be a valid CSS selector that uniquely identifies the target element
    /// on the page. For example, "#intro-button" or ".navbar-item:first-child".
    pub selector: String,

    /// The content to display for this step of the tour.
    ///
    /// This can be plain text or formatted text (e.g., Text or Markdown, depending on
    /// your tour implementation). It explains the highlighted element or provides
    /// instructions to the user.
    /// For Markdown the feature `markdown` must be enabled.
    pub content: String,
}
