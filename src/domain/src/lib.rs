mod utils;
use entities::*;
use use_cases::*;
use repositories::*;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate wasm_bindgen;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
