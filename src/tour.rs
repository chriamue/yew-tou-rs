use crate::components::content::Content;
use crate::components::navigation::Navigation;
use crate::components::progress::Progress;
use crate::components::selection::Selection;
use crate::components::step_info::StepInfo;
use crate::models::get_element_rect;
use crate::models::TourConfig;
use crate::utils::calculate_arrow_position;

#[cfg(feature = "storage")]
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

pub(crate) const ARROW_SIZE: i32 = 10;
pub(crate) const TOOLTIP_WIDTH: i32 = 300;
pub(crate) const TOOLTIP_HEIGHT: i32 = 230;

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

#[function_component(Tour)]
pub fn tour(config: &TourConfig) -> Html {
    let id = config.id.clone().unwrap_or_else(|| "tour".to_string());

    let show_tour = {
        #[cfg(feature = "storage")]
        let default_show = LocalStorage::get(format!("{}-show", id))
            .unwrap_or_else(|_| "true".to_string())
            == "true";
        #[cfg(not(feature = "storage"))]
        let default_show = true;
        use_state(|| default_show)
    };
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
        let id = id.clone();
        Callback::from(move |_| {
            show_tour.set(false);
            #[cfg(feature = "storage")]
            let _ = LocalStorage::set(format!("{}-show", id), "false");
        })
    };

    let on_progress_click = {
        let current_step = current_step.clone();
        Callback::from(move |step: usize| {
            current_step.set(step);
        })
    };

    let selector: String = config.steps[*current_step].selector.clone();

    let selector_rect = get_element_rect(&selector).unwrap_or_default();

    let (arrow_position, dx, dy) = calculate_arrow_position(
        &selector_rect,
        TOOLTIP_WIDTH,
        TOOLTIP_HEIGHT,
        window_width(),
        window_height(),
    );

    html! {
        <div class="tour" id={id.clone()}>
            <div class="introjsFloatingElement"></div>
            <div class="introjs-overlay" style="inset: 0px; position: fixed; cursor: pointer;"></div>
            <Selection rect={selector_rect} />
            <div class="introjs-tooltipReferenceLayer"
                style={format!("left: {}px; top: {}px; width: {}px; height: {}px;",
                    selector_rect.x, selector_rect.y, selector_rect.width, selector_rect.height)} >
                <div class="introjs-tooltip" role="dialog"
                    style={format!("left: {}px; top: {}px;", dx, dy)}>
                    <div class={format!("introjs-arrow {}", arrow_position)} style="display: inherit;"></div>
                    <div class="introjs-tooltip-header">
                        <StepInfo value={*current_step} />
                        <a class="introjs-skipbutton" href="#" onclick={on_skip}>
                            {"×"}
                        </a>
                    </div>
                    <Content content={(&*config.steps[*current_step].content).to_string()} />
                    <Progress current={*current_step} total={config.steps.len()} on_click={on_progress_click} />
                    <Navigation on_prev={on_prev} on_next={on_next} />
                    <div class="introjs-tooltipfooter"></div>
                </div>
            </div>
        </div>
    }
}
