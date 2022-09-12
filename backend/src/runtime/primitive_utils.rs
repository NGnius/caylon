use usdpl_back::core::serdes::Primitive;

//use super::ActError;

/*macro_rules! map_primitive_number_impl {
    ($type:ty, $type_name:literal, $fn_name:ident) => {
        pub fn $fn_name (param: Primitive) -> Result<$type, ActError> {
            match param {
                Primitive::I64(a) => Ok(a as $type),
                Primitive::I32(a) => Ok(a as $type),
                Primitive::U64(a) => Ok(a as $type),
                Primitive::U32(a) => Ok(a as $type),
                Primitive::F64(a) => Ok(a as $type),
                Primitive::F32(a) => Ok(a as $type),
                _ => Err(format!("Parameter must be {} type", $type_name))
            }
        }
    }
}*/

/*macro_rules! map_primitive_impl {
    ($type:ty, $primitive:ident, $type_name:literal, $fn_name:ident) => {
        pub fn $fn_name (param: Primitive) -> Result<$type, ActError> {
            match param {
                Primitive::$primitive(a) => Ok(a),
                _ => Err(format!("Parameter must be {} type", $type_name))
            }
        }
    }
}*/

//map_primitive_impl!{bool, Bool, "boolean", try_primitive_bool}

//map_primitive_impl!{String, String, "string", try_primitive_string}

//map_primitive_number_impl!{usize, "uinteger", try_primitive_usize}

#[inline]
pub fn debug(primitive: &Primitive) -> String {
    match primitive {
        Primitive::Empty => "Primitive::Empty".to_owned(),
        Primitive::String(x) => format!("Primitive::String(`{}`)", x),
        Primitive::F32(x) => format!("Primitive::F32(`{}`)", x),
        Primitive::F64(x) => format!("Primitive::F64(`{}`)", x),
        Primitive::U32(x) => format!("Primitive::U32(`{}`)", x),
        Primitive::U64(x) => format!("Primitive::U64(`{}`)", x),
        Primitive::I32(x) => format!("Primitive::I32(`{}`)", x),
        Primitive::I64(x) => format!("Primitive::I64(`{}`)", x),
        Primitive::Bool(x) => format!("Primitive::Bool(`{}`)", x),
        Primitive::Json(x) => format!("Primitive::Json(`{}`)", x),
    }
}

/*#[inline]
pub fn display(primitive: Primitive) -> String {
    match primitive {
        Primitive::Empty => "".to_owned(),
        Primitive::String(x) => x,
        Primitive::F32(x) => x.to_string(),
        Primitive::F64(x) => x.to_string(),
        Primitive::U32(x) => x.to_string(),
        Primitive::U64(x) => x.to_string(),
        Primitive::I32(x) => x.to_string(),
        Primitive::I64(x) => x.to_string(),
        Primitive::Bool(x) => x.to_string(),
        Primitive::Json(x) => x,
    }
}*/

#[inline]
pub fn clone(primitive: &Primitive) -> Primitive {
    match primitive {
        Primitive::Empty => Primitive::Empty,
        Primitive::String(x) => Primitive::String(x.clone()),
        Primitive::F32(x) => Primitive::F32(*x),
        Primitive::F64(x) => Primitive::F64(*x),
        Primitive::U32(x) => Primitive::U32(*x),
        Primitive::U64(x) => Primitive::U64(*x),
        Primitive::I32(x) => Primitive::I32(*x),
        Primitive::I64(x) => Primitive::I64(*x),
        Primitive::Bool(x) => Primitive::Bool(*x),
        Primitive::Json(x) => Primitive::Json(x.clone()),
    }
}
