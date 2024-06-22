use yew::prelude::*;
use yew_tou_rs::{Tour, TourStep};

#[function_component(App)]
pub fn app() -> Html {
    let steps = vec![
        TourStep {
            selector: "h1".to_string(),
            text: "This is a title".to_string(),
        },
        TourStep {
            selector: "p".to_string(),
            text: "This is a paragraph".to_string(),
        },
        TourStep {
            selector: "button".to_string(),
            text: "This is a button".to_string(),
        }
    ];

    html! {
        <>
            <h1>{"Hello, Yew Tou-rs"}</h1>
            <p>{"This is a simple example of a Yew Tou-rs."}</p>
            <Tour steps={steps} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
