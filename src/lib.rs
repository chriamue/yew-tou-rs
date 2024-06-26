//! A library for creating interactive tours in Yew applications.
//!
//! `yew-tou-rs` provides an easy way to implement guided tours in web applications
//! built with the Yew framework. It allows developers to create step-by-step
//! tutorials that highlight different elements of their user interface.
//!
//! # Features
//!
//! - Easy integration with Yew components
//! - Customizable tour steps
//! - Optional storage feature for persisting tour state
//! - Optional Markdown support for tour content
//!
//! # Usage
//!
//! To use this library, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! yew-tou-rs = { git = "https://github.com/chriamue/yew-tou-rs" }
//! ```
//!
//! For optional features, add:
//!
//! ```toml
//! [dependencies]
//! yew-tou-rs = { git = "https://github.com/chriamue/yew-tou-rs", features = ["storage", "markdown"] }
//! ```
//!
//! Then, in your Rust code, import the prelude to access the main components:
//!
//! ```rust
//! use yew_tou_rs::prelude::*;
//! ```
//!
//! # Example
//!
//! Here's a basic example of how to use the `Tour` component in a Yew application:
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_tou_rs::prelude::*;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let tour_config = TourConfig {
//!         id: Some("main-tour".to_string()),
//!         steps: vec![
//!             TourStep {
//!                 selector: ".intro-element".to_string(),
//!                 content: "Welcome to our app! This is the first step of the tour.".to_string(),
//!             },
//!             TourStep {
//!                 selector: "#feature-button".to_string(),
//!                 content: "Click here to see our main feature.".to_string(),
//!             },
//!             TourStep {
//!                 selector: ".help-section".to_string(),
//!                 content: "If you need help, you can always find it here.".to_string(),
//!             },
//!         ],
//!     };
//!
//!     html! {
//!         <div>
//!             <h1 class="intro-element">{"Welcome to Our App"}</h1>
//!             <button id="feature-button">{"Main Feature"}</button>
//!             <div class="help-section">{"Help & Support"}</div>
//!
//!             <Tour steps={tour_config.steps} />
//!         </div>
//!     }
//! }
//! ```
//!
//! # Modules
//!
//! - `config`: Defines the `TourConfig` struct for configuring tours.
//! - `step`: Contains the `TourStep` struct for individual tour steps.
//! - `tour`: Implements the main `Tour` component.
//! - Other modules provide supporting functionality and components.
//!
//! # License
//!
//! This project is licensed under the MIT License. See the [LICENSE](https://github.com/chriamue/yew-tou-rs/blob/main/LICENSE) file for details.

mod config;
mod content;
mod navigation;
mod progress;
mod rect;
mod selection;
mod step;
mod step_info;
mod tour;

/// Prelude module for convenient imports of commonly used items.
///
/// This module re-exports the main components and structures needed to use the tour functionality.
/// By importing this prelude, users can easily access the necessary items without multiple imports.
///
/// # Usage
///
/// To use the prelude, add the following to your Rust file:
///
/// ```rust
/// use yew_tou_rs::prelude::*;
/// ```
///
/// For a complete usage example, see the crate-level documentation.
pub mod prelude {
    pub use crate::config::TourConfig;
    pub use crate::step::TourStep;
    pub use crate::tour::Tour;
}