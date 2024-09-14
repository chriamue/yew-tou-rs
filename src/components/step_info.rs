use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: usize,
}

#[function_component(StepInfo)]
pub fn step_info(props: &Props) -> Html {
    html! {
        <div class="introjs-tooltip-step" style="
            display: flex;
            align-items: center;
            justify-content: center;
            width: 30px;
            height: 30px;
            border-radius: 50%;
            background-color: #007bff;
            color: white;
            font-weight: bold;
            font-size: 16px;
            margin-right: 10px;
        ">
            {props.value + 1}
        </div>
    }
}
