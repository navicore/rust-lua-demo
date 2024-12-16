use std::error::Error;

use mlua::*;

fn main() -> Result<()> {
    let lua = Lua::new();
    let chunk = lua.load(
        r#"
local num = 42
print("Hello from Lua! " .. num)
return num + value_one

       "#,
    );
    //Option 1 - with globals
    // let globals = lua.globals();
    // globals.set("value_one", 111)?;

    //Option 2 - with set_environment
    let env = lua.create_table()?;
    env.set("value_one", 112)?;
    let globals = lua.globals();
    for pair in globals.pairs::<String, Value>() {
        let (key, value) = pair?;
        env.set(key, value)?;
    }
    let chunk = chunk.set_environment(env);

    let result = chunk.eval::<Value>()?;
    print!("Result: {:?}", result);

    Ok(())
}
