use serde::{Serialize, Deserialize};
use serde_json::Value;
use usdpl_back::core::serdes::Primitive;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "element")]
pub enum ApiDisplayResult {
    #[serde(rename = "value")]
    Value(ApiValue),
    #[serde(rename = "error")]
    Error(ApiError),
}

impl ApiDisplayResult {
    pub fn dump(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_primitive(self) -> Primitive {
        Primitive::Json(self.dump())
    }

    pub fn success(primitive: Primitive) -> Self {
        Self::Value(ApiValue::new(primitive))
    }

    pub fn failure<S: Into<String>, D: core::fmt::Display>(msg: S, err: D) -> Self {
        Self::Error(ApiError::new(msg, err))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiValue {
    pub value: Value,
}

impl ApiValue {
    pub fn new(primitive: Primitive) -> Self {
        let val = match primitive {
            Primitive::Empty => Value::Null,
            Primitive::String(s) => Value::String(s),
            Primitive::F32(x) => x.into(),
            Primitive::F64(x) => x.into(),
            Primitive::U32(x) => Value::Number(x.into()),
            Primitive::U64(x) => Value::Number(x.into()),
            Primitive::I32(x) => Value::Number(x.into()),
            Primitive::I64(x) => Value::Number(x.into()),
            Primitive::Bool(x) => Value::Bool(x),
            Primitive::Json(x) => serde_json::from_str(&x).unwrap_or(Value::Null),
        };
        Self {
            value: val,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiError {
    pub message: String,
    pub exception: String,
}

impl ApiError {
    pub fn new<S: Into<String>, D: core::fmt::Display>(msg: S, err: D) -> Self {
        Self {
            message: msg.into(),
            exception: err.to_string(),
        }
    }
}
