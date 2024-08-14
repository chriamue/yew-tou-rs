use crate::rect::Rect;

pub fn calculate_arrow_position(
    selected_rect: &Rect,
    tooltip_width: i32,
    tooltip_height: i32,
    window_width: i32,
    window_height: i32,
) -> (&'static str, i32, i32) {
    let arrow_position =
        if selected_rect.y + selected_rect.height + tooltip_height + crate::tour::ARROW_SIZE
            > window_height
        {
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
        selected_rect.height + crate::tour::ARROW_SIZE
    };

    (arrow_position, dx, dy)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tour::TOOLTIP_HEIGHT;
    use crate::tour::TOOLTIP_WIDTH;
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
            crate::tour::TOOLTIP_WIDTH,
            crate::tour::TOOLTIP_HEIGHT,
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
            crate::tour::TOOLTIP_WIDTH,
            crate::tour::TOOLTIP_HEIGHT,
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
