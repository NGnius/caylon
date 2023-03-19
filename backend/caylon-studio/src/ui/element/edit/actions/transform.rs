use yew::prelude::*;

//use super::super::super::{JsonContext, JsonCtxAction};
use super::super::AlwaysStringComponent;
use caylon_config::{TransformAction, TransformTypeAction};
#[cfg(target_arch = "wasm32")]
use caylon_config::PatternConfig;

#[derive(PartialEq, Eq)]
enum SelectedTransformer {
    Replace,
    Expand,
    Log,
}

fn selected(trans: &TransformTypeAction) -> SelectedTransformer {
    match trans {
        TransformTypeAction::Replace(_) => SelectedTransformer::Replace,
        TransformTypeAction::Expand(_) => SelectedTransformer::Expand,
        TransformTypeAction::Log(_) => SelectedTransformer::Log,
    }
}

#[derive(Properties, PartialEq)]
pub struct TransformActionProps {
    pub config: TransformAction,
    pub callback: Callback<TransformAction>,
}

pub struct TransformActionComponent;

impl Component for TransformActionComponent {
    type Message = ();
    type Properties = TransformActionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        //props.callback.emit(ElementMessage::NoOp);
        #[cfg(target_arch = "wasm32")]
        let cb = props.callback.clone();
        let config = props.config.clone();
        let selected = selected(&config.transformer);
        let dropdown_cb = ctx.link().callback(move |#[allow(unused_variables)] event: Event| {
            #[cfg(target_arch = "wasm32")]
            {
                log::debug!("Transformer dropdown select");
                let elem = event.target_unchecked_into::<web_sys::HtmlSelectElement>();
                let mut new_config = config.clone();
                new_config.transformer = match &elem.value() as &str {
                    "replace" => default_replace_transformer(),
                    "expand" => default_expand_transformer(),
                    "log" => default_log_transformer(),
                    _ => default_log_transformer(),
                };
                cb.emit(new_config);
            }
            #[cfg(not(target_arch = "wasm32"))]
            { }
        });
        //let config = props.config.clone();
        let editor = match &props.config.transformer {
            TransformTypeAction::Replace(rep) => {
                // TODO allow for more than one pattern
                //let moved_rep = rep.clone();
                let items = rep.patterns.iter()
                    .enumerate()
                    .map(|(index, pattern)| {
                        let cb_for_p = props.callback.clone();
                        let cb_for_f = props.callback.clone();
                        let moved_rep_p = rep.clone();
                        let moved_rep_f = rep.clone();
                        html! {
                            <div class={classes!("caylon-transformer-replace-item")}>
                                <AlwaysStringComponent
                                    title={"Regex pattern"}
                                    value={pattern.pattern.clone()}
                                    callback={ctx.link().callback(move |val: String| {
                                        let mut new_conf = moved_rep_p.clone();
                                        new_conf.patterns[index].pattern = val;
                                        cb_for_p.emit(TransformAction {
                                            transformer: TransformTypeAction::Replace(new_conf)
                                        })
                                    })}
                                />
                                <AlwaysStringComponent
                                    title={"Format"}
                                    value={pattern.pattern.clone()}
                                    callback={ctx.link().callback(move |val: String| {
                                        let mut new_conf = moved_rep_f.clone();
                                        new_conf.patterns[index].format = val;
                                        cb_for_f.emit(TransformAction {
                                            transformer: TransformTypeAction::Replace(new_conf)
                                        })
                                    })}
                                />
                            </div>
                        }
                    }).collect::<Html>();
                html! {
                    <div class={classes!("caylon-transformer-replace")}>
                        {items}
                    </div>
                }
            }
            TransformTypeAction::Expand(exp) => {
                let cb = props.callback.clone();
                let moved_exp = exp.clone();
                html! {
                    <div class={classes!("caylon-transformer-expand")}>
                        <AlwaysStringComponent
                            title={"Format"}
                            value={exp.format.clone()}
                            callback={ctx.link().callback(move |val: String| {
                                let mut new_conf = moved_exp.clone();
                                new_conf.format = val;
                                cb.emit(TransformAction {
                                    transformer: TransformTypeAction::Expand(new_conf)
                                })
                            })}
                        />
                    </div>
                }
            }
            TransformTypeAction::Log(log) => {
                #[cfg(target_arch = "wasm32")]
                let cb = props.callback.clone();
                #[cfg(target_arch = "wasm32")]
                let moved_log = log.clone();
                let log_cb = ctx.link().callback(move |#[allow(unused_variables)] event: Event| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        log::debug!("Transformer log dropdown select");
                        let elem = event.target_unchecked_into::<web_sys::HtmlSelectElement>();
                        let mut new_conf = moved_log.clone();
                        new_conf.level = match &elem.value() as &str {
                            "debug" => caylon_config::LogLevel::DEBUG,
                            "info" => caylon_config::LogLevel::INFO,
                            "warn" => caylon_config::LogLevel::WARN,
                            "error" => caylon_config::LogLevel::ERROR,
                            _ => caylon_config::LogLevel::ERROR,
                        };
                        cb.emit(TransformAction {
                            transformer: TransformTypeAction::Log(new_conf)
                        })
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    { }
                });
                html! {
                    <div class={classes!("caylon-transformer-log")}>
                        <label class={classes!("caylon-label-edit")}>{"Log level"}</label>
                        <select onchange={log_cb} autocomplete={"off"}>
                            <option value={"debug"}
                                selected={log.level == caylon_config::LogLevel::DEBUG}
                            >
                                {"Debug"}
                            </option>
                            <option value={"info"}
                                selected={log.level == caylon_config::LogLevel::INFO}
                            >
                                {"Info"}
                            </option>
                            <option value={"warn"}
                                selected={log.level == caylon_config::LogLevel::WARN}
                            >
                                {"Warn"}
                            </option><option value={"error"}
                                selected={log.level == caylon_config::LogLevel::ERROR}
                            >
                                {"Error"}
                            </option>
                        </select>
                    </div>
                }
            }
        };
        html! {
            <div class={classes!("caylon-transformer-action-edit", "caylon-action-config")}>
                <label class={classes!("caylon-label-edit")}>{"Type"}</label>
                <select onchange={dropdown_cb} autocomplete={"off"}>
                    <option value={"replace"}
                        selected={selected == SelectedTransformer::Replace}
                    >
                        {"Replace"}
                    </option>
                    <option value={"expand"}
                        selected={selected == SelectedTransformer::Expand}
                    >
                        {"Expand"}
                    </option>
                    <option value={"log"}
                        selected={selected == SelectedTransformer::Log}
                    >
                        {"Log"}
                    </option>
                </select>
                <div class={classes!("caylon-transformer-type-action-edit")}>
                    {editor}
                </div>
            </div>
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn default_replace_transformer() -> TransformTypeAction {
    TransformTypeAction::Replace(caylon_config::ReplaceTransformAction { patterns: vec![
        PatternConfig {
            pattern: "regex".to_owned(),
            format: "$1".to_owned(),
            i: None,
            m: None,
            s: None,
            u: None,
            x: None,
        }
    ] })
}

#[cfg(target_arch = "wasm32")]
fn default_expand_transformer() -> TransformTypeAction {
    TransformTypeAction::Expand(caylon_config::ExpandTransformAction { format: "$CAYLON_VALUE".into() })
}

#[cfg(target_arch = "wasm32")]
fn default_log_transformer() -> TransformTypeAction {
    TransformTypeAction::Log(caylon_config::LogTransformAction { level: caylon_config::LogLevel::INFO })
}
