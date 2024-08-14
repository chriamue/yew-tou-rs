use web_sys::Element;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rect {
    /// The X coordinate of the top-left corner of the rectangle.
    pub x: i32,

    /// The Y coordinate of the top-left corner of the rectangle.
    pub y: i32,

    /// The width of the rectangle.
    pub width: i32,

    /// The height of the rectangle.
    pub height: i32,
}

impl Rect {
    /// Calculates the area of the rectangle.
    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    /// Returns the leftmost X coordinate of the rectangle.
    pub fn left(&self) -> i32 {
        self.x
    }

    /// Returns the rightmost X coordinate of the rectangle.
    pub fn right(&self) -> i32 {
        self.x + self.width
    }

    /// Returns the topmost Y coordinate of the rectangle.
    pub fn top(&self) -> i32 {
        self.y
    }

    /// Returns the bottommost Y coordinate of the rectangle.
    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }

    /// Calculates the overlap area between this rectangle and another rectangle.
    pub fn overlap(&self, other: &Rect) -> i32 {
        let x_overlap = (self.right().min(other.right()) - self.left().max(other.left())).max(0);
        let y_overlap = (self.bottom().min(other.bottom()) - self.top().max(other.top())).max(0);
        x_overlap * y_overlap
    }
}

/// Rect from Tuple
impl From<(i32, i32, i32, i32)> for Rect {
    fn from(tuple: (i32, i32, i32, i32)) -> Self {
        Rect {
            x: tuple.0,
            y: tuple.1,
            width: tuple.2,
            height: tuple.3,
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        }
    }
}

pub fn get_element(selector: &str) -> Result<Element, String> {
    let document = web_sys::window()
        .ok_or_else(|| "Failed to get window".to_string())?
        .document()
        .ok_or_else(|| "Failed to get document".to_string())?;

    document
        .query_selector(selector)
        .map_err(|e| {
            e.as_string()
                .unwrap_or_else(|| "Failed to query selector".to_string())
        })
        .and_then(|element| element.ok_or_else(|| "Element not found".to_string()))
}

pub fn get_scroll_offsets() -> Result<(f64, f64), String> {
    let window = web_sys::window().ok_or_else(|| "Failed to get window".to_string())?;
    let scroll_x = window.scroll_x().map_err(|e| {
        e.as_string()
            .unwrap_or_else(|| "Failed to get scroll_x".to_string())
    })?;
    let scroll_y = window.scroll_y().map_err(|e| {
        e.as_string()
            .unwrap_or_else(|| "Failed to get scroll_y".to_string())
    })?;
    Ok((scroll_x, scroll_y))
}

pub fn get_element_rect(selector: &str) -> Result<Rect, String> {
    let element = get_element(selector)?;
    let rect = element.get_bounding_client_rect();
    let (scroll_x, scroll_y) = get_scroll_offsets()?;

    Ok(Rect {
        x: (rect.left() + scroll_x) as i32,
        y: (rect.top() + scroll_y) as i32,
        width: rect.width() as i32,
        height: rect.height() as i32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::rect1((10, 20, 30, 40), 1200)]
    #[case::rect2((0, 0, 5, 5), 25)]
    #[case::rect3((15, 25, 0, 0), 0)]
    fn test_area(#[case] rect: (i32, i32, i32, i32), #[case] expected_area: i32) {
        let rect = Rect::from(rect);
        assert_eq!(rect.area(), expected_area);
    }

    #[test]
    fn test_left_right_top_bottom() {
        let rect = Rect {
            x: 5,
            y: 10,
            width: 20,
            height: 30,
        };
        assert_eq!(rect.left(), 5);
        assert_eq!(rect.right(), 25);
        assert_eq!(rect.top(), 10);
        assert_eq!(rect.bottom(), 40);
    }

    #[test]
    fn test_overlap_no_overlap() {
        let rect1 = Rect {
            x: 10,
            y: 10,
            width: 10,
            height: 10,
        };
        let rect2 = Rect {
            x: 25,
            y: 25,
            width: 5,
            height: 5,
        };
        assert_eq!(rect1.overlap(&rect2), 0);
    }

    #[test]
    fn test_overlap_full_overlap() {
        let rect1 = Rect {
            x: 10,
            y: 10,
            width: 20,
            height: 30,
        };
        let rect2 = Rect {
            x: 5,
            y: 5,
            width: 30,
            height: 40,
        };
        assert_eq!(rect1.overlap(&rect2), rect1.area());
    }

    #[test]
    fn test_overlap_partial_overlap() {
        let rect1 = Rect {
            x: 10,
            y: 10,
            width: 20,
            height: 30,
        };
        let rect2 = Rect {
            x: 15,
            y: 5,
            width: 10,
            height: 20,
        };
        assert_eq!(rect1.overlap(&rect2), 150);
    }
}
