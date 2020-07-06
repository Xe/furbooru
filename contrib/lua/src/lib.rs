#[macro_use]
extern crate mlua_derive;
use mlua::{prelude::*, UserData, UserDataMethods, Value};
use std::sync::Arc;

#[lua_module]
fn luafurbooru(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("new", lua.create_function(new)?)?;
    Ok(exports)
}

fn new(_: &Lua, (user_agent, api_token): (String, String)) -> LuaResult<Client> {
    Ok(Client::new(user_agent, api_token).unwrap())
}

#[derive(Clone)]
struct Client {
    cli: Arc<furbooru::Client>,
}

impl<'lua> Client {
    fn new(user_agent: String, api_token: String) -> LuaResult<Self> {
        Ok(Client {
            cli: Arc::new(
                furbooru::Client::new(user_agent, api_token).expect("client construction failed"),
            ),
        })
    }

    async fn forum(&self, lua: &'lua Lua, name: String) -> LuaResult<Value<'lua>> {
        let frm = self
            .cli
            .clone()
            .forum(name)
            .await
            .expect("forum call to work");
        Ok(mlua_serde::to_value(&lua, frm).expect("encode as lua"))
    }
}

async fn forum<'lua>(lua: &'lua Lua, cli: Client, name: String) -> LuaResult<Value<'lua>> {
    println!("got here");
    cli.forum(lua, name).await
}

impl UserData for Client {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("forum", forum);
    }
}
