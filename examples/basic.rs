use yew::prelude::*;
use yew_tou_rs::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // Define the steps for the tour
    let steps = vec![
        TourStep {
            selector: ".app".to_string(),
            content: "### Welcome to the tour\n\
                You can find a full demo on [Konnektoren](https://konnektoren.help)"
                .to_string(),
        },
        TourStep {
            selector: ".h1-step".to_string(),
            content: "This is a title".to_string(),
        },
        TourStep {
            selector: ".p-step".to_string(),
            content: "This is a paragraph".to_string(),
        },
        TourStep {
            selector: ".button-step".to_string(),
            content: "This is a button".to_string(),
        },
        // New step for the right-side element
        TourStep {
            selector: ".right-side-element".to_string(),
            content: "This element is on the right edge of the screen.".to_string(),
        },
        TourStep {
            selector: "footer".to_string(),
            content: "This is a footer".to_string(),
        },
    ];

    // Render the main application structure
    html! {
        <div class="app">
            // Existing elements
            <h1 class="h1-step" data-title="h1" data-intro="This is a title">{"Hello, Yew Tou-rs"}</h1>
            <p class="p-step" data-title="p" data-intro="This is a paragraph">{"This is a simple example of a Yew Tou-rs."}</p>
            <button class="button-step">{"Click me"}</button>

            // New element on the right side
            <div class="right-side-element">
                {"Right Side Element"}
            </div>

            <footer class="footer-step">
                <a href="https://github.com/chriamue/yew-tou-rs">{"yew-tou-rs"}</a>
            </footer>

            // Include the Tour component, passing in the defined steps
            <Tour steps={steps} />
        </div>
    }
}

fn main() {
    // Initialize and render the Yew application
    yew::Renderer::<App>::new().render();
}
