use yew::prelude::*;

#[function_component]
pub fn TitleComponent() -> Html {
    log::debug!("Header render");
    html! {
        <div class={classes!("header")}>
            <span class={classes!("header-elem")}>
                <h1>{"Caylon Studio"}</h1>
            </span>
        </div>
    }
}
