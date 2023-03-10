use yew::prelude::*;

use caylon_config::ToggleConfig;

#[derive(Properties, PartialEq)]
pub struct FakeToggleProps {
    pub config: ToggleConfig,
}

#[function_component]
pub fn FakeToggleComponent(props: &FakeToggleProps) -> Html {
    html! {
        <div class={classes!("fake-toggle")}>
         <span class={classes!("fake-toggle-title")}>{props.config.title.clone()}</span>
         {props.config.description.clone().map(|desc| html! {
             <span class={classes!("fake-toggle-description")}>{desc}</span>
        })}
         <label class={classes!("fake-toggle-button")}>
            <input type={"checkbox"} />
            <span class={classes!("toggle-round")}></span>
        </label>
        </div>
    }
}
