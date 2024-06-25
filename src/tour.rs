use crate::config::TourConfig;
use crate::navigation::Navigation;
use crate::progress::Progress;
use crate::selection::{selection_rect, Selection};
use crate::step_info::StepInfo;
use yew::prelude::*;

fn window_height() -> i32 {
    web_sys::window()
        .unwrap()
        .inner_height()
        .unwrap()
        .as_f64()
        .unwrap() as i32
}

#[function_component(Tour)]
pub fn tour(config: &TourConfig) -> Html {
    let show_tour = use_state(|| true);
    let current_step = use_state(|| 0usize);

    if !*show_tour {
        return html! { <></> };
    }

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

    let on_skip = {
        let show_tour = show_tour.clone();
        Callback::from(move |_| {
            show_tour.set(false);
        })
    };

    let on_progress_click = {
        let current_step = current_step.clone();
        Callback::from(move |step: usize| {
            current_step.set(step);
        })
    };

    let selector: String = config.steps[*current_step].selector.clone();

    let selector_rect = selection_rect(&selector).unwrap_or_default();

    let arrow_position = if selector_rect.y < window_height() / 2 {
        "top"
    } else {
        "bottom"
    };

    let dx = selector_rect.width / 2 + 10;
    let dy = if arrow_position == "top" {
        selector_rect.height + 10
    } else {
        -230
    };

    html! {
        <div class="tour">
            <div class="introjsFloatingElement"></div>
            <div class="introjs-overlay" style="inset: 0px; position: fixed; cursor: pointer;"></div>
            <Selection x={selector_rect.x} y={selector_rect.y} width={selector_rect.width} height={selector_rect.height} />
            <div class="introjs-tooltipReferenceLayer"
                style={format!("left: {}px; top: {}px; width: {}px; height: {}px;",
                    selector_rect.x, selector_rect.y, selector_rect.width, selector_rect.height)} >
                <div class="introjs-tooltip introjs-bottom-left-aligned" role="dialog"
                    style={format!("left: {}px; top: {}px; height: 220px", dx, dy)}>
                    <div class={format!("introjs-arrow {}", arrow_position)} style="display: inherit;"></div>
                    <div class="introjs-tooltip-header">
                    <StepInfo value={*current_step} />
                    <h1 class="introjs-tooltip-title">
                    </h1>
                    <a class="introjs-skipbutton" href="#" onclick={on_skip}>
                        {"×"}
                    </a>
                </div>
                <div class="introjs-tooltiptext">
                    <div>{&config.steps[*current_step].content}</div>
                </div>
                <Progress current={*current_step} total={config.steps.len()} on_click={on_progress_click} />
                <Navigation on_prev={on_prev} on_next={on_next} />
                <div class="introjs-tooltipfooter"></div>
            </div>
        </div>
        </div>
    }
}
