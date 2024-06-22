use yew::prelude::*;

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

    let on_next = {
        let current_step = current_step.clone();
        let step_count = config.steps.len();
        Callback::from(move |_| {
            if *current_step < step_count - 1 {
                current_step.set(*current_step + 1);
            }
        })
    };

    let on_prev = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            if *current_step > 0 {
                current_step.set(*current_step - 1);
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
