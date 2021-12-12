pub mod products;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate wasm_bindgen;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;

use js_sys::{Function, Object, Reflect, WebAssembly, Date, Array, JsString, JSON, Promise};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
struct Entitie<T>
where T: Fn(JsValue) -> Result<Value, JsValue>
{
    _id: String,
    _process: T,
    _ctx: Option<Value>,
}

impl<T> Entitie<T>
where T: Fn(JsValue) -> Result<Value, JsValue>
{
        fn new(process: T) -> Self {
        let uuid = format!("{}", uuid::Uuid::new_v4());

        Entitie {
            _id: uuid,
            _process: process,
            _ctx: None,
        }
    }

        fn ctx(&mut self, args: JsValue) -> Result<Value, JsValue> {
        match &self._ctx {
            Some(v) => Ok(v.to_owned()),
            None => {
                let v = (self._process)(args);
                self._ctx = Some(v.to_owned()?);
                v
            },
        }
    }

        fn get_ctx(&self) -> Value {
        match &self._ctx {
            Some(c) => c.to_owned(),
            _ => panic!("Problem reading the file: {:?}", self._ctx),
        }
    }

        fn get_id(&self) -> String {
        self._id.to_owned()
    }
}
