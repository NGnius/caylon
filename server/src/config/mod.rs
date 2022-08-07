mod about;
mod action;
mod base;
mod button;
mod element;
mod reading;
mod slider;
mod toggle;

pub use about::AboutConfig;
pub use action::{ActionConfig, CommandAction};
pub use base::BaseConfig;
pub use button::ButtonConfig;
pub use element::ElementConfig;
pub use reading::ReadingConfig;
pub use slider::SliderConfig;
pub use toggle::ToggleConfig;

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn dump_test() {
        let conf = BaseConfig::V0 {
            items: vec![
                ElementConfig::Button(ButtonConfig {
                    title: "Test Button".into(),
                    on_click: ActionConfig::Command(CommandAction{run: "echo 'hello button'".into()}),
                }),
                ElementConfig::Toggle(ToggleConfig {
                    title: "Test Toggle".into(),
                    description: Some("Toggle description".into()),
                    on_enable: ActionConfig::Command(CommandAction{run: "echo 'hello toggle 1'".into()}),
                    on_disable: ActionConfig::Command(CommandAction{run: "echo 'hello toggle 0'".into()}),
                }),
                ElementConfig::Slider(SliderConfig {
                    title: "Test Slider".into(),
                    min: 0,
                    max: 3,
                    notches: None,
                    on_set: ActionConfig::Command(CommandAction{run: "echo 'hello slider'".into()}),
                }),
                ElementConfig::Reading(ReadingConfig {
                    title: "Test Reading".into(),
                    period_ms: 10000,
                    on_period: ActionConfig::Command(CommandAction{run: "echo 'hello reading'".into()})
                }),
            ],
            about: AboutConfig {
                name: "Test name".into(),
                version: "v0.42.0".into(),
                description: "Test description".into(),
                url: Some("https://github.com/NGnius/kaylon".into()),
                author: Some("NGnius <ngniusness@gmail.com>".into()),
                license: Some("MIT".into()),
            },
        };
        let output = serde_json::to_string_pretty(&conf).unwrap();
        println!("JSON: {}", output);
    }
}
