use log::info;
use rusty_css::*;
use bevy_reflect::{ Reflect };

use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bindgen_test::wasm_bindgen_test_configure;
use web_sys::Document;
wasm_bindgen_test_configure!(run_in_browser);

#[derive(Reflect)]
struct NStruct {
    func1: String,
    func2: String,
}

// test class string

#[allow(non_camel_case_types)]
#[derive(Reflect)]
struct B_hover {
    prop: String,
    nested: NStruct,
}

impl Style for B_hover {
    fn create() -> Self {
        Self {
            prop: "200px".to_string(),
            nested: NStruct { 
                func1: "func_prop1".to_string(), 
                func2: "func_prop2".to_string() 
            }
        }
    }
}

#[derive(Reflect)]
struct B {
    prop: String,
    nested: NStruct,
}

impl Style for B {
    fn create() -> Self {
        Self {
            prop: "200px".to_string(),
            nested: NStruct { 
                func1: "func_prop1".to_string(), 
                func2: "func_prop2".to_string() 
            }
        }
    }
}

fn get_style_content<S: Into<String>>(id:  S) -> String {
   // grab the current document
   let window = web_sys::window().expect("No global `window` found");
   let document = window.document().expect("couldn't get `document");

   // grab the contents of the style tag of the document again
   let style_id = format!("#{}", id.into());
   let style = document.query_selector(&style_id).unwrap();
   info!("{:?}", style);
   let style_content = style.unwrap().text_content().unwrap(); 

   style_content
}

fn get_document() -> Document {
    // grab the current document
   let window = web_sys::window().expect("No global `window` found");
   let document = window.document().expect("couldn't get `document");
   document
}

#[wasm_bindgen_test]
fn test_add_as_pseudo_class (){
    
    // add the style for B_hover to the style tag in the document
    let b = B_hover::create();
    b.add_as_pseudo_class(&get_document());


    assert_eq!(get_style_content("rusty-css-B_hover"), ".B:hover { prop: 200px; nested:  func1(func_prop1) func2(func_prop2); }");
}

#[wasm_bindgen_test]
fn test_add_as_pseudo_class_2 (){
    
    // add the style for B_hover to the style tag in the document
    let b = B::create();
    let _res = b.as_class(&get_document());

    let b = B_hover::create();
    b.add_as_pseudo_class(&get_document());

    assert_eq!(get_style_content("rusty-css-B_hover"), ".B:hover { prop: 200px; nested:  func1(func_prop1) func2(func_prop2); }");
}
