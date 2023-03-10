use std::rc::Rc;
use yew::prelude::*;

pub enum JsonCtxAction {
    InsertElement {
        item: caylon_config::ElementConfig,
        index: usize,
    },
    RemoveElement {
        index: usize,
    },
    UpdateElement {
        new_item: caylon_config::ElementConfig,
        index: usize,
    },
}

#[derive(Clone, PartialEq)]
pub struct JsonCtx {
    pub json: caylon_config::BaseConfig,
}

pub type JsonContext = UseReducerHandle<JsonCtx>;

impl Eq for JsonCtx {}

impl JsonCtx {
    pub fn init() -> Self {
        Self {
            json: minimal_config(),
        }
    }
}

impl Reducible for JsonCtx {
    type Action = JsonCtxAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let json_base = match action {
            JsonCtxAction::InsertElement { item, index } => {
                let mut items = self.json.items().to_owned();
                items.insert(index, item);
                caylon_config::BaseConfig::assemble(items, self.json.get_about().to_owned())
            }
            JsonCtxAction::RemoveElement { index } => {
                let mut items = self.json.items().to_owned();
                items.remove(index);
                caylon_config::BaseConfig::assemble(items, self.json.get_about().to_owned())
            }
            JsonCtxAction::UpdateElement { new_item, index } => {
                let mut items = self.json.items().to_owned();
                if items.len() > index {
                    items[index] = new_item;
                }
                caylon_config::BaseConfig::assemble(items, self.json.get_about().to_owned())
            }
        };
        Rc::new(Self { json: json_base })
    }
}

fn minimal_config() -> caylon_config::BaseConfig {
    caylon_config::BaseConfig::V0 {
        items: vec![],
        about: caylon_config::AboutConfig {
            name: env!("CARGO_PKG_NAME").to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            description: "Studio-generated UI layout for Caylon".to_owned(),
            url: Some("https://caylon.ngni.us".to_owned()),
            authors: vec!["NGnius".to_owned()],
            license: Some("MIT".to_owned()),
        },
    }
}
