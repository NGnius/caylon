use yew::prelude::*;

//use caylon_config::DisplayConfig;

#[derive(Properties, PartialEq)]
pub struct FakeDisplayProps {
    pub title: String,
    pub content: Option<String>,
}

#[function_component]
pub fn FakeDisplayComponent(props: &FakeDisplayProps) -> Html {
    html! {
         <div class={classes!("fake-display")}>
            <span class={classes!("fake-display-title")}>{props.title.clone()}</span>
            <span class={classes!("fake-display-content")}>{props.content.clone().unwrap_or_else(|| "[info]".into())}</span>
         </div>
    }
}
