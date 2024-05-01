use mlua::{Error, FromLua, Lua, MetaMethod, Result, UserData, UserDataMethods, Value};
use num_traits::{pow::Pow, FromPrimitive};
use rust_decimal::Decimal;

#[derive(Copy, Clone)]
pub struct DecimalValue {
    value: Decimal,
}
impl DecimalValue {
    pub fn new(value: Decimal) -> Self {
        Self { value }
    }

    pub fn add_constructor(lua: &Lua, function_name: &str) {
        let globals = lua.globals();
        let decimal_constructor = lua
            .create_function(|_, value: Value| match value {
                Value::Nil => Err(Error::UserDataTypeMismatch),
                Value::Boolean(_) => Err(Error::UserDataTypeMismatch),
                Value::LightUserData(_) => Err(Error::UserDataTypeMismatch),
                Value::Integer(value) => Decimal::from_i64(value)
                    .map(DecimalValue::new)
                    .ok_or(Error::UserDataTypeMismatch),
                Value::Number(value) => Decimal::from_f64(value)
                    .map(DecimalValue::new)
                    .ok_or(Error::UserDataTypeMismatch),
                Value::String(value) => Decimal::from_str_radix(value.to_str().unwrap(), 10)
                    .map(DecimalValue::new)
                    .or(Err(Error::UserDataTypeMismatch)),
                Value::Table(_) => Err(Error::UserDataTypeMismatch),
                Value::Function(_) => Err(Error::UserDataTypeMismatch),
                Value::Thread(_) => Err(Error::UserDataTypeMismatch),
                Value::UserData(_) => Err(Error::UserDataTypeMismatch),
                Value::Error(_) => Err(Error::UserDataTypeMismatch),
            })
            .unwrap();
        globals.set(function_name, decimal_constructor).unwrap();
    }
}

impl<'lua> FromLua<'lua> for DecimalValue {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> Result<Self> {
        match value {
            Value::UserData(ud) => Ok(*ud.borrow::<Self>()?),
            _ => unreachable!(),
        }
    }
}

impl UserData for DecimalValue {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(
            MetaMethod::Add,
            |_, (a, b): (DecimalValue, DecimalValue)| Ok(DecimalValue::new(a.value + b.value)),
        );
        methods.add_meta_function(
            MetaMethod::Sub,
            |_, (a, b): (DecimalValue, DecimalValue)| Ok(DecimalValue::new(a.value - b.value)),
        );
        methods.add_meta_function(
            MetaMethod::Mul,
            |_, (a, b): (DecimalValue, DecimalValue)| Ok(DecimalValue::new(a.value * b.value)),
        );
        methods.add_meta_function(
            MetaMethod::Div,
            |_, (a, b): (DecimalValue, DecimalValue)| Ok(DecimalValue::new(a.value / b.value)),
        );
        methods.add_meta_function(
            MetaMethod::Mod,
            |_, (a, b): (DecimalValue, DecimalValue)| Ok(DecimalValue::new(a.value % b.value)),
        );
        methods.add_meta_function(
            MetaMethod::Pow,
            |_, (a, b): (DecimalValue, DecimalValue)| Ok(DecimalValue::new(a.value.pow(b.value))),
        );
        methods.add_meta_function(MetaMethod::Unm, |_, a: DecimalValue| {
            Ok(DecimalValue::new(-a.value))
        });
        methods.add_meta_function(
            MetaMethod::IDiv,
            |_, (a, b): (DecimalValue, DecimalValue)| {
                Ok(DecimalValue::new((a.value / b.value).floor()))
            },
        );
        methods.add_meta_function(MetaMethod::Eq, |_, (a, b): (DecimalValue, DecimalValue)| {
            Ok(a.value == b.value)
        });
        methods.add_meta_function(MetaMethod::Lt, |_, (a, b): (DecimalValue, DecimalValue)| {
            Ok(a.value < b.value)
        });
        methods.add_meta_function(MetaMethod::Le, |_, (a, b): (DecimalValue, DecimalValue)| {
            Ok(a.value <= b.value)
        });
    }

    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(_fields: &mut F) {}
}
