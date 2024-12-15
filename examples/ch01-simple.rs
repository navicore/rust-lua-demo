use std::error::Error;

use mlua::*;

fn main() -> Result<()> {
    let lua = Lua::new();
    let chunk = lua.load(
        r#"
local num = 42
print("Hello from Lua! " .. num)
return num + 1

       "#,
    );
    //chunk.exec()?;

    let result = chunk.eval::<Value>()?;
    print!("Result: {:?}", result);

    Ok(())
}
