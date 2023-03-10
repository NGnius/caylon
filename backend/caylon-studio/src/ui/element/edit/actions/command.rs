use yew::prelude::*;

//use super::super::super::{JsonContext, JsonCtxAction};
use super::super::AlwaysStringComponent;
use caylon_config::CommandAction;

#[derive(Properties, PartialEq)]
pub struct CommandActionProps {
    pub config: CommandAction,
    pub callback: Callback<CommandAction>,
}

pub struct CommandActionComponent;

impl Component for CommandActionComponent {
    type Message = ();
    type Properties = CommandActionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        //props.callback.emit(ElementMessage::NoOp);
        let cb = props.callback.clone();
        let config = props.config.clone();
        html! {
            <div class={classes!("caylon-command-action-edit", "caylon-action-config")}>
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
