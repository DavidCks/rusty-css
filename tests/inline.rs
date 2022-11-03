use rusty_css::*;
use bevy_reflect::{ Reflect };

#[derive(Reflect)]
struct A {
    width: String,
}

impl Style for A {
    fn create() -> Self {
        Self {
            width: "200px".to_string(),
        }
    }
}

#[test]
fn test1(){
    let a = A::create();
    assert_eq!(a.inline(), "width: 200px; ");
}


#[derive(Reflect)]
struct NStruct {
    func1: String,
    func2: String,
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

#[test]
fn test2(){
    let b = B::create();
    assert_eq!(b.inline(), "prop: 200px; nested: func1( func_prop1) func2( func_prop2); ");
}

    
    