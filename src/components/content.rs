use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub(crate) content: String,
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    let content = props.content.clone();
    #[cfg(feature = "markdown")]
    {
        let content = Html::from_html_unchecked(AttrValue::from(markdown::to_html(&content)));

        html! {
            <div class="introjs-tooltiptext">
                {content}
            </div>
        }
    }
    #[cfg(not(feature = "markdown"))]
    {
        html! {
            <div class="introjs-tooltiptext">
                {content}
            </div>
        }
    }
}
