use yew::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
async fn fetch_hits() -> u64 {
    // URL only valid server-side, after the TLS reverse-proxy
    //let resp = reqwest::get("http://localhost:8080/stats/hits").await.unwrap();
    //let hits = resp.json::<u64>().await.unwrap();
    let hits = crate::api::get_hits::INDEX_HITS.load(std::sync::atomic::Ordering::Relaxed);

    hits
}

#[function_component]
pub fn HitCounterComponent() -> HtmlResult {
    let hits = use_prepared_state!(async move |_| -> u64 { fetch_hits().await }, ())?.unwrap();

    Ok(html! {
        <span class={classes!("hit-count")}>{"Hit #"}{hits}</span>
    })
}

pub fn fallback() -> Html {
    html! {
        <span class={classes!("hit-count")}>{"Hit ..."}</span>
    }
}
