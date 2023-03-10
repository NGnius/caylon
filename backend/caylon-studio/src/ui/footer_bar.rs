use yew::prelude::*;

#[function_component]
pub fn FooterComponent() -> Html {
    log::debug!("Footer render");
    html! {
        <div class={classes!("footer")}>
            <span class={classes!("footer-elem")}>
                {"Javascript required (though it's mostly WASM)"}
            </span>
            <span class={classes!("footer-elem")}>
                {" Made for "}
                <a href={"https://github.com/NGnius/caylon"}>{"Caylon"}</a>
                {" by "}
                <a href={"http://ngni.us"}>{"NGnius"}</a>
            </span>
            <span class={classes!("footer-elem")}>
                <a href={"https://liberapay.com/NGnius"}>{"Donate"}</a>
            </span>
            <span class={classes!("footer-elem")}>
                <Suspense fallback={super::hit_counter::fallback()}>
                    <super::HitCounterComponent />
                </Suspense>
            </span>
        </div>
    }
}
