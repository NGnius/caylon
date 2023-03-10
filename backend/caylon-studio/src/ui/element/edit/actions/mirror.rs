use yew::prelude::*;

//use super::super::super::{JsonContext, JsonCtxAction};
use caylon_config::MirrorAction;

#[derive(Properties, PartialEq)]
pub struct MirrorActionProps {
    pub config: MirrorAction,
    pub callback: Callback<MirrorAction>,
}

pub struct MirrorActionComponent;

impl Component for MirrorActionComponent {
    type Message = ();
    type Properties = MirrorActionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("caylon-mirror-action-edit", "caylon-action-config")}>
            </div>
        }
    }
}
