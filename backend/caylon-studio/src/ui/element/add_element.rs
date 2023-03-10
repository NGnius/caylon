use yew::prelude::*;

use super::super::{JsonContext, JsonCtxAction};

#[derive(Properties, PartialEq)]
pub struct AddElementProps {
    pub json_ctx: JsonContext,
}

pub enum AddElementMsg {
    SelectCb(SelectedElementType),
    AddClick,
    ReDraw,
    NoOp,
}

#[derive(PartialEq, Eq)]
pub enum SelectedElementType {
    Button,
    Toggle,
    Slider,
    ReadingDisplay,
    ResultDisplay,
    EventDisplay,
}

pub struct AddElementComponent {
    selected: SelectedElementType,
}

impl Component for AddElementComponent {
    type Message = AddElementMsg;
    type Properties = AddElementProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected: SelectedElementType::EventDisplay, // should be last <option> in <select>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AddElementMsg::SelectCb(elem) => {
                self.selected = elem;
                false
            }
            AddElementMsg::AddClick => {
                let next_index = ctx.props().json_ctx.json.items().len();
                let item = match &self.selected {
                    SelectedElementType::Button => {
                        caylon_config::ElementConfig::Button(caylon_config::ButtonConfig {
                            title: format!("Element {}", next_index),
                            on_click: default_top_level_action(),
                        })
                    }
                    SelectedElementType::Toggle => {
                        caylon_config::ElementConfig::Toggle(caylon_config::ToggleConfig {
                            title: format!("Element {}", next_index),
                            description: None, // TODO
                            on_toggle: default_top_level_action(),
                        })
                    }
                    SelectedElementType::Slider => {
                        caylon_config::ElementConfig::Slider(caylon_config::SliderConfig {
                            title: format!("Element {}", next_index),
                            min: 0,
                            max: 10,
                            notches: None,
                            on_set: default_top_level_action(),
                        })
                    }
                    SelectedElementType::ReadingDisplay => {
                        caylon_config::ElementConfig::ReadingDisplay(caylon_config::ReadingConfig {
                            title: format!("Element {}", next_index),
                            period_ms: None,
                            on_period: default_top_level_action(),
                        })
                    }
                    SelectedElementType::ResultDisplay => {
                        caylon_config::ElementConfig::ResultDisplay(
                            caylon_config::ResultDisplayConfig {
                                title: format!("Element {}", next_index),
                                result_of: 0,
                            },
                        )
                    }
                    SelectedElementType::EventDisplay => {
                        caylon_config::ElementConfig::EventDisplay(
                            caylon_config::EventDisplayConfig {
                                title: format!("Element {}", next_index),
                                event: caylon_config::EventType::GameStart,
                                on_event: default_top_level_action(),
                            },
                        )
                    }
                };
                ctx.props().json_ctx.dispatch(JsonCtxAction::InsertElement {
                    index: next_index,
                    item,
                });
                true
            }
            AddElementMsg::NoOp => false,
            AddElementMsg::ReDraw => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::debug!("Add element rendered");
        let cb = ctx.link().callback(|#[allow(unused_variables)] event: Event| {
            #[cfg(target_arch = "wasm32")]
            {
                log::info!("Element dropdown select");
                let elem = event.target_unchecked_into::<web_sys::HtmlSelectElement>();
                let new_item = match &elem.value() as &str {
                    "button" => SelectedElementType::Button,
                    "toggle" => SelectedElementType::Toggle,
                    "slider" => SelectedElementType::Slider,
                    "reading-display" => SelectedElementType::ReadingDisplay,
                    "result-display" => SelectedElementType::ResultDisplay,
                    "event-display" => SelectedElementType::EventDisplay,
                    _ => SelectedElementType::ReadingDisplay,
                };
                AddElementMsg::SelectCb(new_item)
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                AddElementMsg::NoOp
            }
        });
        html! {
            <div class={classes!("add-element")} onload={ctx.link().callback(|_| {
                    log::info!("Add element dropdown loaded, redrawing");
                    AddElementMsg::ReDraw
                })}>
                <label>{"Add a new... "}</label>
                <select onchange={cb} autocomplete={"off"}>
                    <option value={"button"}
                        selected={self.selected == SelectedElementType::Button}
                    >
                        {"Button"}
                    </option>
                    <option value={"toggle"}
                        selected={self.selected == SelectedElementType::Toggle}
                    >
                        {"Toggle"}
                    </option>
                    <option value={"slider"}
                        selected={self.selected == SelectedElementType::Slider}
                    >
                        {"Slider"}
                    </option>
                    <option value={"reading-display"}
                        selected={self.selected == SelectedElementType::ReadingDisplay}
                    >
                        {"Reading Display"}
                    </option>
                    <option value={"result-display"}
                        selected={self.selected == SelectedElementType::ResultDisplay}
                    >
                        {"Result Display"}
                    </option>
                    <option value={"event-display"}
                        selected={self.selected == SelectedElementType::EventDisplay}
                    >
                        {"Event Display"}
                    </option>
                </select>
                <button onclick={ctx.link()
                            .callback(
                                |_| AddElementMsg::AddClick
                            )} class={classes!("add-element-button")}>
                    { "+" }
                </button>
            </div>
        }
    }
}

fn default_top_level_action() -> caylon_config::TopLevelActionConfig {
    caylon_config::TopLevelActionConfig::Sequence(caylon_config::SequenceAction {
        steps: vec![
            caylon_config::ActionConfig::Command(caylon_config::CommandAction {
                run: "echo \"Hello world!\"".to_owned(),
            }),
        ]
    })
}

/*fn default_action() -> caylon_config::ActionConfig {
    caylon_config::ActionConfig::Command(
        caylon_config::CommandAction { run: "echo \"Hello world!\"".to_owned() }
    )
}*/
