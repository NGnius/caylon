use yew::prelude::*;

//use super::super::super::{JsonContext, JsonCtxAction};
use super::super::ElementMessage;
use caylon_config::{TopLevelActionConfig, ActionConfig};

#[derive(PartialEq, Eq)]
enum SelectedActionConfig {
    Command,
    Transform,
    Javascript,
    Json
}

fn selected(act: &ActionConfig) -> SelectedActionConfig {
    match act {
        ActionConfig::Command(_) => SelectedActionConfig::Command,
        ActionConfig::Transform(_) => SelectedActionConfig::Transform,
        ActionConfig::Javascript(_) => SelectedActionConfig::Javascript,
        ActionConfig::Json(_) => SelectedActionConfig::Json,
    }
}

#[derive(Properties, PartialEq)]
pub struct ActionProps {
    pub index: usize,
    pub config: TopLevelActionConfig,
    pub callback: super::EditCallback,
}

pub struct ActionComponent;

impl Component for ActionComponent {
    type Message = ();
    type Properties = ActionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let inner_action = match &props.config {
            TopLevelActionConfig::Sequence(seq) => {
                let action_items = seq.steps
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(index, step)| {
                    let selected = selected(&step);
                    let item = match step {
                        ActionConfig::Command(cmd) =>
                        {
                            let cb = props.callback.clone();
                            let config = seq.clone();
                            html! {
                                <div class={classes!("caylon-sequence-command-action", "caylon-sequence-action-edit")}>
                                    <super::actions::CommandActionComponent
                                    config={cmd.clone()}
                                    callback={
                                        ctx.link().callback(
                                            move |x| {
                                                let mut new_seq = config.clone();
                                                new_seq.steps[index] = ActionConfig::Command(x);
                                                cb.emit(ElementMessage::SetAction(TopLevelActionConfig::Sequence(new_seq)))
                                            })
                                        }
                                    />
                                </div>
                            }
                        },
                        ActionConfig::Transform(t) =>
                        {
                            let cb = props.callback.clone();
                            let config = seq.clone();
                            html! {
                                <div class={classes!("caylon-sequence-transform-action", "caylon-sequence-action-edit")}>
                                    <super::actions::TransformActionComponent
                                    config={t.clone()}
                                    callback={
                                        ctx.link().callback(
                                            move |x| {
                                                let mut new_seq = config.clone();
                                                new_seq.steps[index] = ActionConfig::Transform(x);
                                                cb.emit(ElementMessage::SetAction(TopLevelActionConfig::Sequence(new_seq)))
                                            })
                                        }
                                    />
                                </div>
                            }
                        },
                        ActionConfig::Javascript(js) =>
                        {
                            let cb = props.callback.clone();
                            let config = seq.clone();
                            html! {
                                <div class={classes!("caylon-sequence-javascript-action", "caylon-sequence-action-edit")}>
                                    <super::actions::JavascriptActionComponent
                                    config={js.clone()}
                                    callback={
                                        ctx.link().callback(
                                            move |x| {
                                                let mut new_seq = config.clone();
                                                new_seq.steps[index] = ActionConfig::Javascript(x);
                                                cb.emit(ElementMessage::SetAction(TopLevelActionConfig::Sequence(new_seq)))
                                            })
                                        }
                                    />
                                </div>
                            }
                        },
                        ActionConfig::Json(json) =>
                        {
                            let cb = props.callback.clone();
                            let config = seq.clone();
                            html! {
                                <div class={classes!("caylon-sequence-json-action", "caylon-sequence-action-edit")}>
                                    <super::actions::JsonActionComponent
                                    config={json.clone()}
                                    callback={
                                        ctx.link().callback(
                                            move |x| {
                                                let mut new_seq = config.clone();
                                                new_seq.steps[index] = ActionConfig::Json(x);
                                                cb.emit(ElementMessage::SetAction(TopLevelActionConfig::Sequence(new_seq)))
                                            })
                                        }
                                    />
                                </div>
                            }
                        },
                    };
                    #[cfg(target_arch = "wasm32")]
                    let cb = props.callback.clone();
                    #[cfg(target_arch = "wasm32")]
                    let moved_seq = seq.clone();
                    let dropdown_cb = ctx.link().callback(move |#[allow(unused_variables)] event: Event| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            log::debug!("Transformer dropdown select");
                            let elem = event.target_unchecked_into::<web_sys::HtmlSelectElement>();
                            let new_sel = match &elem.value() as &str {
                                "command" => SelectedActionConfig::Command,
                                "transform" => SelectedActionConfig::Transform,
                                "javascript" => SelectedActionConfig::Javascript,
                                "json" => SelectedActionConfig::Json,
                                _ => SelectedActionConfig::Json,
                            };
                            let new_item = default_action_config(new_sel);
                            let mut new_conf = moved_seq.clone();
                            new_conf.steps[index] = new_item;
                            cb.emit(ElementMessage::SetAction(TopLevelActionConfig::Sequence(new_conf)));
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        { }
                    });
                    html! {
                        <div class={classes!("caylon-sequence-action-edit-item")}>
                            // dropdown to change action
                            <label>{"Action"}</label>
                            <select onchange={dropdown_cb} autocomplete={"off"}>
                                <option value={"command"}
                                    selected={selected == SelectedActionConfig::Command}
                                >
                                    {"Command"}
                                </option>
                                <option value={"transform"}
                                    selected={selected == SelectedActionConfig::Transform}
                                >
                                    {"Transform"}
                                </option>
                                <option value={"javascript"}
                                    selected={selected == SelectedActionConfig::Javascript}
                                >
                                    {"Javascript"}
                                </option><option value={"json"}
                                    selected={selected == SelectedActionConfig::Json}
                                >
                                    {"JSON"}
                                </option>
                            </select>

                            {item}
                            // TODO remove button
                        </div>
                    }
                }).collect::<Html>();
                // TODO add button
                html! {
                    <div class={classes!("caylon-sequence-action-edit", "caylon-action-config")}>
                        {action_items}
                    </div>
                }
            },
            _ => html! {<span>{"//TODO"}</span>},
            /*TopLevelActionConfig::Command(cmd) =>
                html! {<super::actions::CommandActionComponent config={cmd.clone()} callback={ctx.link().callback(move |x| cb.emit(ElementMessage::SetAction(TopLevelActionConfig::Command(x))))} />},
            TopLevelActionConfig::Transform(t) => html! {<span>{"//TODO"}</span>},
            TopLevelActionConfig::Mirror(mir) => html! {<span>{"//TODO"}</span>},
            TopLevelActionConfig::Javascript(js) => html! {<span>{"//TODO"}</span>},
            TopLevelActionConfig::Json(json) => html! {<span>{"//TODO"}</span>},*/
        };
        html! {
            <div class={classes!("caylon-action-edit", "caylon-editor")}>
                // TODO editing
                {inner_action}
            </div>
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn default_action_config(selected: SelectedActionConfig) -> ActionConfig {
    match selected {
        SelectedActionConfig::Command => ActionConfig::Command(
            caylon_config::CommandAction { run: "echo \"Hello caylon world!\"".to_owned() }
        ),
        SelectedActionConfig::Transform => ActionConfig::Transform(caylon_config::TransformAction {
            transformer: caylon_config::TransformTypeAction::Log(caylon_config::LogTransformAction {
                level: caylon_config::LogLevel::INFO
            })
        }),
        SelectedActionConfig::Javascript => ActionConfig::Javascript(
            caylon_config::JavascriptAction { run: "console.log(\"Hello caylon world!\")".to_owned() }
        ),
        SelectedActionConfig::Json => ActionConfig::Json(
            caylon_config::JsonAction { jmespath: "".to_owned() }
        ),
    }
}
