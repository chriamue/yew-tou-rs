use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TourStep {
    pub selector: String,
    pub content: String,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct TourConfig {
    pub selector: String,
    pub steps: Vec<TourStep>,
}

#[function_component(Tour)]
pub fn tour(config: &TourConfig) -> Html {
    let current_step = use_state(|| 0usize);
    let last_selector = use_state(|| "".to_string());

    let highlight_element = {
        let current_step = current_step.clone();
        let last_selector = last_selector.clone();
        let steps = config.steps.clone();
        let selector = config.selector.clone();
        Callback::from(move |_| {
            let current_selector = &steps[*current_step].selector;
            clear_highlight(&*last_selector);
            if let Some(element) = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .query_selector(&current_selector)
                .unwrap()
            {
                element
                    .set_attribute("class", "introjs-helperNumberLayer")
                    .unwrap();
                element
                    .set_attribute("style", "border: 2px solid red;")
                    .unwrap();
            }
        })
    };

    let on_next = {
        let config = config.clone();
        let current_step = current_step.clone();
        let last_selector = last_selector.clone();
        let step_count = config.steps.len();
        let highlight_element = highlight_element.clone();
        Callback::from(move |_| {
            if *current_step < step_count - 1 {
                last_selector.set(config.steps[*current_step].selector.clone());
                current_step.set(*current_step + 1);
                highlight_element.emit(());
            }
        })
    };

    let on_prev = {
        let config = config.clone();
        let current_step = current_step.clone();
        let last_selector = last_selector.clone();
        let highlight_element = highlight_element.clone();
        Callback::from(move |_| {
            if *current_step > 0 {
                last_selector.set(config.steps[*current_step].selector.clone());
                current_step.set(*current_step - 1);
                highlight_element.emit(());
            }
        })
    };

    html! {
        <>
            <div class="introjs-overlay"></div>
            <div class="introjs-helperLayer">
                <div class="introjs-tooltipReferenceLayer"></div>
            </div>
            <div class="introjs-tooltipReferenceLayer"></div>
            <div class="introjs-tooltip introjs-floating">
                <div class="introjs-tooltipheader">
                    <div class="introjs-tooltiptitle"></div>
                </div>
                <div class="introjs-tooltiptext">
                    <div>{"Text: "}{&config.steps[*current_step].content}</div>
                </div>
                <div class="introjs-bullets">
                    <ul role="tablist">
                        {for (0..config.steps.len()).map(|i| {
                            let current_step = current_step.clone();
                            let highlight_element = highlight_element.clone();
                            let i = i as usize;
                            let step = &config.steps[i];
                            let is_current = *current_step == i;
                            let on_click = Callback::from(move |_| {
                                current_step.set(i);
                                highlight_element.emit(());
                            });

                            html! {
                                <li role="presentation">
                                    <a role="button" data-step-number={format!("{}", i)} onclick={on_click} class={if is_current { "active" } else { "" }} >
                                        {" "}
                                    </a>
                                </li>
                            }
                        })}
                    </ul>
                </div>
                <div class="introjs-tooltipbuttons">
                    <button onclick={on_prev}>{"Prev"}</button>
                    <button onclick={on_next}>{"Next"}</button>
                </div>
                <div class="introjs-tooltipfooter"></div>
            </div>
        </>
    }
}

fn clear_highlight(selector: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let elements = document.query_selector_all(".introjs-helperNumberLayer").unwrap();
    for i in 0..elements.length() {
        if let Some(node) = elements.item(i) {
            if let Ok(element) = node.dyn_into::<Element>() {
                element.set_attribute("style", "").unwrap();
                element.set_attribute("class", selector).unwrap();
            }
        }
    }
}
