pub mod create_product_uc;

#[macro_use]
extern crate wasm_bindgen;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
