use yew::prelude::*;

use super::element::ElementsComponent;
use super::FooterComponent;
use super::JsonViewComponent;
use super::TitleComponent;
use super::{JsonContext, JsonCtx};

#[function_component(App)]
pub fn app() -> Html {
    let json_ctx = use_reducer(JsonCtx::init);
    log::debug!("App render");
    html! {
        <main>
            <TitleComponent />
            <ContextProvider<JsonContext> context={json_ctx}>
                <div class={classes!("work-view")}>
                    <JsonViewComponent />
                    <ElementsComponent />
                </div>
                //<h1>{ "Hello World!" }</h1>
            </ContextProvider<JsonContext>>
            <FooterComponent />
        </main>
    }
}
