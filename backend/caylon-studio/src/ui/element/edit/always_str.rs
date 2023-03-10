use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlwaysStringProps {
    pub title: Option<&'static str>,
    pub value: String,
    pub callback: Callback<String>,
}

pub struct AlwaysStringComponent;

impl Component for AlwaysStringComponent {
    type Message = ();
    type Properties = AlwaysStringProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let callback = props.callback.clone();
        html! {
            <div class={classes!("caylon-always-str-edit", "caylon-editor")}>
                {props.title.map(|t| html!{ <label class={classes!("caylon-label-edit")}>{t}</label>})}
                <input type="text"
                    value={props.value.clone()}
                    onchange={ctx.link().callback(move |#[allow(unused_variables)] event: Event| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            log::debug!("Always str input change");
                            let elem = event.target_unchecked_into::<web_sys::HtmlInputElement>();
                            callback.emit(elem.value());
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            callback.emit("[UNREACHABLE]".into())
                        }
                    })}
                    class={classes!("caylon-always-str-input", "caylon-input-editor")}
                />
            </div>
        }
    }
}
