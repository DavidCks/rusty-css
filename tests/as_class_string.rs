use rusty_css::*;
use bevy_reflect::{ Reflect };

#[derive(Reflect)]
struct NStruct {
    func1: String,
    func2: String,
}

// test class string

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

#[test]
fn test_standard_class_string_export (){
    let mut b = B::create();
    assert_eq!(b.as_class_string( b.get_struct_name().unwrap() ).unwrap(), ".B { prop: 200px; nested: func1( func_prop1) func2( func_prop2); }");
}

// test class string with pseudo classes 1

#[derive(Reflect)]
struct BA {
    append: String,
    prop: String,
    nested: NStruct,
}

impl Style for BA {
    fn create() -> Self {
        Self {
            append: ":before".to_string(),
            prop: "200px".to_string(),
            nested: NStruct { 
                func1: "func_prop1".to_string(), 
                func2: "func_prop2".to_string() 
            }
        }
    }
}

#[test]
fn test_class_string_export_with_pseudo_classes_1 (){
    let mut b = BA::create();
    assert_eq!(b.as_class_string( b.get_struct_name().unwrap() ).unwrap(), ".BA:before { prop: 200px; nested: func1( func_prop1) func2( func_prop2); }");
}


// test class string with pseudo classes 2

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

#[test]
fn test_class_string_export_with_pseudo_classes_2 (){
    let mut b = BB::create();
    assert_eq!(b.as_class_string( b.get_struct_name().unwrap() ).unwrap(), ".BB:arbitrary_name { prop: 200px; nested: func1( func_prop1) func2( func_prop2); }");
}