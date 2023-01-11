use rusty_css::*;
use bevy_reflect::{ Reflect };

// simple String

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

// with nested struct

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

// with nested struct and array 

#[derive(Reflect)]
struct NCStruct {
    func1: String,
    func2: String,
}

#[derive(Reflect)]
struct C {
    prop: String,
    nested: NCStruct,
    array: [String; 3],
}
    
impl Style for C {
    fn create() -> Self {
        Self {
            prop: "200px".to_string(),
            nested: NCStruct { 
                func1: "func_prop1".to_string(), 
                func2: "func_prop2".to_string(),
            },
            array: ["firststr".to_string(), "secondstr".to_string(), "thirdstr".to_string()]
        }
    }
}

#[test]
fn test3(){
    let c = C::create();
    assert_eq!(c.inline(), "prop: 200px; nested: func1( func_prop1) func2( func_prop2); array: firststr, secondstr, thirdstr; ");
}

// with nested struct and array in a nested struct

#[derive(Reflect)]
struct NDStruct {
    func1: String,
    func2: [String; 4],
}

#[derive(Reflect)]
struct D {
    prop: String,
    nested: NDStruct,
    array: [String; 3],
}
    
impl Style for D {
    fn create() -> Self {
        Self {
            prop: "200px".to_string(),
            nested: NDStruct { 
                func1: "func_prop1".to_string(), 
                func2: [
                    "func_prop2_vec_1".to_string(),
                    "func_prop2_vec_2".to_string(),
                    "func_prop2_vec_3".to_string(),
                    "func_prop2_vec_4".to_string(),
                ],
            },
            array: [
                "firststr".to_string(), 
                "secondstr".to_string(), 
                "thirdstr".to_string()
            ],
        }
    }
}

#[test]
fn test4(){
    let d = D::create();
    assert_eq!(d.inline(), "prop: 200px; nested: func1( func_prop1) func2( func_prop2_vec_1, func_prop2_vec_2, func_prop2_vec_3, func_prop2_vec_4); array: firststr, secondstr, thirdstr; ");
}