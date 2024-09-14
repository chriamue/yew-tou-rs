use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub current: usize,
    pub total: usize,
    pub on_click: Callback<usize>,
}

#[function_component(Progress)]
pub fn progress(props: &Props) -> Html {
    html! {
        <div class="introjs-bullets">
            <ul role="tablist">
                {for (0..props.total).map(|i| {
                    let is_current = i == props.current;
                    let on_click = {
                        let on_click = props.on_click.clone();
                        Callback::from(move |_| on_click.emit(i))
                    };
                    html! {
                        <li role="presentation">
                            <a role="button" data-step-number={format!("{}", i)} onclick={on_click} class={if is_current { "active" } else { "" }} >
                                {" "}
                            </a>
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
