use yew::prelude::*;

use caylon_config::{ElementConfig, TopLevelActionConfig};

use std::rc::Rc;

pub enum ElementMessage {
    SetTitle(String),
    SetDescription(Option<String>),
    SetAction(TopLevelActionConfig),
    SetPeriod(Option<u64>),
    SetResultOf(usize),
    NoOp,
}

// unused (it's a bad idea)

#[derive(PartialEq)]
pub struct ElementCtx(ElementConfig);

pub type ElementContext = UseReducerHandle<ElementCtx>;

impl Eq for ElementCtx {}

impl ElementCtx {
    pub fn init(element: ElementConfig) -> Self {
        Self(element)
    }
}

impl Reducible for ElementCtx {
    type Action = ElementMessage;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let new_config = match self.0.clone() {
            ElementConfig::Button(mut button) => {
                match action {
                    ElementMessage::SetTitle(title) => button.title = title,
                    ElementMessage::SetDescription(_) => {},
                    ElementMessage::SetAction(action) => button.on_click = action,
                    ElementMessage::SetPeriod(_) => {},
                    ElementMessage::SetResultOf(_) => {},
                    ElementMessage::NoOp => {},
                }
                ElementConfig::Button(button)
            },
            ElementConfig::Toggle(mut toggle) => {
                match action {
                    ElementMessage::SetTitle(title) => toggle.title = title,
                    ElementMessage::SetDescription(desc) => toggle.description = desc,
                    ElementMessage::SetAction(action) => toggle.on_toggle = action,
                    ElementMessage::SetPeriod(_) => {},
                    ElementMessage::SetResultOf(_) => {},
                    ElementMessage::NoOp => {},
                }
                ElementConfig::Toggle(toggle)
            },
            ElementConfig::Slider(mut slider) => {
                match action {
                    ElementMessage::SetTitle(title) => slider.title = title,
                    ElementMessage::SetDescription(_) => {},
                    ElementMessage::SetAction(action) => slider.on_set = action,
                    ElementMessage::SetPeriod(_) => {},
                    ElementMessage::SetResultOf(_) => {},
                    ElementMessage::NoOp => {},
                }
                ElementConfig::Slider(slider)
            },
            ElementConfig::ReadingDisplay(mut disp) => {
                match action {
                    ElementMessage::SetTitle(title) => disp.title = title,
                    ElementMessage::SetDescription(_) => {},
                    ElementMessage::SetAction(action) => disp.on_period = action,
                    ElementMessage::SetPeriod(period) => disp.period_ms = period,
                    ElementMessage::SetResultOf(_) => {},
                    ElementMessage::NoOp => {},
                }
                ElementConfig::ReadingDisplay(disp)
            },
            ElementConfig::ResultDisplay(mut disp) => {
                match action {
                    ElementMessage::SetTitle(title) => disp.title = title,
                    ElementMessage::SetDescription(_) => {},
                    ElementMessage::SetAction(_) => {},
                    ElementMessage::SetPeriod(_) => {},
                    ElementMessage::SetResultOf(result) => disp.result_of = result,
                    ElementMessage::NoOp => {},
                }
                ElementConfig::ResultDisplay(disp)
            },
            ElementConfig::EventDisplay(mut disp) => {
                match action {
                    ElementMessage::SetTitle(title) => disp.title = title,
                    ElementMessage::SetDescription(_) => {},
                    ElementMessage::SetAction(action) => disp.on_event = action,
                    ElementMessage::SetPeriod(_) => {},
                    ElementMessage::SetResultOf(_) => {},
                    ElementMessage::NoOp => {},
                }
                ElementConfig::EventDisplay(disp)
            },
        };
        Rc::new(Self::init(new_config))
    }
}
