use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OptionStringProps {
    pub title: Option<&'static str>,
    pub value: Option<String>,
    pub callback: Callback<Option<String>>,
}

pub struct OptionStringComponent;

impl Component for OptionStringComponent {
    type Message = ();
    type Properties = OptionStringProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let callback = props.callback.clone();
        html! {
            <div class={classes!("caylon-option-str-edit", "caylon-editor")}>
                {props.title.map(|t| html!{ <label class={classes!("caylon-label-edit")}>{t}</label>})}
                <input type="text"
                    value={props.value.clone().unwrap_or("".into())}
                    onchange={ctx.link().callback(move |#[allow(unused_variables)] event: Event| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            log::debug!("Option str input change");
                            let elem = event.target_unchecked_into::<web_sys::HtmlInputElement>();
                            let value = elem.value();
                            callback.emit(if value.is_empty() { None } else { Some(value) });
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            callback.emit(Some("[UNREACHABLE]".into()))
                        }
                    })}
                    class={classes!("caylon-option-str-input", "caylon-input-editor")}
                />
            </div>
        }
    }
}
