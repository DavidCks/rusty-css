use std::vec;

use rusty_css::*;
use bevy_reflect::{ Reflect, FromReflect };

#[derive(Reflect, FromReflect, PartialEq, Debug)]
struct NStruct {
    func1: String,
    func2: String,
}

#[derive(Reflect, PartialEq, Debug)]
struct NstructWithStrings {
    func1: String,
    func2: String,
    func3: String,
}

#[derive(Reflect, PartialEq, Debug)]
struct NstructWithVecsStringsAndStructs {
    vec: Vec<String>,
    str: String,
    struc: NStruct,
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
    prop: Vec<String>,
}

impl Style for CC {
    fn create() -> Self {
        Self {
            append: ":arbitrary_name".to_string(),
            prop: vec![
                "200px".to_string(),
                "400px".to_string(),
                "600px".to_string(),
            ]
        }
    }
}

#[test]
fn test_from_string_with_vec() {
    let mut cc = CC::create();
    cc.set_from_inline_string("prop: newpropval1, newpropval2, newpropval3;".to_string());

    let mut newcc= CC::create();
    newcc.prop = vec![
        " newpropval1".to_string(),
        " newpropval2".to_string(),
        " newpropval3".to_string(),
    ];


    assert_eq!(cc, newcc);
}

#[derive(Reflect, PartialEq, Debug)]
struct DD {
    append: String,
    nstruct: NstructWithStrings,
    nstruct_vec_str_struc: NstructWithVecsStringsAndStructs,
    prop: Vec<String>,
    prop2: Vec<NStruct>,
}

impl Style for DD {
    fn create() -> Self {
        Self {
            append: ":arbitrary_name".to_string(),
            nstruct: NstructWithStrings { func1: "NStopf1".to_string(), func2: "NStopf2".to_string(), func3: "NStopf3".to_string(), },
            nstruct_vec_str_struc: NstructWithVecsStringsAndStructs { 
                vec: vec!["old1".to_owned(), "old2".to_owned(), "old3".to_owned()], 
                str: "oldx".to_owned(),
                struc: NStruct { func1: "sssss".to_string(), func2: "ddddd".to_string() }
            },
            prop: vec!(
                "str1".to_string(), 
                "str2".to_string(),
                "str3".to_string(),
            ),
            prop2: vec![
                NStruct { func1: "NS1f1".to_string(), func2: "NS1f2".to_string(), },
                NStruct { func1: "NS2f1".to_string(), func2: "NS2f2".to_string(), }
            ]
        }
    }
}



#[test]
fn test_from_string_with_vec_struct_and_vec_of_struct() {
    let mut dd = DD::create();
    dd.set_from_inline_string(" 
        nstruct-vec-str-struc: vec(new1,new2,new3) str(new4) struc(func1(str1) func2(str2)); 
        nstruct: func1(new1) func2(new2) func3(new3) bloat1(b);
        append: new_append;
        prop: newpropval1, newpropval2, newpropval3;
        prop2: func1(1deg) func2(2deg), func1(3deg) func2(4deg)".to_string());

    let mut newdd= DD::create();
    newdd.nstruct_vec_str_struc.vec = vec![
        "new1".to_string(),
        "new2".to_string(),
        "new3".to_string(),
    ];
    newdd.nstruct_vec_str_struc.str = "new4".to_string();
    newdd.nstruct_vec_str_struc.struc = NStruct {
        func1: "str1".to_owned(),
        func2: "str2".to_owned(),
    };
    newdd.prop = vec!(
        " newpropval1".to_string(),
        " newpropval2".to_string(),
        " newpropval3".to_string(),
    );
    newdd.append = " new_append".to_string();
    newdd.nstruct = NstructWithStrings { func1: "new1".to_string(), func2: "new2".to_string(), func3: "new3".to_string()};
    newdd.prop2 = vec![
        NStruct { func1: "1deg".to_string(), func2: "2deg".to_string(), },
        NStruct { func1: "3deg".to_string(), func2: "4deg".to_string(), }
    ];


    assert_eq!(dd, newdd);

    // position of css arguments should be irrelevant
    dd.set_from_inline_string(" 
        nstruct-vec-str-struc: str(new4) struc(func1(str1) func2(str2)) bloat() vec(new1,new2,new3); 
        nstruct: func1(new1) func3(new3) bloat1(b) func2(new2);
        append: new_append;
        prop: newpropval1, newpropval2, newpropval3;
        prop2: func1(1deg) func2(2deg), func1(3deg) func2(4deg)".to_string());

    assert_eq!(dd, newdd);

    // number of css arguments should be irrelevant
    dd.set_from_inline_string(" 
        nstruct-vec-str-struc: struc(func1(str1) func2(str2) bloat()) bloat()); 
        nstruct: func1(new1) func3(new3) bloat1(b) func2(new2);
        append: new_append;
        prop: newpropval1, newpropval2, newpropval3;
        prop2: func1(1deg) func2(2deg), func1(3deg) func2(4deg)".to_string());

    assert_eq!(dd, newdd);
}

#[test]
fn faulty_css() {
    let mut dd = DD::create();
    dd.set_from_inline_string(" 
        nstruct-vec-str-struc: vec(new1,new2,new3) str(new4) struc(func1(str1) func2(str2)); 
        nstruct: func1(new1) func2 func3(new3) bloat1(b);
        append: new_append;
        prop: newpropval1, newpropval2, newpropval3;
        prop2: func1(1deg) func2deg), func1(3deg) func2(4deg)".to_string());

    let mut newdd= DD::create();
    newdd.nstruct_vec_str_struc.vec = vec![
        "new1".to_string(),
        "new2".to_string(),
        "new3".to_string(),
    ];
    newdd.nstruct_vec_str_struc.str = "new4".to_string();
    newdd.nstruct_vec_str_struc.struc = NStruct {
        func1: "str1".to_owned(),
        func2: "str2".to_owned(),
    };
    newdd.prop = vec!(
        " newpropval1".to_string(),
        " newpropval2".to_string(),
        " newpropval3".to_string(),
    );
    newdd.append = " new_append".to_string();
    newdd.nstruct = NstructWithStrings { func1: "new1".to_string(), func2: "new2".to_string(), func3: "new3".to_string()};
    newdd.prop2 = vec![
        NStruct { func1: "1deg".to_string(), func2: "2deg".to_string(), },
        NStruct { func1: "3deg".to_string(), func2: "4deg".to_string(), }
    ];


    assert_ne!(dd, newdd);
}