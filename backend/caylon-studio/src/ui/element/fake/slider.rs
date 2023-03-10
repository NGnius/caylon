use yew::prelude::*;

use caylon_config::SliderConfig;

#[derive(Properties, PartialEq)]
pub struct FakeSliderProps {
    pub config: SliderConfig,
}

#[function_component]
pub fn FakeSliderComponent(props: &FakeSliderProps) -> Html {
    html! {
        <div class={classes!("fake-slider")}>
            <span class={classes!("fake-slider-title")}>{props.config.title.clone()}</span>
            <input type="range" min={props.config.min.to_string()} max={props.config.max.to_string()} value={0} class={classes!("fake-slider-input")} />
        </div>
    }
}
