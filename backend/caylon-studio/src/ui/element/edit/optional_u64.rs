use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OptionU64Props {
    pub title: Option<&'static str>,
    pub value: Option<u64>,
    pub callback: Callback<Option<u64>>,
}

pub struct OptionU64Component;

impl Component for OptionU64Component {
    type Message = ();
    type Properties = OptionU64Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let callback = props.callback.clone();
        html! {
            <div class={classes!("caylon-option-u64-edit", "caylon-editor")}>
                {props.title.map(|t| html!{ <label class={classes!("caylon-label-edit")}>{t}</label>})}
                <input type="number"
                    value={props.value.unwrap_or(0).to_string()}
                    onchange={ctx.link().callback(move |#[allow(unused_variables)] event: Event| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            log::debug!("Option u64 input change");
                            let elem = event.target_unchecked_into::<web_sys::HtmlInputElement>();
                            let value: Option<u64> = elem.value().parse().ok();
                            callback.emit(value);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            callback.emit(Some(u64::MAX))
                        }
                    })}
                    class={classes!("caylon-option-u64-input", "caylon-input-editor")}
                />
            </div>
        }
    }
}
