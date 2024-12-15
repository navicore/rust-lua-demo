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
    let globals = lua.globals();
    globals.set("value_one", 111)?;

    let result = chunk.eval::<Value>()?;
    print!("Result: {:?}", result);

    Ok(())
}
