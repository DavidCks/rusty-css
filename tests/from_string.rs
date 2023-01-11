use std::{collections::LinkedList, iter::Map};

use rusty_css::*;
use bevy_reflect::{ Reflect, DynamicList, DynamicArray, List, FromReflect };

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


#[derive(Reflect, PartialEq, Debug)]
struct CC {
    append: String,
    prop: [String; 3],
}

impl Style for CC {
    fn create() -> Self {
        Self {
            append: ":arbitrary_name".to_string(),
            prop: [
                "200px".to_string(),
                "400px".to_string(),
                "600px".to_string(),
            ]
        }
    }
}

#[test]
fn test_from_string_with_array() {
    let mut cc = CC::create();
    cc.set_from_inline_string("prop: newpropval1, newpropval1, newpropval1;".to_string());

    let mut newcc= CC::create();
    newcc.prop = [
        "200px".to_string(),
        "400px".to_string(),
        "600px".to_string(),
    ];


    assert_eq!(cc, newcc);
}

#[derive(Reflect, PartialEq, Debug)]
struct DD {
    append: String,
    prop: Vec<String>,
}

impl Style for DD {
    fn create() -> Self {
        Self {
            append: ":arbitrary_name".to_string(),
            prop: vec!(
                "str1".to_string(), 
                "str2".to_string(),
                "str3".to_string(),
            )
        }
    }
}

#[test]
fn test_from_string_with_vec() {
    let mut dd = DD::create();
    dd.set_from_inline_string("prop: newpropval1, newpropval1, newpropval1;".to_string());

    let mut newdd= DD::create();
    newdd.prop = vec!(
        "str1".to_string(),
        "str2".to_string(),
        "str3".to_string(),
    );


    assert_eq!(dd, newdd);
}

// mega struct
#[derive(Reflect, FromReflect, PartialEq, Debug)]
struct MegaNStruct3 {
    f1: Vec<String>
}

#[derive(Reflect, PartialEq, Debug)]
struct MegaNStruct2 {
    func_nested_1: Vec<MegaNStruct3>
}

#[derive(Reflect, PartialEq, Debug)]
struct Mega {
    nested: MegaNStruct2,
}

impl Style for Mega {
    fn create() -> Self {
        Self {
            nested: MegaNStruct2 { 
                func_nested_1: vec![
                    MegaNStruct3 { 
                        f1: vec![
                            "p4".to_string(),
                            "p5".to_string(),
                            "p6".to_string(),
                        ] 
                    },
                    MegaNStruct3 { 
                        f1: vec![
                            "p1".to_string(),
                            "p2".to_string(),
                            "p3".to_string(),
                        ] 
                    }
                ]
            }
        }
    }
}

#[test]
fn test_from_string_including_all() {
    let mut mega = Mega::create();
    mega.set_from_inline_string("nested: func-nested-1(f1(v4,v5,v6), f1(v1, v2, v3));".to_string());

    let new_mega = Mega {
        nested: MegaNStruct2 { 
            func_nested_1: vec![
                MegaNStruct3 { 
                    f1: vec![
                        "v4".to_string(),
                        "v5".to_string(),
                        "v6".to_string(),
                    ] 
                },
                MegaNStruct3 { 
                    f1: vec![
                        "v1".to_string(),
                        "v2".to_string(),
                        "v3".to_string(),
                    ] 
                }
            ]
        }
    };

    assert_eq!(mega, new_mega);
}