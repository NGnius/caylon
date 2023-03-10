use serde_json::to_string_pretty;
use yew::prelude::*;

#[function_component]
pub fn JsonViewComponent() -> Html {
    log::debug!("Json view render");
    let json_ctx = use_context::<super::JsonContext>().expect("Missing JSON context");
    let pretty_json = to_string_pretty(&json_ctx.json).expect("Invalid JSON");
    let line_count = pretty_json.chars().filter(|&c| c == '\n').count() + 2;
    /*html! {
        <pre>
            {pretty_json.clone()}
        </pre>
    }*/
    html! {
        <div class={classes!("json-view")}>
            <textarea class={classes!("json-input")} rows={line_count.to_string()} value={pretty_json} readonly={true}/>
        </div>
    }
}
