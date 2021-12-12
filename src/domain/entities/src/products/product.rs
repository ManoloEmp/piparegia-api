use js_sys::{Function, Object, Reflect, WebAssembly, Date, Array, JsString, JSON, Promise};
use serde_json::Value;
use std::future::Future;
use wasm_bindgen::prelude::*;
use crate::Entitie;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Debug, Serialize)]
pub struct Product {
    ctx: Value,
    id: String,
    name: Name,
    image: Image,
    description: Description,
    supplier_id: String,
    variety: Variety,
    recipe: Recipe,
    nutritional: Nutritional,
}

#[derive(PartialEq, Clone, Debug, Serialize)]
struct Name {
    value: String
}

#[derive(PartialEq, Clone, Debug, Serialize)]
struct Image {
    src: String,
    alt: String
}

#[derive(PartialEq, Clone, Debug, Serialize)]
struct Description {
    value: String
}

#[derive(PartialEq, Clone, Debug, Serialize)]
enum Variety {
    Vegetarians,
    Meats,
    Mixs
}

#[derive(PartialEq, Clone, Debug, Serialize)]
struct Recipe {
    time: String,
    ingredients: Vec<Ingredient>,
    preparation: Vec<String>
}

#[derive(PartialEq, Clone, Debug, Serialize)]
struct Ingredient {
    amount: String,
    name: String
}

#[derive(PartialEq, Clone, Debug, Serialize)]
struct Nutritional {
    values: Vec<String>
}

#[wasm_bindgen]
impl Product {

    #[wasm_bindgen(constructor)]
    pub fn new(ctx: JsValue) -> Self {
        let mut proto = Entitie::new(
            move | json | -> Result<Value, JsValue> {
                let args: Value = Value::Object(serde_wasm_bindgen::from_value(json)?);
                Ok(args)
            });

        let _attrs = &proto.ctx(ctx);

        //convirtiendo result a option
        let val = _attrs.as_ref().ok();

        println!("The name is {}", &val.unwrap().get("name").unwrap()["value"].to_string());

        let product = Product {
            ctx: val.unwrap().to_owned(), //.as_object().unwrap().to_owned() ...--> convertir en Map<String, Value>,
            //ctx: proto.get_ctx(),
            id: proto.get_id(),
            name: Name {
                value: val.unwrap().get("name").unwrap()["value"].to_string() // proto.get_ctx().get("name").unwrap().as_str().unwrap().to_string()
            }, // ,
            image: Image {
                src: if val.unwrap().get("image").is_some() {
                    val.unwrap().get("image").unwrap()["src"].to_string()

                } else { "".to_string() }, //val.unwrap().get("image").unwrap()["src"].to_string(),
                alt: if val.unwrap().get("image").is_some() {
                    val.unwrap().get("image").unwrap()["alt"].to_string()
                } else { "".to_string() }
            },
            description: Description {
                value: val.unwrap().get("description").unwrap()["value"].to_string()
            },
            supplier_id: "".to_string(),
            variety: Variety::Vegetarians,
            recipe: Recipe {
                time: "".to_string(),
                ingredients: vec![Ingredient{ amount: "".to_string(), name: "".to_string()}, Ingredient{ amount: "".to_string(), name: "".to_string()}],
                preparation: vec!["".to_string(), "".to_string(),"".to_string(), "".to_string()]
            },
            nutritional: Nutritional {
                values: vec!["".to_string(), "".to_string(),"".to_string(), "".to_string()]
            }

        };
        product
    }

    #[wasm_bindgen(getter)]
    pub fn ctx(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.ctx).unwrap()

    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.id).unwrap()

    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> JsValue {
        JsValue::from_serde(&self.name).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> JsValue {
        JsValue::from_serde(&self.description).unwrap()

    }

    #[wasm_bindgen(getter)]
    pub fn image(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.image).unwrap()

    }

    #[wasm_bindgen(getter)]
    pub fn supplier_id(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.supplier_id).unwrap()

    }

    #[wasm_bindgen(getter)]
    pub fn variety(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.variety).unwrap()

    }

    #[wasm_bindgen(getter)]
    pub fn recipe(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.recipe).unwrap()

    }

    #[wasm_bindgen(getter)]
    pub fn nutritional(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.nutritional).unwrap()

    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, value: String) {
        self.name.value = value;
    }

    #[wasm_bindgen(setter)]
    pub fn set_description(&mut self, value: String) {
        self.description.value = value;
    }

}
