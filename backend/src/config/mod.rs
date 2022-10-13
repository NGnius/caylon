mod about;
mod action;
mod base;
mod button;
mod element;
mod event_display;
mod reading;
mod result_display;
mod slider;
mod toggle;
mod transformer;

pub use about::AboutConfig;
pub use action::{TopLevelActionConfig, ActionConfig, CommandAction, MirrorAction, SequenceAction, JavascriptAction, JsonAction};
pub use base::BaseConfig;
pub use button::ButtonConfig;
pub use element::ElementConfig;
pub use event_display::{EventDisplayConfig, EventType};
pub use reading::ReadingConfig;
pub use result_display::ResultDisplayConfig;
pub use slider::SliderConfig;
pub use toggle::ToggleConfig;
pub use transformer::{TransformAction, TransformTypeAction, ReplaceTransformAction, ExpandTransformAction, LogTransformAction, LogLevel, PatternConfig};

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn dump_test() {
        let conf = BaseConfig::V0 {
            items: vec![
                ElementConfig::Button(ButtonConfig {
                    title: "Test Button".into(),
                    on_click: TopLevelActionConfig::Command(CommandAction{run: "echo 'hello button'".into()}),
                }),
                ElementConfig::Toggle(ToggleConfig {
                    title: "Test Toggle".into(),
                    description: Some("Toggle description".into()),
                    on_toggle: TopLevelActionConfig::Command(CommandAction{run: "echo 'hello toggle $KAYLON_VALUE'".into()}),
                }),
                ElementConfig::Slider(SliderConfig {
                    title: "Test Slider".into(),
                    min: 0,
                    max: 3,
                    notches: None,
                    on_set: TopLevelActionConfig::Command(CommandAction{run: "echo 'hello slider'".into()}),
                }),
                ElementConfig::ReadingDisplay(ReadingConfig {
                    title: "Test Reading".into(),
                    period_ms: Some(10000),
                    on_period: TopLevelActionConfig::Command(CommandAction{run: "echo 'hello reading'".into()})
                }),
                ElementConfig::ResultDisplay(ResultDisplayConfig {
                    title: "Test Reading".into(),
                    result_of: 1,
                }),
            ],
            about: AboutConfig {
                name: "Test name".into(),
                version: "v0.42.0".into(),
                description: "Test description".into(),
                url: Some("https://github.com/NGnius/kaylon".into()),
                authors: vec!["NGnius <ngniusness@gmail.com>".into()],
                license: Some("MIT".into()),
            },
        };
        let output = serde_json::to_string_pretty(&conf).unwrap();
        println!("JSON: {}", output);
    }
}
