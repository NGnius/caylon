use usdpl_back::core::serdes::Primitive;

use jmespath::{Expression, Variable};

use crate::config::JsonAction;
use super::{SeqAct, ActError};

pub struct JsonActor {
    expr: Expression<'static>,
}

impl JsonActor {
    fn jmespath_value_to_primitive(var: Variable) -> Primitive {
        match var {
            Variable::Null => Primitive::Empty,
            Variable::String(s) => Primitive::String(s),
            Variable::Bool(b) => Primitive::Bool(b),
            Variable::Number(f) => f.as_f64().map(|x| Primitive::F64(x)).unwrap_or(Primitive::Empty),
            Variable::Array(arr) => serde_json::to_string(&arr)
                                        .map(Primitive::Json)
                                        .unwrap_or(Primitive::Empty),
            Variable::Object(obj) => serde_json::to_string(&obj)
                                        .map(Primitive::Json)
                                        .unwrap_or(Primitive::Empty),
            Variable::Expref(_) => {
                log::warn!("The jmespath result cannot be another jmespath");
                Primitive::Empty
            }
        }
    }
}

impl<'a> SeqAct<'a> for JsonActor {
    type BuildParam = ();
    type Config = JsonAction;

    fn build(config: &'a Self::Config, _: Self::BuildParam) -> Result<Self, ActError> {
        Ok(
            Self {
                expr: jmespath::compile(&config.jmespath)
                    .map_err(|e| format!("Failed to compile jmespath `{}`: {}", config.jmespath, e))?,
            }
        )
    }

    fn run(self, parameter: Primitive) -> Primitive {
        match parameter {
            Primitive::Json(json) => {
                match Variable::from_json(&json) {
                    Ok(var) => {
                        match self.expr.search(var) {
                            Ok(result) => Self::jmespath_value_to_primitive(
                                std::sync::Arc::try_unwrap(result)
                                    .unwrap_or_else(|e| {
                                        log::debug!("Cloning jmespath search result");
                                        (*e).clone()
                                    })
                            ),
                            Err(e) => {
                                log::error!("Cannot search through JSON `{}`: {}", json, e);
                                Primitive::Empty
                            }
                        }
                    },
                    Err(e) => {
                        log::error!("Cannot convert to jmespath Variable from JSON `{}`: {}", json, e);
                        Primitive::Empty
                    }
                }
            },
            _ => {
                log::error!("Cannot apply JSON action to non-JSON primitive");
                Primitive::Empty
            },
        }
    }
}
