use wasm_bindgen::prelude::*;
use std::future::Future;
use std::rc::Rc;
//use std::fmt;
//use std::io;
use std::collections::HashMap;
//use std::fmt::Error;
use js_sys::{Function, Object, Reflect, WebAssembly, Date, Array, JsString, JSON, Promise};
use serde_json::Value;
use entities::*;
use repositories::*;
use wasm_bindgen_futures::JsFuture;

use product_repository::IProductRepo;
use products::product::Product;


#[wasm_bindgen(module = "/prototypes/create-product-controller.js")]
extern "C" {
    #[derive(PartialEq, Clone, Debug)]
    type CreateProductController;

    #[wasm_bindgen(constructor)]
    pub fn new(use_case: CreateProductUC) -> CreateProductController;
    #[wasm_bindgen(method, catch)]
    pub async fn handle(this: &CreateProductController, f: &dyn Fn(/*Request, Response*/) -> Result<JsValue, JsValue>) -> Result<JsValue, JsValue>;
}

use serde::ser::{Serialize, SerializeStruct, Serializer};

// This is what #[derive(Serialize)] would generate.
impl Serialize for CreateProductController {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CreateProductController", 0)?;
        s.end()
    }
}



#[wasm_bindgen]
#[derive(PartialEq, Clone, Debug)]
pub struct CreateProductUC {
    product_repository: IProductRepo
}


impl CreateProductUC {

    pub fn new(repository: &IProductRepo ) -> CreateProductUC {
        CreateProductUC {
            product_repository: repository.to_owned()
        }
    }


    pub async fn execute(&self, ctx: JsValue) -> Result<JsValue, JsValue> {
        //Ok(ctx) /*
        let props = &ctx;
        let args: Value = Value::Object(serde_wasm_bindgen::from_value(props.to_owned())?);
        let value = args.get("name").unwrap()["value"].as_str().unwrap().to_string();
        let prop = &value;
        let promise = self.product_repository.find_by_name(prop.to_owned());
        let result = wasm_bindgen_futures::JsFuture::from(promise.to_owned()).await?;

        if result ==  prop {
            let result_2 = JsValue::from_serde(&"The product allready exist".to_string());
            return Ok(result_2.unwrap())
        }
        let product = Product::new(props.to_owned());
        let promise_2 = self.product_repository.save(product);
        let result_2 = wasm_bindgen_futures::JsFuture::from(promise_2.to_owned()).await?;
        Ok(result_2)
    }
}



/*
impl Worker {
    pub fn process(&self, command: String) -> impl Future<Output = Result<JsValue, JsValue>> {
        let info = JsFuture::from(self.user.info());

        async move {
            let output = match command.as_str() {
                "adapter" => { self.user.adapter() }
                "info" => {
                    match info.await {
                        Ok(resolved) => {
                            match resolved.into_serde::<Info>() {
                                Ok(val) => format!("{:?}", &val),
                                Err(_) => "Deserialize error".to_string(),
                            }
                        },
                        Err(_) => "Promise error".to_string(),
                    }
                }
                _ => String::from("Unknown command")
            };
            Ok(JsValue::from(output))
        }
    }
}

pub fn process(&self, command: String) -> Promise {
        let info = JsFuture::from(self.user.info());
        let adapter = self.user.adapter();

        future_to_promise(async move {
            let output = match command.as_str() {
                "adapter" => adapter,
                "info" => {
                    match info.await {
                        Ok(resolved) => {
                            match resolved.into_serde::<Info>() {
                                Ok(val) => format!("{:?}", &val),
                                Err(_) => "Deserialize error".to_string(),
                            }
                        },
                        Err(_) => "Promise error".to_string(),
                    }
                }
                _ => String::from("Unknown command")
            };
            Ok(JsValue::from(output))
        })
    } */

/*
#[derive(PartialEq, Clone, Debug, Serialize)]
struct CreateProductService {
    controller: CreateProductController
}



impl CreateProductService {
    fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        // ...
        println!("{}", my_string);
    }
}
     pub fn new <F>(self, executable: F) -> CreateProductService
     where F: Fn(JsValue) -> CreateProductUseCase,
           F: Serialize
     {
         CreateProductService {
             controller: CreateProductController {
                 obj: JsValue::from_serde(&executable).unwrap()
             }
         }
     }


/*    async fn load<F>(self, ctx: JsValue, executable: F) -> Result<JsValue, JsValue>
    where F: Fn(JsValue) -> Result<JsValue, JsValue>
    {

       let exe = executable(ctx);

        // obj en js { statusCode: status, response: res}
       let obj = self.controller.handle(exe, );
   } */

    async fn foo(self, duck: &IProductRepo, ctx: JsValue) ->  CreateProductController

    {
        let mut use_case = CreateProductUseCase::new(duck);
        //let result = use_case.execute(ctx).await?;
        let valo = self.new(|ctx| {
            use_case
        });

         valo
        /*let con = CreateProductController {
            obj: JsValue::from_serde(&use_case).unwrap()
        };*/

        //let value = CreateProductController;

    }
}*/



// This is what #[derive(Serialize)] would generate.
impl Serialize for CreateProductUC {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CreateProductUC", 1)?;
        s.serialize_field("product_repository", &self.product_repository)?;
        s.end()
    }
}



#[wasm_bindgen]
#[derive(PartialEq, Clone, Debug)]
pub struct Request {
//method: String,
url: String,
body: JsValue //Vec<JsValue>
}

#[wasm_bindgen]
impl Request {
    #[wasm_bindgen(constructor)]
    pub fn new(url: String, body: JsValue) -> Self {
        Self {
            url,
            body
        }
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> JsValue {
        self.body.to_owned()
    }

}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Debug)]
struct Response {
code: u32,
body: JsValue //Vec<JsValue>
}

#[wasm_bindgen]
impl Response {
    #[wasm_bindgen(constructor)]
    pub fn new(code: u32, body: JsValue) -> Self {
        Self {
            code,
            body
        }
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> JsValue {
        self.body.to_owned()
    }

}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

#[wasm_bindgen]
struct BasicRouter {
routes: HashMap<String, BoxedCallback>
}

#[wasm_bindgen]
impl BasicRouter {
// Create an empty router.
#[wasm_bindgen(constructor)]
pub fn new() -> BasicRouter {
BasicRouter { routes: HashMap::new() }
}
// Add a route to the router.
fn add_route<C>(&mut self, url: &str, callback: C)
where C: Fn(&Request) -> Response + 'static
{
self.routes.insert(url.to_string(), Box::new(callback));
}
}

#[wasm_bindgen]
impl BasicRouter {
pub fn not_found_response(&self) -> Response {
    //let mut vec = Vec::new();
    let res = JsValue::from_serde(&"The product allready exist".to_string()).unwrap();
    //vec.push(res);
    Response {
        code: 400,
        body: res
    }
}
fn handle_request(&self, request: &Request) -> Response {
match self.routes.get(&request.url) {
None => self.not_found_response(),
Some(callback) => callback(request)
}
}
}
