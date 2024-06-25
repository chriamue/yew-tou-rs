use crate::config::TourConfig;
use crate::navigation::Navigation;
use crate::progress::Progress;
use crate::rect::{get_element_rect, Rect};
use crate::selection::Selection;
use crate::step_info::StepInfo;
#[cfg(feature = "storage")]
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

const ARROW_SIZE: i32 = 10;
const TOOLTIP_WIDTH: i32 = 300;
const TOOLTIP_HEIGHT: i32 = 230;

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

pub fn calculate_arrow_position(
    selected_rect: &Rect,
    tooltip_width: i32,
    tooltip_height: i32,
    window_width: i32,
    window_height: i32,
) -> (&'static str, i32, i32) {
    let arrow_position =
        if selected_rect.y + selected_rect.height + tooltip_height + ARROW_SIZE > window_height {
            "bottom"
        } else {
            "top"
        };

    let dx = selected_rect.width / 2 - tooltip_width / 2;

    let dx = if selected_rect.x + dx > window_width {
        window_width - tooltip_width
    } else if dx < 0 {
        0
    } else {
        dx
    };

    let dy = if arrow_position == "bottom" {
        -tooltip_height
    } else {
        selected_rect.height + ARROW_SIZE
    };

    (arrow_position, dx, dy)
}

#[function_component(Tour)]
pub fn tour(config: &TourConfig) -> Html {
    let show_tour = {
        #[cfg(feature = "storage")]
        let default_show =
            LocalStorage::get("show_tour").unwrap_or_else(|_| "true".to_string()) == "true";
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
        Callback::from(move |_| {
            show_tour.set(false);
            #[cfg(feature = "storage")]
            let _ = LocalStorage::set("show_tour", "false");
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

    let window_height = window_height();
    let (arrow_position, dx, dy) = calculate_arrow_position(
        &selector_rect,
        TOOLTIP_WIDTH,
        TOOLTIP_HEIGHT,
        window_width(),
        window_height,
    );

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_arrow_position_for_selection_on_top() {
        let selected_rect = Rect {
            x: 200,
            y: 20,
            width: 100,
            height: 50,
        };
        let (arrow_position, dx, dy) =
            calculate_arrow_position(&selected_rect, TOOLTIP_WIDTH, TOOLTIP_HEIGHT, 600, 800);
        assert_eq!(arrow_position, "top");
        assert_eq!(dx, 0);
        assert_eq!(dy, 60);
    }

    #[test]
    fn calculate_arrow_position_for_selection_on_bottom() {
        let selected_rect = Rect {
            x: 200,
            y: 700,
            width: 100,
            height: 50,
        };
        let (arrow_position, dx, dy) =
            calculate_arrow_position(&selected_rect, TOOLTIP_WIDTH, TOOLTIP_HEIGHT, 600, 800);
        assert_eq!(arrow_position, "bottom");
        assert_eq!(dx, 0);
        assert_eq!(dy, -TOOLTIP_HEIGHT);
    }

    #[test]
    fn calculate_arrow_position_for_selection_on_left() {
        let selected_rect = Rect {
            x: 10,
            y: 200,
            width: 100,
            height: 50,
        };
        let (arrow_position, dx, dy) =
            calculate_arrow_position(&selected_rect, TOOLTIP_WIDTH, TOOLTIP_HEIGHT, 600, 800);
        assert_eq!(arrow_position, "top");
        assert_eq!(dx, 0);
        assert_eq!(dy, 60);
    }

    #[test]
    fn calculate_arrow_position_for_selection_on_right() {
        let selected_rect = Rect {
            x: 500,
            y: 200,
            width: 100,
            height: 50,
        };
        let (arrow_position, dx, dy) =
            calculate_arrow_position(&selected_rect, TOOLTIP_WIDTH, TOOLTIP_HEIGHT, 600, 800);
        assert_eq!(arrow_position, "top");
        assert_eq!(dx, 0);
        assert_eq!(dy, 60);
    }
}
