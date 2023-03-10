use yew::prelude::*;

use super::super::{JsonContext, JsonCtxAction};
use super::ElementMessage;
use caylon_config::ResultDisplayConfig;

#[derive(Properties, PartialEq)]
pub struct ResultDisplayProps {
    pub index: usize,
    pub config: ResultDisplayConfig,
    pub json_ctx: JsonContext,
}

pub struct ResultDisplayComponent;

impl Component for ResultDisplayComponent {
    type Message = ElementMessage;
    type Properties = ResultDisplayProps;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut new_config = ctx.props().config.clone();
        let index = ctx.props().index;
        let update_needed = match msg {
            ElementMessage::SetTitle(title) => {
                new_config.title = title;
                true
            }
            ElementMessage::SetDescription(_desc) => false,
            ElementMessage::SetAction(_action) => false,
            ElementMessage::SetPeriod(_) => false,
            ElementMessage::SetResultOf(result) => {
                new_config.result_of = result;
                true
            },
            ElementMessage::NoOp => false,
            //_ => false,
        };
        if update_needed {
            ctx.props().json_ctx.dispatch(JsonCtxAction::UpdateElement {
                index,
                new_item: caylon_config::ElementConfig::ResultDisplay(new_config),
            });
        }
        update_needed
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        //let callback = ctx.link().callback(|msg: Self::Message| msg);
        html! {
            <div class={classes!("caylon-result-display", "caylon-element")}>
                // TODO editing
                <super::edit::AlwaysStringComponent
                    title={"Title"}
                    value={props.config.title.clone()}
                    callback={ctx.link().callback(|n_title: String| ElementMessage::SetTitle(n_title))}
                />
                <super::edit::AlwaysUsizeComponent
                    title={"Result Of"}
                    value={props.config.result_of}
                    callback={ctx.link().callback(|n_result| ElementMessage::SetResultOf(n_result))}
                />
                <super::fake::FakeDisplayComponent title={ctx.props().config.title.clone()} content={"[result]".to_owned()}/>
            </div>
        }
    }
}
