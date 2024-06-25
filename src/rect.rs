use web_sys::Element;

#[derive(Debug, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
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
