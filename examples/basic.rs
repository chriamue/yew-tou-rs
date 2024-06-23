use yew::prelude::*;
use yew_tou_rs::{Tour, TourStep};

#[function_component(App)]
pub fn app() -> Html {
    let steps = vec![
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
        }
    ];

    html! {
        <>
            <h1 class="h1-step" data-title="h1" data-intro="This is a title">{"Hello, Yew Tou-rs"}</h1>
            <p class="p-step" data-title="p" data-intro="This is a paragraph">{"This is a simple example of a Yew Tou-rs."}</p>
            <button class="button-step">{"Click me"}</button>
            <Tour steps={steps} selector={"introduction"}/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
