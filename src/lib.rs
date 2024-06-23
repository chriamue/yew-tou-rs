use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TourStep {
    pub selector: String,
    pub content: String,
}

#[derive(Properties, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TourConfig {
    pub steps: Vec<TourStep>,
}

#[function_component(Tour)]
pub fn tour(config: &TourConfig) -> Html {
    let show_tour = use_state(|| true);
    let current_step = use_state(|| 0usize);
    let last_selector = use_state(|| "".to_string());
    let tooltip_position = use_state(|| (0.0, 0.0));
    let arrow_position = use_state(|| "top".to_string());

    let update_position = {
        let tooltip_position = tooltip_position.clone();
        let arrow_position = arrow_position.clone();
        Callback::from(move |selector: String| {
            let document = web_sys::window().unwrap().document().unwrap();
            if let Some(element) = document.query_selector(&selector).unwrap() {
                let rect = element.get_bounding_client_rect();
                let window = web_sys::window().unwrap();
                let scroll_x = window.scroll_x().unwrap();
                let scroll_y = window.scroll_y().unwrap();

                let top = rect.top() + scroll_y;
                let left = rect.left() + scroll_x;
                let height = rect.height();
                let width = rect.width();

                tooltip_position.set((
                    (top + height).max(0.0),
                    (left + width / 2.0 - 125.0).max(0.0),
                ));
                if top < 0.0 {
                    arrow_position.set("bottom".to_string());
                } else {
                    arrow_position.set("top".to_string());
                }
            }
        })
    };

    let on_next = {
        let config = config.clone();
        let current_step = current_step.clone();
        let last_selector = last_selector.clone();
        let step_count = config.steps.len();
        let update_position = update_position.clone();
        Callback::from(move |_| {
            if *current_step < step_count - 1 {
                clear_highlight(&config.steps[*current_step].selector.clone());
                highlight_element(&config.steps[*current_step + 1].selector);
                update_position.emit(config.steps[*current_step + 1].selector.clone());
                last_selector.set(config.steps[*current_step].selector.clone());
                current_step.set(*current_step + 1);
            }
        })
    };

    let on_prev = {
        let config = config.clone();
        let current_step = current_step.clone();
        let last_selector = last_selector.clone();
        let update_position = update_position.clone();
        Callback::from(move |_| {
            if *current_step > 0 {
                clear_highlight(&config.steps[*current_step].selector.clone());
                highlight_element(&config.steps[*current_step - 1].selector);
                update_position.emit(config.steps[*current_step - 1].selector.clone());
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

    let (tooltip_top, tooltip_left) = *tooltip_position;

    html! {
        <div class="tour">
            <div class="introjsFloatingElement"></div>
            <div class="introjs-overlay" style="inset: 0px; position: fixed; cursor: pointer;"></div>
            <div class="introjs-helperLayer"></div>
            <div class="introjs-tooltipReferenceLayer" style={format!("left: {}px; top: {}px;", tooltip_left, tooltip_top)} >
                <div class="introjs-tooltip introjs-floating" role="dialog" >
                    <div class="introjs-arrow top" style="display: inherit;"></div>
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
                            let update_position = update_position.clone();
                            let i = i as usize;
                            let is_current = *current_step == i;
                            let config = config.clone();
                            let on_click = Callback::from(move |_| {
                                clear_highlight(&*last_selector);
                                highlight_element(&config.steps[i].selector);
                                update_position.emit(config.steps[i].selector.clone());
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
        </div>
    }
}

fn highlight_element(selector: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    if let Some(element) = document.query_selector(selector).unwrap() {
        let rect = element.get_bounding_client_rect();

        let helper_layer = if let Some(existing) = document.query_selector(".introjs-helperLayer").unwrap() {
            existing
        } else {
            let div = document.create_element("div").unwrap();
            div.set_class_name("introjs-helperLayer");
            document.body().unwrap().append_child(&div).unwrap();
            div
        };

        let reference_layer = if let Some(existing) = document.query_selector(".introjs-tooltipReferenceLayer").unwrap() {
            existing
        } else {
            let div = document.create_element("div").unwrap();
            div.set_class_name("introjs-tooltipReferenceLayer");
            document.body().unwrap().append_child(&div).unwrap();
            div
        };

        helper_layer
            .set_attribute(
                "style",
                &format!(
                    "position: absolute; top: {}px; left: {}px; width: {}px; height: {}px; box-shadow: rgba(33, 33, 33, 0.8) 0px 0px 1px 2px, rgba(33, 33, 33, 0.5) 0px 0px 0px 5000px; opacity: 1;",
                    rect.top(), rect.left(), rect.width(), rect.height()
                ),
            )
            .unwrap();

        reference_layer
            .set_attribute(
                "style",
                &format!(
                    "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
                    rect.left(), rect.top(), rect.width(), rect.height()
                ),
            )
            .unwrap();
    }
}

fn clear_highlight(selector: &str) {
    let cleared_selector = selector.replace(".", "");
    let document = web_sys::window().unwrap().document().unwrap();
    let elements = document.query_selector_all(".introjs-showElement").unwrap();
    for i in 0..elements.length() {
        if let Some(node) = elements.item(i) {
            if let Ok(element) = node.dyn_into::<Element>() {
                element.set_attribute("style", "").unwrap();
                let old_class = element.get_attribute("class").unwrap();
                let new_class = old_class.replace("introjs-showElement", "");
                let new_class = new_class.replace("introjs-relativePosition", "");
                element.set_attribute("class", &new_class).unwrap();
            }
        }
    }
}
