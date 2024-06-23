use yew::prelude::*;

#[derive(Debug)]
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

pub fn selection_rect(selector: &str) -> Result<Rect, String> {
    let window = web_sys::window().ok_or_else(|| "Failed to get window".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "Failed to get document".to_string())?;

    if let Some(element) = document.query_selector(selector).map_err(|e| {
        e.as_string()
            .unwrap_or_else(|| "Failed to query selector".to_string())
    })? {
        let rect = element.get_bounding_client_rect();
        let scroll_x = window.scroll_x().map_err(|e| {
            e.as_string()
                .unwrap_or_else(|| "Failed to get scroll_x".to_string())
        })?;
        let scroll_y = window.scroll_y().map_err(|e| {
            e.as_string()
                .unwrap_or_else(|| "Failed to get scroll_y".to_string())
        })?;

        let top = rect.top() + scroll_y;
        let left = rect.left() + scroll_x;
        let height = rect.height();
        let width = rect.width();

        Ok(Rect {
            x: left as i32,
            y: top as i32,
            width: width as i32,
            height: height as i32,
        })
    } else {
        Err("Element not found".to_string())
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[function_component(Selection)]
pub fn selection(props: &Props) -> Html {
    html! {
        <div class="introjs-helperLayer"
            style={format!("position: absolute; top: {}px; left: {}px; width: {}px; height: {}px;\
             box-shadow: rgba(33, 33, 33, 0.8) 0px 0px 1px 2px, rgba(33, 33, 33, 0.5) 0px 0px 0px 5000px; opacity: 1;",
                props.y, props.x, props.width, props.height)}>
        </div>
    }
}
