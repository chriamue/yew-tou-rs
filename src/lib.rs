use yew::prelude::*;
use web_sys::Element;
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, PartialEq)]
pub struct TourStep {
    pub selector: String,
    pub text: String,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct TourConfig {
    pub steps: Vec<TourStep>,
}
#[function_component(Tour)]
pub fn tour(config: &TourConfig) -> Html {
    let current_step = use_state(|| 0usize);

    let highlight_element = {
        let current_step = current_step.clone();
        let steps = config.steps.clone();
        Callback::from(move |_| {
            let current_selector = &steps[*current_step].selector;
            if let Some(element) = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .query_selector(current_selector)
                .unwrap()
            {
                clear_highlight();
                element.set_attribute("style", "border: 2px solid red;").unwrap();
            }
        })
    };

    let on_next = {
        let current_step = current_step.clone();
        let step_count = config.steps.len();
        let highlight_element = highlight_element.clone();
        Callback::from(move |_| {
            if *current_step < step_count - 1 {
                current_step.set(*current_step + 1);

                highlight_element.emit(());
            }
        })
    };

    let on_prev = {
        let current_step = current_step.clone();
        let highlight_element = highlight_element.clone();
        Callback::from(move |_| {
            if *current_step > 0 {
                current_step.set(*current_step - 1);

                highlight_element.emit(());
            }
        })
    };

    html! {
        <div>
            <div>{"Step: "}{*current_step}</div>
            <div>{"Selector: "}{&config.steps[*current_step].selector}</div>
            <div>{"Text: "}{&config.steps[*current_step].text}</div>
            <button onclick={on_prev}>{"Prev"}</button>
            <button onclick={on_next}>{"Next"}</button>
        </div>
    }
}

fn clear_highlight() {
    let document = web_sys::window().unwrap().document().unwrap();
    let elements = document.query_selector_all("*").unwrap();
    for i in 0..elements.length() {
        if let Some(node) = elements.item(i) {
            if let Ok(element) = node.dyn_into::<Element>() {
                element.set_attribute("style", "").unwrap();
            }
        }
    }
}
