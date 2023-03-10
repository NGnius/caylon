use yew::prelude::*;

use super::super::{JsonContext, JsonCtxAction};

#[derive(Properties, PartialEq)]
pub struct RemoveElementProps {
    pub index: usize,
}


#[function_component]
pub fn RemoveElementComponent(props: &RemoveElementProps) -> Html {
    let json_ctx = use_context::<JsonContext>().expect("Missing JSON context");
    let index = props.index;
    html! {
        <div class={classes!("remove-element")}>
            <button onclick={Callback::from(move |_| json_ctx.dispatch(JsonCtxAction::RemoveElement { index }))} class={classes!("remove-element-button")}>
                    { "-" }
                </button>
        </div>
    }
}
