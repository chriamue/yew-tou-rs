use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_prev: Callback<()>,
    pub on_next: Callback<()>,
}

#[function_component(Navigation)]
pub fn navigation(props: &Props) -> Html {
    let on_prev = {
        let on_prev = props.on_prev.clone();
        Callback::from(move |_| on_prev.emit(()))
    };

    let on_next = {
        let on_next = props.on_next.clone();
        Callback::from(move |_| on_next.emit(()))
    };

    html! {
        <div class="introjs-tooltipbuttons">
            <button class="introjs-button introjs-prevbutton" onclick={on_prev}>{"Prev"}</button>
            <button class="introjs-button introjs-nextbutton" onclick={on_next}>{"Next"}</button>
            <div class="introjs-tooltipbuttons::after"></div>
        </div>
    }
}
