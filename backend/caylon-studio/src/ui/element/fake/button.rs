use yew::prelude::*;

use caylon_config::ButtonConfig;

#[derive(Properties, PartialEq)]
pub struct FakeButtonProps {
    pub config: ButtonConfig,
}

#[function_component]
pub fn FakeButtonComponent(props: &FakeButtonProps) -> Html {
    html! {
        <div class={classes!("fake-button")}>
            <button type="button" class={classes!("fake-button-button")}>{props.config.title.clone()}</button>
        </div>
    }
}
