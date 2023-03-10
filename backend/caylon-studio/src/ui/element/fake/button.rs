use yew::prelude::*;

use caylon_config::ButtonConfig;

#[derive(Properties, PartialEq)]
pub struct FakeButtonProps {
    pub config: ButtonConfig,
}

#[function_component]
pub fn FakeButtonComponent(props: &FakeButtonProps) -> Html {
    html! {
         <button type="button" class={classes!("fake-button")}>{props.config.title.clone()}</button>
    }
}
