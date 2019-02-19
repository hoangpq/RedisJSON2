#[macro_use]
extern crate redismodule;

use redismodule::{Context, RedisResult, NextArg};
use redismodule::native_types::RedisType;

mod redisjson;

use crate::redisjson::RedisJSON;

static REDIS_JSON_TYPE: RedisType = RedisType::new("RedisJSON");

fn json_set(ctx: &Context, args: Vec<String>) -> RedisResult {

    let mut args = args.into_iter().skip(1);

    let key = args.next_string()?;
      let value = args.next_string()?;

    let key = ctx.open_key_writable(&key);

    match key.get_value::<RedisJSON>(&REDIS_JSON_TYPE)? {
        Some(doc) => {
            doc.set_value(&value)?;
        }
        None => {
            let doc = RedisJSON::from_str(&value)?;
            key.set_value(&REDIS_JSON_TYPE, doc)?;
        }
    }

    Ok(().into())
}

fn json_get(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;

    let key = ctx.open_key_writable(&key);

    let value = match key.get_value::<RedisJSON>(&REDIS_JSON_TYPE)? {
        Some(doc) => { doc.to_string()?.into() }
        None => ().into()
    };

    Ok(value)
}

//////////////////////////////////////////////////////

redis_module! {
    name: "redisjson",
    version: 1,
    data_types: [
        REDIS_JSON_TYPE,
    ],
    commands: [
        ["json.set", json_set, "write"],
        ["json.get", json_get, ""],
    ],
}