use yew::prelude::*;

//use super::super::super::{JsonContext, JsonCtxAction};
use super::super::AlwaysStringComponent;
use caylon_config::JavascriptAction;

#[derive(Properties, PartialEq)]
pub struct JavascriptActionProps {
    pub config: JavascriptAction,
    pub callback: Callback<JavascriptAction>,
}

pub struct JavascriptActionComponent;

impl Component for JavascriptActionComponent {
    type Message = ();
    type Properties = JavascriptActionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        //props.callback.emit(ElementMessage::NoOp);
        let cb = props.callback.clone();
        let config = props.config.clone();
        html! {
            <div class={classes!("caylon-javascript-action-edit", "caylon-action-config")}>
                <AlwaysStringComponent
                    title={"Run"}
                    value={props.config.run.clone()}
                    callback={ctx.link().callback(move |run: String| {
                        let mut new_conf = config.clone();
                        new_conf.run = run;
                        cb.emit(new_conf)
                    })}
                />
            </div>
        }
    }
}
