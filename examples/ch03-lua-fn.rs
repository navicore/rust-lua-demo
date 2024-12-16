use std::error::Error;

use mlua::*;

fn main() -> Result<()> {
    let lua = Lua::new();
    let chunk = lua.load(
        r#"
function do_stuff(name)
    local num = 42
    name = name or "haha"
    print("Hello from " .. name .. " " .. num)
    return num + value_one
end
return do_stuff
       "#,
    );

    let lua_fn = chunk.eval::<Function>()?;
    let env = lua.create_table()?;
    env.set("value_one", 112)?;
    let globals = lua.globals();
    for pair in globals.pairs::<String, Value>() {
        let (key, value) = pair?;
        env.set(key, value)?;
    }
    lua_fn.set_environment(env);

    for i in 0..3 {
        let mut args = MultiValue::new();
        args.push_front("Rust".into_lua(&lua)?);
        let result = lua_fn.call::<Value>(args)?;
        println!("Result: {:?}", result);
    }

    Ok(())
}
