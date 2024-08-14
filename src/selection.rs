use yew::prelude::*;
use crate::rect::Rect;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub rect: Rect
}

#[function_component(Selection)]
pub fn selection(props: &Props) -> Html {
    html! {
        <div class="introjs-helperLayer"
            style={format!("position: absolute; top: {}px; left: {}px; width: {}px; height: {}px;\
             box-shadow: rgba(33, 33, 33, 0.8) 0px 0px 1px 2px, rgba(33, 33, 33, 0.5) 0px 0px 0px 5000px; opacity: 1;",
                props.rect.top(), props.rect.left(), props.rect.width, props.rect.height)}>
        </div>
    }
}
