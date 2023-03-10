use yew::prelude::*;

use super::super::{JsonContext, JsonCtxAction};
use super::ElementMessage;
use caylon_config::ReadingConfig;

#[derive(Properties, PartialEq)]
pub struct ReadingDisplayProps {
    pub index: usize,
    pub config: ReadingConfig,
    pub json_ctx: JsonContext,
}

pub struct ReadingDisplayComponent;

impl Component for ReadingDisplayComponent {
    type Message = ElementMessage;
    type Properties = ReadingDisplayProps;

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
            ElementMessage::SetPeriod(period) => {
                new_config.period_ms = period;
                true
            },
            ElementMessage::SetResultOf(_) => false,
            ElementMessage::NoOp => false,
            //_ => false,
        };
        if update_needed {
            ctx.props().json_ctx.dispatch(JsonCtxAction::UpdateElement {
                index,
                new_item: caylon_config::ElementConfig::ReadingDisplay(new_config),
            });
        }
        update_needed
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let callback = ctx.link().callback(|msg: Self::Message| msg);
        html! {
            <div class={classes!("caylon-reading-display", "caylon-element")}>
                <super::edit::AlwaysStringComponent
                    title={"Title"}
                    value={props.config.title.clone()}
                    callback={ctx.link().callback(|n_title: String| ElementMessage::SetTitle(n_title))}
                />
                <super::edit::OptionU64Component
                    title={"Period"}
                    value={props.config.period_ms}
                    callback={ctx.link().callback(|n_period| ElementMessage::SetPeriod(n_period))}
                />
                <super::edit::ActionComponent index={props.index} config={props.config.on_period.clone()} {callback}/>
                <super::fake::FakeDisplayComponent title={ctx.props().config.title.clone()} />
            </div>
        }
    }
}
