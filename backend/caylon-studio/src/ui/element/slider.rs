use yew::prelude::*;

use super::super::{JsonContext, JsonCtxAction};
use super::ElementMessage;
use caylon_config::SliderConfig;

#[derive(Properties, PartialEq)]
pub struct SliderProps {
    pub index: usize,
    pub config: SliderConfig,
    pub json_ctx: JsonContext,
}

pub struct SliderComponent;

impl Component for SliderComponent {
    type Message = ElementMessage;
    type Properties = SliderProps;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut new_config = ctx.props().config.clone();
        let index = ctx.props().index;
        let update_needed = match msg {
            ElementMessage::SetTitle(title) => {
                new_config.title = title;
                true
            }
            ElementMessage::SetDescription(_desc) => false,
            ElementMessage::SetAction(action) => {
                new_config.on_set = action;
                true
            }
            ElementMessage::SetPeriod(_) => false,
            ElementMessage::SetResultOf(_) => false,
            ElementMessage::NoOp => false,
            //_ => false,
        };
        if update_needed {
            ctx.props().json_ctx.dispatch(JsonCtxAction::UpdateElement {
                index,
                new_item: caylon_config::ElementConfig::Slider(new_config),
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
            <div class={classes!("caylon-slider", "caylon-element")}>
                // TODO editing
                <super::edit::AlwaysStringComponent
                    title={"Title"}
                    value={props.config.title.clone()}
                    callback={ctx.link().callback(|n_title: String| ElementMessage::SetTitle(n_title))}
                />
                <super::edit::ActionComponent index={props.index} config={props.config.on_set.clone()} {callback}/>
                <super::fake::FakeSliderComponent config={ctx.props().config.clone()} />
            </div>
        }
    }
}
