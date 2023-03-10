use yew::prelude::*;

//use super::super::super::{JsonContext, JsonCtxAction};
use super::super::AlwaysStringComponent;
use caylon_config::JsonAction;

#[derive(Properties, PartialEq)]
pub struct JsonActionProps {
    pub config: JsonAction,
    pub callback: Callback<JsonAction>,
}

pub struct JsonActionComponent;

impl Component for JsonActionComponent {
    type Message = ();
    type Properties = JsonActionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        //props.callback.emit(ElementMessage::NoOp);
        let cb = props.callback.clone();
        let config = props.config.clone();
        html! {
            <div class={classes!("caylon-json-action-edit", "caylon-action-config")}>
                <AlwaysStringComponent
                    title={"JMESPath"}
                    value={props.config.jmespath.clone()}
                    callback={ctx.link().callback(move |run: String| {
                        let mut new_conf = config.clone();
                        new_conf.jmespath = run;
                        cb.emit(new_conf)
                    })}
                />
            </div>
        }
    }
}
