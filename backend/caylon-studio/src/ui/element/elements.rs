use yew::prelude::*;

use super::super::JsonContext;
use caylon_config::ElementConfig;

#[function_component]
pub fn ElementsComponent() -> Html {
    log::debug!("Elements render");
    let json_ctx = use_context::<JsonContext>().expect("Missing JSON context");
    let elem_view = if json_ctx.json.items().is_empty() {
        html! {
            {"--- No elements ---"}
        }
    } else {
        let elements_html = json_ctx
            .json
            .items()
            .iter()
            .enumerate()
            .map(|(i, elem)| element_impl(i, elem, &json_ctx))
            .collect::<Html>();
        html! {
            {elements_html}
        }
    };
    html! {
            <div class={classes!("elements-view")}>
                <div class={classes!("elements-toolbar")}>
                    <super::AddElementComponent {json_ctx}/>
                </div>
                <div class={classes!("elements-list")}>
                    {elem_view}
                </div>
            </div>
    }
}

fn element_impl(index: usize, elem: &ElementConfig, ctx: &JsonContext) -> Html {
    let inner = match elem {
        ElementConfig::Button(button) => html! {
            <super::button::ButtonComponent {index} config={button.to_owned()} json_ctx={ctx.to_owned()}/>
        },
        ElementConfig::Toggle(toggle) => {
            html! {<super::toggle::ToggleComponent {index} config={toggle.to_owned()} json_ctx={ctx.to_owned()} />}
        }
        ElementConfig::Slider(slider) => {
            html! {<super::slider::SliderComponent {index} config={slider.to_owned()} json_ctx={ctx.to_owned()} />}
        }
        ElementConfig::ReadingDisplay(disp) => {
            html! {<super::reading_display::ReadingDisplayComponent {index} config={disp.to_owned()} json_ctx={ctx.to_owned()} />}
        }
        ElementConfig::ResultDisplay(disp) => {
            html! {<super::result_display::ResultDisplayComponent {index} config={disp.to_owned()} json_ctx={ctx.to_owned()} />}
        }
        ElementConfig::EventDisplay(disp) => {
            html! {<super::event_display::EventDisplayComponent {index} config={disp.to_owned()} json_ctx={ctx.to_owned()} />}
        } //_ => html!{{format!("elem #{} //TODO", index)}},
    };
    html! {
        <div class={classes!("elements-item")}>
            {inner}
            <super::RemoveElementComponent {index} />
        </div>
    }
}
