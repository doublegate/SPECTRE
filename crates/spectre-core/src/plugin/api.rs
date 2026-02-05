//! Spectre Lua API registration
//!
//! Registers the `spectre.*` table in the Lua environment,
//! providing controlled access to SPECTRE functionality.

use mlua::Lua;
use tracing::debug;

use super::manifest::PermissionConfig;

/// Register the spectre API table in the Lua environment
pub fn register_spectre_api(lua: &Lua, _permissions: &PermissionConfig) -> crate::Result<()> {
    let spectre = lua.create_table()?;

    // spectre.version() -> string
    let version_fn = lua.create_function(|_, ()| Ok(env!("CARGO_PKG_VERSION").to_string()))?;
    spectre.set("version", version_fn)?;

    // spectre.log(message) -> nil
    let log_fn = lua.create_function(|_, msg: String| {
        debug!(plugin_log = %msg, "Plugin log");
        Ok(())
    })?;
    spectre.set("log", log_fn)?;

    // spectre.info(message) -> nil
    let info_fn = lua.create_function(|_, msg: String| {
        tracing::info!(plugin_log = %msg, "Plugin info");
        Ok(())
    })?;
    spectre.set("info", info_fn)?;

    // spectre.warn(message) -> nil
    let warn_fn = lua.create_function(|_, msg: String| {
        tracing::warn!(plugin_log = %msg, "Plugin warning");
        Ok(())
    })?;
    spectre.set("warn", warn_fn)?;

    // spectre.json_encode(table) -> string
    let json_encode = lua.create_function(|lua, value: mlua::Value| {
        let json = lua_value_to_json(lua, &value)?;
        serde_json::to_string(&json)
            .map_err(|e| mlua::Error::external(format!("JSON encode error: {}", e)))
    })?;
    spectre.set("json_encode", json_encode)?;

    // spectre.json_decode(string) -> table
    let json_decode = lua.create_function(|lua, s: String| {
        let value: serde_json::Value = serde_json::from_str(&s)
            .map_err(|e| mlua::Error::external(format!("JSON decode error: {}", e)))?;
        json_to_lua_value(lua, &value)
    })?;
    spectre.set("json_decode", json_decode)?;

    // spectre.sleep(seconds) -> nil (limited to 5s max for safety)
    let sleep_fn = lua.create_function(|_, seconds: f64| {
        std::thread::sleep(std::time::Duration::from_secs_f64(seconds.min(5.0)));
        Ok(())
    })?;
    spectre.set("sleep", sleep_fn)?;

    // Set the spectre table as a global
    lua.globals().set("spectre", spectre)?;

    Ok(())
}

/// Convert a Lua value to serde_json::Value
#[allow(clippy::only_used_in_recursion)]
fn lua_value_to_json(lua: &Lua, value: &mlua::Value) -> mlua::Result<serde_json::Value> {
    match value {
        mlua::Value::Nil => Ok(serde_json::Value::Null),
        mlua::Value::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
        mlua::Value::Integer(i) => Ok(serde_json::Value::Number(serde_json::Number::from(*i))),
        mlua::Value::Number(n) => serde_json::Number::from_f64(*n)
            .map(serde_json::Value::Number)
            .ok_or_else(|| mlua::Error::external("Cannot convert NaN/Infinity to JSON number")),
        mlua::Value::String(s) => Ok(serde_json::Value::String(s.to_str()?.to_string())),
        mlua::Value::Table(t) => {
            // Check if it's an array or object
            let len = t.raw_len();
            if len > 0 {
                // Treat as array
                let mut arr = Vec::new();
                for i in 1..=len {
                    let v: mlua::Value = t.raw_get(i)?;
                    arr.push(lua_value_to_json(lua, &v)?);
                }
                Ok(serde_json::Value::Array(arr))
            } else {
                // Treat as object
                let mut map = serde_json::Map::new();
                for pair in t.pairs::<String, mlua::Value>() {
                    let (k, v) = pair?;
                    map.insert(k, lua_value_to_json(lua, &v)?);
                }
                Ok(serde_json::Value::Object(map))
            }
        },
        _ => Ok(serde_json::Value::Null),
    }
}

/// Convert a serde_json::Value to a Lua value
fn json_to_lua_value(lua: &Lua, value: &serde_json::Value) -> mlua::Result<mlua::Value> {
    match value {
        serde_json::Value::Null => Ok(mlua::Value::Nil),
        serde_json::Value::Bool(b) => Ok(mlua::Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(mlua::Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(mlua::Value::Number(f))
            } else {
                Ok(mlua::Value::Nil)
            }
        },
        serde_json::Value::String(s) => {
            let lua_str = lua.create_string(s)?;
            Ok(mlua::Value::String(lua_str))
        },
        serde_json::Value::Array(arr) => {
            let table = lua.create_table()?;
            for (i, v) in arr.iter().enumerate() {
                let lua_val = json_to_lua_value(lua, v)?;
                table.raw_set(i + 1, lua_val)?;
            }
            Ok(mlua::Value::Table(table))
        },
        serde_json::Value::Object(obj) => {
            let table = lua.create_table()?;
            for (k, v) in obj {
                let lua_val = json_to_lua_value(lua, v)?;
                table.set(k.as_str(), lua_val)?;
            }
            Ok(mlua::Value::Table(table))
        },
    }
}

#[cfg(test)]
mod tests {
    use super::super::manifest::PermissionConfig;
    use super::*;

    fn make_lua() -> Lua {
        let lua = Lua::new();
        register_spectre_api(&lua, &PermissionConfig::default()).unwrap();
        lua
    }

    #[test]
    fn test_version_api() {
        let lua = make_lua();
        let result: String = lua.load("return spectre.version()").eval().unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_log_api() {
        let lua = make_lua();
        lua.load("spectre.log('test message')").exec().unwrap();
    }

    #[test]
    fn test_json_encode() {
        let lua = make_lua();
        let result: String = lua
            .load("return spectre.json_encode({name = 'test', value = 42})")
            .eval()
            .unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["name"], "test");
        assert_eq!(parsed["value"], 42);
    }

    #[test]
    fn test_json_decode() {
        let lua = make_lua();
        let result: String = lua
            .load(
                r#"
                local data = spectre.json_decode('{"name":"test","value":42}')
                return data.name
            "#,
            )
            .eval()
            .unwrap();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_json_roundtrip() {
        let lua = make_lua();
        let result: String = lua
            .load(
                r#"
                local original = {a = 1, b = "hello", c = true}
                local encoded = spectre.json_encode(original)
                local decoded = spectre.json_decode(encoded)
                return decoded.b
            "#,
            )
            .eval()
            .unwrap();
        assert_eq!(result, "hello");
    }
}
