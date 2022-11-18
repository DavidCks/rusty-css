use rusty_css::*;
use bevy_reflect::{ Reflect };

use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bindgen_test::wasm_bindgen_test_configure;
wasm_bindgen_test_configure!(run_in_browser);

// test class string with pseudo classes 2

#[derive(Reflect)]
struct NStruct {
    func1: String,
    func2: String,
}

#[derive(Reflect)]
struct BB {
    append: String,
    prop: String,
    nested: NStruct,
}

impl Style for BB {
    fn create() -> Self {
        Self {
            append: ":arbitrary_name".to_string(),
            prop: "200px".to_string(),
            nested: NStruct { 
                func1: "func_prop1".to_string(), 
                func2: "func_prop2".to_string() 
            }
        }
    }
}

#[wasm_bindgen_test]
fn test_class_export_with_pseudo_classes_in_style_tag_1 (){
    // grab the current document
    let window = web_sys::window().expect("No global `window` found");
    let document = window.document().expect("couldn't get `document");

    // add the style for BB to the style tag in the document
    let b = BB::create();
    let class_name = b.as_class(&document).unwrap();

    assert_eq!(class_name, "BB");
}

#[wasm_bindgen_test]
fn test_class_export_with_pseudo_classes_in_style_tag_2 (){
    // grab the current document
    let window = web_sys::window().expect("No global `window` found");
    let document = window.document().expect("couldn't get `document");

    // add the style for BB to the style tag in the document
    let b = BB::create();
    let class_name = b.as_class(&document).unwrap();

    // grab the contents of the style tag of the document again
    let style = document.query_selector("#rusty-css-BB").unwrap();
    let style_content = style.unwrap().text_content().unwrap();

    // compare the inserted style with the computed class string
    assert_eq!(style_content, b.as_class_string(class_name).unwrap());

}