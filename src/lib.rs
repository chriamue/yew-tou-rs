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
    let show_tour = use_state(|| true);
    let current_step = use_state(|| 0usize);
    let last_selector = use_state(|| "".to_string());

    let on_next = {
        let config = config.clone();
        let current_step = current_step.clone();
        let last_selector = last_selector.clone();
        let step_count = config.steps.len();
        let highlight_element = highlight_element.clone();
        Callback::from(move |_| {
            if *current_step < step_count - 1 {
                clear_highlight(&config.steps[*current_step].selector.clone());
                highlight_element(&config.steps[*current_step + 1].selector);
                last_selector.set(config.steps[*current_step].selector.clone());
                current_step.set(*current_step + 1);
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
                clear_highlight(&config.steps[*current_step].selector.clone());
                highlight_element(&config.steps[*current_step - 1].selector);
                last_selector.set(config.steps[*current_step].selector.clone());
                current_step.set(*current_step - 1);
            }
        })
    };

    let on_skip = {
        let config = config.clone();
        let current_step = current_step.clone();
        let show_tour = show_tour.clone();
        Callback::from(move |_| {
            clear_highlight(&config.steps[*current_step].selector.clone());
            show_tour.set(false);
        })
    };

    let config_clone = config.clone();
    let current_step_clone = current_step.clone();
    use_effect_with((), move |_| {
        highlight_element(&config_clone.steps[*current_step_clone].selector);
        || ()
    });

    if !*show_tour {
        return html! { <></> };
    }

    html! {
        <>
            <div class="introjsFloatingElement"></div>
            <div class="introjs-overlay" style="inset: 0px; position: fixed; cursor: pointer;"></div>
            <div class="introjs-helperLayer"></div>
            <div class="introjs-tooltipReferenceLayer" style="width: 0px; height: 0px; top: 356px; left: 416.5px;">
            <div class="introjs-tooltip introjs-floating" role="dialog" style="left: 50%; top: 50%; margin-left: -125px; margin-top: -107px;">
                <div class="introjs-tooltip-header">
                    <h1 class="introjs-tooltip-title">
                        {"Step "}{*current_step + 1}{" of "}{config.steps.len()}
                    </h1>
                    <a class="introjs-skipbutton" href="#" onclick={on_skip}>
                        {"×"}
                    </a>
                </div>
                <div class="introjs-tooltiptext">
                    <div>{&config.steps[*current_step].content}</div>
                </div>
                <div class="introjs-bullets">
                    <ul role="tablist">
                        {for (0..config.steps.len()).map(|i| {
                            let current_step = current_step.clone();
                            let last_selector = last_selector.clone();
                            let i = i as usize;
                            let is_current = *current_step == i;
                            let config = config.clone();
                            let on_click = Callback::from(move |_| {
                                clear_highlight(&*last_selector);
                                highlight_element(&config.steps[i].selector);
                                last_selector.set(config.steps[*current_step].selector.clone());
                                current_step.set(i);
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
                    <button class="introjs-button introjs-prevbutton" onclick={on_prev}>{"Prev"}</button>
                    <button class="introjs-button introjs-nextbutton" onclick={on_next}>{"Next"}</button>
                    <div class="introjs-tooltipbuttons::after"></div>
                </div>
                <div class="introjs-tooltipfooter"></div>
            </div>
        </div>
        </>
    }
}

fn highlight_element(selector: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    if let Some(element) = document.query_selector(selector).unwrap() {
        element
            .set_attribute("class", "introjs-helperNumberLayer")
            .unwrap();
        element
            .set_attribute("style", "border: 2px solid red; z-index: 10001;")
            .unwrap();
    }
}

fn clear_highlight(selector: &str) {
    let cleared_selector = selector.replace(".", "");
    let document = web_sys::window().unwrap().document().unwrap();
    let elements = document.query_selector_all(".introjs-helperNumberLayer").unwrap();
    for i in 0..elements.length() {
        if let Some(node) = elements.item(i) {
            if let Ok(element) = node.dyn_into::<Element>() {
                element.set_attribute("style", "").unwrap();
                element.set_attribute("class", &cleared_selector).unwrap();
            }
        }
    }
}
