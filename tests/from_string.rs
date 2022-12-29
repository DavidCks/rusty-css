use rusty_css::*;
use bevy_reflect::{ Reflect };

#[derive(Reflect, PartialEq, Debug)]
struct NStruct {
    func1: String,
    func2: String,
}

#[derive(Reflect, PartialEq, Debug)]
struct BB {
    append: String,
    prop_with_underscore: String,
    prop: String,
    nested: NStruct,
}

impl Style for BB {
    fn create() -> Self {
        Self {
            append: ":arbitrary_name".to_string(),
            prop: "200px".to_string(),
            prop_with_underscore: "prop_underscore_value".to_string(),
            nested: NStruct { 
                func1: "func_prop1".to_string(), 
                func2: "func_prop2".to_string() 
            }
        }
    }
}

#[test]
fn test_from_string_nested() {
    let mut bb = BB::create();
    bb.set_from_inline_string("nested: func1(val1) func2(val2);".to_string());

    let mut newbb = BB::create();
    assert_ne!(bb, newbb);

    newbb.nested = NStruct { func1: "val1".to_string(), func2: "val2".to_string() }; 
    assert_eq!(bb, newbb);
}

#[test]
fn test_from_string_with_underscore() {
    let mut bb = BB::create();
    bb.set_from_inline_string("prop-with-underscore: new_underscore_val;".to_string());

    let mut newbb = BB::create();
    newbb.prop_with_underscore = " new_underscore_val".to_string(); 

    assert_eq!(bb, newbb);
}

#[test]
fn test_from_string() {
    let mut bb = BB::create();
    bb.set_from_inline_string("prop: newpropval;".to_string());

    let mut newbb = BB::create();
    newbb.prop = " newpropval".to_string(); 

    assert_eq!(bb, newbb);
}