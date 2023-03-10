use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlwaysUsizeProps {
    pub title: Option<&'static str>,
    pub value: usize,
    pub callback: Callback<usize>,
}

pub struct AlwaysUsizeComponent;

impl Component for AlwaysUsizeComponent {
    type Message = ();
    type Properties = AlwaysUsizeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let callback = props.callback.clone();
        html! {
            <div class={classes!("caylon-always-usize-edit", "caylon-editor")}>
                {props.title.map(|t| html!{ <label class={classes!("caylon-label-edit")}>{t}</label>})}
                <input type="number"
                    value={props.value.to_string()}
                    onchange={ctx.link().callback(move |#[allow(unused_variables)] event: Event| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            log::debug!("Always usize input change");
                            let elem = event.target_unchecked_into::<web_sys::HtmlInputElement>();
                            let result: Result<usize, _> = elem.value().parse();
                            match result {
                                Ok(value) => callback.emit(value),
                                Err(e) => log::warn!("Failed to parse always usize: {}", e),
                            }

                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            callback.emit(usize::MAX)
                        }
                    })}
                    class={classes!("caylon-always-usize-input", "caylon-input-editor")}
                />
            </div>
        }
    }
}
