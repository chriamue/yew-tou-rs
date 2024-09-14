use crate::components::{Content, Navigation, Progress, Selection, StepInfo};
use crate::models::{Rect, TourConfig};
use crate::utils::calculate_arrow_position;
#[cfg(feature = "storage")]
use gloo_storage::{LocalStorage, Storage};
use web_sys::ScrollToOptions;
use yew::prelude::*;

pub(crate) const ARROW_SIZE: i32 = 10;
pub(crate) const TOOLTIP_WIDTH: i32 = 300;
pub(crate) const TOOLTIP_HEIGHT: i32 = 230;

// Helper functions for window dimensions
fn window_height() -> i32 {
    web_sys::window()
        .unwrap()
        .inner_height()
        .unwrap()
        .as_f64()
        .unwrap() as i32
}

fn window_width() -> i32 {
    web_sys::window()
        .unwrap()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap() as i32
}

// New function to scroll the selected element into view
fn scroll_into_view(rect: &Rect) {
    let window = web_sys::window().unwrap();
    let mut options = ScrollToOptions::new();
    options.top(rect.top().into());
    options.left(rect.left().into());
    options.behavior(web_sys::ScrollBehavior::Smooth);
    window.scroll_to_with_scroll_to_options(&options);
}

#[function_component(Tour)]
pub fn tour(config: &TourConfig) -> Html {
    let id = config.id.clone().unwrap_or_else(|| "tour".to_string());

    let show_tour = {
        #[cfg(feature = "storage")]
        let default_show = LocalStorage::get(format!("{}-show", id)).unwrap_or_else(|_| true);
        #[cfg(not(feature = "storage"))]
        let default_show = true;
        use_state(|| default_show)
    };
    let current_step = use_state(|| 0usize);

    if !*show_tour {
        return html! {};
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
        let id = id.clone();
        Callback::from(move |_| {
            show_tour.set(false);
            #[cfg(feature = "storage")]
            let _ = LocalStorage::set(format!("{}-show", id), false);
        })
    };

    let on_progress_click = {
        let current_step = current_step.clone();
        Callback::from(move |step: usize| {
            current_step.set(step);
        })
    };

    let selector: String = config.steps[*current_step].selector.clone();

    // Get the rectangle of the selected element
    let selector_rect = crate::models::get_element_rect(&selector).unwrap_or_default();

    // Scroll the selected element into view
    scroll_into_view(&selector_rect);

    // Calculate the tooltip position
    let (arrow_position, dx, dy) = calculate_arrow_position(
        &selector_rect,
        TOOLTIP_WIDTH,
        TOOLTIP_HEIGHT,
        window_width(),
        window_height(),
    );

    // Adjust tooltip position relative to the selected element
    let tooltip_left = dx - selector_rect.left();
    let tooltip_top = dy - selector_rect.top();

    html! {
        <div class="tour" id={id.clone()}>
            <div class="introjsFloatingElement"></div>
            <div class="introjs-overlay" style="inset: 0px; position: fixed; cursor: pointer;" onclick={on_skip.clone()}></div>
            <Selection rect={selector_rect} />
            <div class="introjs-tooltipReferenceLayer"
                style={format!("left: {}px; top: {}px; width: {}px; height: {}px; position: absolute;",
                    selector_rect.left(), selector_rect.top(), selector_rect.width, selector_rect.height)} >
                <div class="introjs-tooltip" role="dialog"
                    style={format!("left: {}px; top: {}px; position: absolute; width: {}px; height: {}px;",
                        tooltip_left, tooltip_top, TOOLTIP_WIDTH, TOOLTIP_HEIGHT)}>
                    <div class={format!("introjs-arrow {}", arrow_position)} style="display: inherit;"></div>
                    <div class="introjs-tooltip-header">
                        <StepInfo value={*current_step} />
                        <a class="introjs-skipbutton" href="#" onclick={on_skip.clone()}>
                            {"×"}
                        </a>
                    </div>
                    <Content content={config.steps[*current_step].content.clone()} />
                    <Progress current={*current_step} total={config.steps.len()} on_click={on_progress_click} />
                    <Navigation on_prev={on_prev} on_next={on_next} />
                    <div class="introjs-tooltipfooter"></div>
                </div>
            </div>
        </div>
    }
}
