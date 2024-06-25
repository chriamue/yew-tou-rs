use yew::prelude::*;

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
