use crate::config::TourConfig;
use crate::content::Content;
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
            <Selection x={selector_rect.x} y={selector_rect.y} width={selector_rect.width} height={selector_rect.height} />
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

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    const DEFAULT_WINDOW_WIDTH: i32 = 800;
    const DEFAULT_WINDOW_HEIGHT: i32 = 600;

    #[rstest]
    #[case("Element on top", Rect { x: 200, y: 20, width: 100, height: 50 }, "top", 0, 60)]
    #[case("Element on bottom", Rect { x: 200, y: 500, width: 100, height: 50 }, "bottom", 0, -TOOLTIP_HEIGHT)]
    #[case("Element on left edge", Rect { x: 0, y: 200, width: 100, height: 50 }, "top", 0, 60)]
    #[case("Element on right edge", Rect { x: 700, y: 200, width: 100, height: 50 }, "top", 0, 60)]
    fn test_calculate_arrow_position(
        #[case] name: &str,
        #[case] rect: Rect,
        #[case] expected_arrow: &str,
        #[case] expected_dx: i32,
        #[case] expected_dy: i32,
    ) {
        let (arrow_position, dx, dy) = calculate_arrow_position(
            &rect,
            TOOLTIP_WIDTH,
            TOOLTIP_HEIGHT,
            DEFAULT_WINDOW_WIDTH,
            DEFAULT_WINDOW_HEIGHT,
        );

        assert_eq!(
            arrow_position, expected_arrow,
            "Arrow position mismatch for {}",
            name
        );
        assert_eq!(dx, expected_dx, "DX mismatch for {}", name);
        assert_eq!(dy, expected_dy, "DY mismatch for {}", name);
    }

    #[rstest]
    #[case("Small tooltip", Rect { x: 200, y: 200, width: 100, height: 50 }, 200, 100, "top", 0, 60)]
    #[case("Element larger than tooltip", Rect { x: 200, y: 200, width: 400, height: 50 }, TOOLTIP_WIDTH, TOOLTIP_HEIGHT, "top", 50, 60)]
    fn test_calculate_arrow_position_with_different_sizes(
        #[case] name: &str,
        #[case] rect: Rect,
        #[case] tooltip_width: i32,
        #[case] tooltip_height: i32,
        #[case] expected_arrow: &str,
        #[case] expected_dx: i32,
        #[case] expected_dy: i32,
    ) {
        let (arrow_position, dx, dy) = calculate_arrow_position(
            &rect,
            tooltip_width,
            tooltip_height,
            DEFAULT_WINDOW_WIDTH,
            DEFAULT_WINDOW_HEIGHT,
        );

        assert_eq!(
            arrow_position, expected_arrow,
            "Arrow position mismatch for {}",
            name
        );
        assert_eq!(dx, expected_dx, "DX mismatch for {}", name);
        assert_eq!(dy, expected_dy, "DY mismatch for {}", name);
    }

    #[rstest]
    #[case("Element at (0, 0)", Rect { x: 0, y: 0, width: 50, height: 50 }, "top", 0, 60)]
    #[case("Element at bottom right corner", Rect { x: 750, y: 550, width: 50, height: 50 }, "bottom", 0, -TOOLTIP_HEIGHT)]
    #[case("Element larger than window", Rect { x: -100, y: -100, width: 1000, height: 1000 }, "bottom", 350, -230)]
    fn test_edge_cases(
        #[case] name: &str,
        #[case] rect: Rect,
        #[case] expected_arrow: &str,
        #[case] expected_dx: i32,
        #[case] expected_dy: i32,
    ) {
        let (arrow_position, dx, dy) = calculate_arrow_position(
            &rect,
            TOOLTIP_WIDTH,
            TOOLTIP_HEIGHT,
            DEFAULT_WINDOW_WIDTH,
            DEFAULT_WINDOW_HEIGHT,
        );

        assert_eq!(
            arrow_position, expected_arrow,
            "Arrow position mismatch for {}",
            name
        );
        assert_eq!(dx, expected_dx, "DX mismatch for {}", name);
        assert_eq!(dy, expected_dy, "DY mismatch for {}", name);
    }
}
