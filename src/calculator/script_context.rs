use mlua::{Lua, Value};

use crate::helpers::errors::GenericError;

use super::decimal::DecimalValue;

pub struct CalculatorScriptContext {
    lua: Lua,
}

impl CalculatorScriptContext {
    pub fn new() -> Self {
        let lua = Lua::new();

        DecimalValue::add_constructor(&lua, "decimal");

        Self { lua }
    }

    pub fn execute_function(
        &self,
        script: &str,
        function_name: &str,
        parameters: Value,
    ) -> Result<Value, GenericError> {
        todo!()
    }
}

impl Default for CalculatorScriptContext {
    fn default() -> Self {
        Self::new()
    }
}
