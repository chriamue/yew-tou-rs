use crate::models::Rect;
use crate::tour::ARROW_SIZE;

/// Calculates the best position for the tooltip relative to the selected element.
///
/// This function determines the optimal position (top, bottom, left, right) for the tooltip,
/// ensuring it stays within the window boundaries and chooses the side with the most available space.
///
/// # Parameters
///
/// - `selected_rect`: The rectangle representing the selected element.
/// - `tooltip_width`: The width of the tooltip.
/// - `tooltip_height`: The height of the tooltip.
/// - `window_width`: The width of the browser window.
/// - `window_height`: The height of the browser window.
///
/// # Returns
///
/// A tuple containing:
/// - `arrow_position`: A string indicating where the arrow should point (e.g., "top", "bottom").
/// - `x`: The x-coordinate for positioning the tooltip.
/// - `y`: The y-coordinate for positioning the tooltip.
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

    // Determine if the tooltip can fit on each side
    let can_place_top = top_space >= tooltip_height + ARROW_SIZE;
    let can_place_bottom = bottom_space >= tooltip_height + ARROW_SIZE;
    let can_place_left = left_space >= tooltip_width + ARROW_SIZE;
    let can_place_right = right_space >= tooltip_width + ARROW_SIZE;

    // Define preferred order of positions
    let preferred_order = ["bottom", "top", "left", "right"];

    // Create a list of possible positions where the tooltip fits
    let mut possible_positions = Vec::new();

    if can_place_bottom {
        possible_positions.push((
            "top",
            bottom_space,
            0, // Index for ordering
            selected_rect.left() + (selected_rect.width - tooltip_width) / 2,
            selected_rect.bottom() + ARROW_SIZE,
        ));
    }
    if can_place_top {
        possible_positions.push((
            "bottom",
            top_space,
            1,
            selected_rect.left() + (selected_rect.width - tooltip_width) / 2,
            selected_rect.top() - tooltip_height - ARROW_SIZE,
        ));
    }
    if can_place_right {
        possible_positions.push((
            "left",
            right_space,
            2,
            selected_rect.right() + ARROW_SIZE,
            selected_rect.top() + (selected_rect.height - tooltip_height) / 2,
        ));
    }
    if can_place_left {
        possible_positions.push((
            "right",
            left_space,
            3,
            selected_rect.left() - tooltip_width - ARROW_SIZE,
            selected_rect.top() + (selected_rect.height - tooltip_height) / 2,
        ));
    }

    // Choose the position with the most space and highest priority
    let (arrow_position, _, _, mut x_pos, mut y_pos) = if let Some(position) = possible_positions
        .into_iter()
        .max_by(|a, b| {
            let space_cmp = a.1.cmp(&b.1);
            if space_cmp == std::cmp::Ordering::Equal {
                // Compare based on preferred order
                a.2.cmp(&b.2)
            } else {
                space_cmp
            }
        })
    {
        position
    } else {
        // Default to placing below the element
        (
            "top",
            bottom_space,
            0,
            selected_rect.left() + (selected_rect.width - tooltip_width) / 2,
            selected_rect.bottom() + ARROW_SIZE,
        )
    };

    // Adjust position to keep the tooltip within window boundaries
    if x_pos < 0 {
        x_pos = 0;
    } else if x_pos + tooltip_width > window_width {
        x_pos = window_width - tooltip_width;
    }

    if y_pos < 0 {
        y_pos = 0;
    } else if y_pos + tooltip_height > window_height {
        y_pos = window_height - tooltip_height;
    }

    (arrow_position, x_pos, y_pos)
}


#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    const TOOLTIP_WIDTH: i32 = 100;
    const TOOLTIP_HEIGHT: i32 = 100;

    const DEFAULT_WINDOW_WIDTH: i32 = 800;
    const DEFAULT_WINDOW_HEIGHT: i32 = 600;

    const ARROW_SIZE: i32 = 10;

    #[rstest]
    #[case("Element at top", Rect { x: 200, y: 20, width: 100, height: 50 }, "top", 200, 80)]
    #[case("Element at bottom", Rect { x: 200, y: 500, width: 100, height: 50 }, "bottom", 200, 390)]
    #[case("Element at left edge", Rect { x: 0, y: 200, width: 100, height: 50 }, "left", 110, 175)]
    #[case("Element at right edge", Rect { x: 700, y: 200, width: 100, height: 50 }, "right", 590, 175)]
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

        // Add debug output
        println!(
            "Test '{}': Expected arrow_position='{}', dx={}, dy={}. Got arrow_position='{}', dx={}, dy={}",
            name, expected_arrow, expected_dx, expected_dy, arrow_position, dx, dy
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
    #[case("Small tooltip", Rect { x: 200, y: 200, width: 100, height: 50 }, 200, 100, "left", 310, 175)]
    #[case("Element larger than tooltip", Rect { x: 200, y: 200, width: 400, height: 50 }, TOOLTIP_WIDTH, TOOLTIP_HEIGHT, "top", 350, 260)]
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
    #[case("Element at (0, 0)", Rect { x: 0, y: 0, width: 50, height: 50 }, "right", 0, 0)]
    #[case("Element at bottom right corner", Rect { x: 750, y: 550, width: 50, height: 50 }, "left", 690, 475)]
    #[case("Element larger than window", Rect { x: -100, y: -100, width: 1000, height: 1000 }, "top", 0, 0)]
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
