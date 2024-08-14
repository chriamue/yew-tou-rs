use crate::rect::Rect;
use crate::tour::ARROW_SIZE;

pub fn calculate_arrow_position(
    selected_rect: &Rect,
    tooltip_width: i32,
    tooltip_height: i32,
    window_width: i32,
    window_height: i32,
) -> (&'static str, i32, i32) {
    let top_space = selected_rect.top();
    let bottom_space = window_height - selected_rect.bottom();
    let left_space = selected_rect.left();
    let right_space = window_width - selected_rect.right();

    let max_space = top_space.max(bottom_space).max(left_space).max(right_space);

    let (arrow_position, x, y) = if max_space == top_space {
        // Position tooltip above the selected_rect
        let x_pos = (selected_rect.width / 2) - (tooltip_width / 2);
        let y_pos = -ARROW_SIZE - tooltip_height;
        ("bottom", x_pos, y_pos)
    } else if max_space == bottom_space {
        // Position tooltip below the selected_rect
        let x_pos = (selected_rect.width / 2) - (tooltip_width / 2);
        let y_pos = selected_rect.height + ARROW_SIZE;
        ("top", x_pos, y_pos)
    } else if max_space == left_space {
        // Position tooltip to the left of the selected_rect
        let x_pos = -tooltip_width - ARROW_SIZE;
        let y_pos = (selected_rect.height / 2) - (tooltip_height / 2);
        ("right", x_pos, y_pos)
    } else {
        // Position tooltip to the right of the selected_rect
        let x_pos = selected_rect.width + ARROW_SIZE;
        let y_pos = (selected_rect.height / 2) - (tooltip_height / 2);
        ("left", x_pos, y_pos)
    };

    (arrow_position, x, y)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    const TOOLTIP_WIDTH: i32 = 100;
    const TOOLTIP_HEIGHT: i32 = 100;

    const DEFAULT_WINDOW_WIDTH: i32 = 800;
    const DEFAULT_WINDOW_HEIGHT: i32 = 600;

    #[rstest]
    #[case("Element on top", Rect { x: 200, y: 20, width: 100, height: 50 }, "top", 0, 50 + ARROW_SIZE)]
    #[case("Element on bottom", Rect { x: 200, y: 500, width: 100, height: 50 }, "bottom", 0, -ARROW_SIZE - TOOLTIP_HEIGHT)]
    #[case("Element on left edge", Rect { x: 0, y: 200, width: 100, height: 50 }, "left", 100 + ARROW_SIZE, -25)]
    #[case("Element on right edge", Rect { x: 700, y: 200, width: 100, height: 50 }, "right", -ARROW_SIZE -TOOLTIP_WIDTH, -25)]
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
    #[case("Small tooltip", Rect { x: 200, y: 200, width: 100, height: 50 }, 200, 100, "left", 110, -25)]
    #[case("Element larger than tooltip", Rect { x: 200, y: 200, width: 400, height: 50 }, TOOLTIP_WIDTH, TOOLTIP_HEIGHT, "top", 150, 60)]
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
    #[case("Element at (0, 0)", Rect { x: 0, y: 0, width: 50, height: 50 }, "left", 60, -25)]
    #[case("Element at bottom right corner", Rect { x: 750, y: 550, width: 50, height: 50 }, "right", -TOOLTIP_WIDTH - ARROW_SIZE, -25)]
    #[case("Element larger than window", Rect { x: -100, y: -100, width: 1000, height: 1000 }, "bottom", 450, -110)]
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
