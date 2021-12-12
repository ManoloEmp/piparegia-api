use wasm_bindgen::prelude::*;
use js_sys::{Function, Object, Reflect, WebAssembly, Date, Array, JsString, JSON, Promise};
use serde_json::Value;
use entities::products::product::Product;

#[wasm_bindgen]
extern "C" {
    #[derive(PartialEq, Clone, Debug)]
    pub type IProductRepo;

    #[wasm_bindgen(constructor)]
    fn new() -> IProductRepo;
    #[wasm_bindgen(structural, method)]
    pub  fn find_by_name(this: &IProductRepo, name: String) -> Promise;
    #[wasm_bindgen(structural, method)]
    pub  fn save(this: &IProductRepo, product: Product) -> Promise;
}

use serde::ser::{Serialize, SerializeStruct, Serializer};

// This is what #[derive(Serialize)] would generate.
impl Serialize for IProductRepo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("IProductRepo", 0)?;
        s.end()
    }
}

use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

impl<'de> Deserialize<'de> for IProductRepo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {


        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = IProductRepo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct IProductRepo")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<IProductRepo, V::Error>
            where
                V: SeqAccess<'de>,
            {

                Ok(IProductRepo::new())
            }

            fn visit_map<V>(self, mut map: V) -> Result<IProductRepo, V::Error>
            where
                V: MapAccess<'de>,
            {

                Ok(IProductRepo::new())
            }
        }

        const FIELDS: &'static [&'static str] = &[];
        deserializer.deserialize_struct("IProductRepo", FIELDS, DurationVisitor)
    }
}

/*
#[wasm_bindgen]
extern "C" {
    #[derive(PartialEq, Clone, Debug)]
    pub type IterfaceProductRepo;

    #[wasm_bindgen(structural, method)]
    pub  fn find_by_name(this: &IProductRepo, name: String) -> Promise;
    #[wasm_bindgen(structural, method)]
    pub  fn save(this: &IProductRepo, product: Product) -> Promise;
} */
