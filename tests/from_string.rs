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

// String & Struct Test

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
    newbb.prop_with_underscore = "new_underscore_val".to_string(); 

    assert_eq!(bb, newbb);
}

#[test]
fn test_from_string() {
    let mut bb = BB::create();
    bb.set_from_inline_string("prop: newpropval;".to_string());

    let mut newbb = BB::create();
    newbb.prop = "newpropval".to_string(); 

    assert_eq!(bb, newbb);
}

// Vec Test

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
        "newpropval1".to_string(),
        "newpropval2".to_string(),
        "newpropval3".to_string(),
    ];


    assert_eq!(cc, newcc);
}

// Tuple Test

#[derive(Reflect, FromReflect, PartialEq, Debug)]
struct RGBT {
    rgb: Vec<String>,
}

#[derive(Reflect, PartialEq, Debug)]
struct BG {
    linear_gradient: Vec<(RGBT, String, String)>,
}

#[derive(Reflect, PartialEq, Debug)]
struct StructWithVecOfTupleOfNStructAndTuple {
    border: (String, String, String),
    background_image: BG,
}


impl Style for StructWithVecOfTupleOfNStructAndTuple {
    fn create() -> Self {
        Self {
            border: ("thick".to_string(), "double".to_string(), "#32a1ce".to_string()),
            background_image: BG { 
                linear_gradient: vec![
                    (
                        RGBT {
                            rgb: vec!["1".to_string(), "2".to_string(), "3".to_string()],
                        }, 
                        "0%".to_string(), 
                        "50%".to_string()
                    )
                ] 
            }
        }
    }
}

#[test]
fn test_from_string_with_tuple() {
    let mut tt = StructWithVecOfTupleOfNStructAndTuple::create();
    tt.set_from_inline_string("border: donts topmen owowowo; background-image: linear-gradient(rgb(4,5,6) 1% 51%);".to_string());

    let mut newtt = StructWithVecOfTupleOfNStructAndTuple::create();
    newtt.background_image.linear_gradient[0].0.rgb = vec!["4".to_string(), "5".to_string(), "6".to_string()];
    newtt.background_image.linear_gradient[0].1 = "1%".to_string();
    newtt.background_image.linear_gradient[0].2 ="51%".to_string(); 
    newtt.border.0 = "donts".to_string();
    newtt.border.1 = "topmen".to_string();
    newtt.border.2 = "owowowo".to_string();
    

    assert_eq!(tt, newtt);
}


// All Test

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
        "newpropval1".to_string(),
        "newpropval2".to_string(),
        "newpropval3".to_string(),
    );
    newdd.append = "new_append".to_string();
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
        nstruct: func1(new1) func2() func3(new3) bloat1(b);
        append: new_append;
        prop: newpropval1, newpropval2, newpropval3;
        prop2: func1(1deg) func2deg), func1(3deg func2(4deg)".to_string());

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
        "newpropval1".to_string(),
        "newpropval2".to_string(),
        "newpropval3".to_string(),
    );
    newdd.append = "new_append".to_string();
    newdd.nstruct = NstructWithStrings { func1: "new1".to_string(), func2: "new2".to_string(), func3: "new3".to_string()};
    newdd.prop2 = vec![
        NStruct { func1: "1deg".to_string(), func2: "2deg".to_string(), },
        NStruct { func1: "3deg".to_string(), func2: "4deg".to_string(), }
    ];


    assert_ne!(dd, newdd);
}

#[derive(Reflect, FromReflect)]
struct RGBA {
    rgba: Vec<String>,
}

impl Style for RGBA {
    fn create() -> Self {
        Self {
            rgba: vec!["127".to_string(), "127".to_string(), "127".to_string(), "1".to_string()],
        }
    }
}

#[derive(Reflect)]
struct Gradient {
    linear_gradient: (String, Vec<(RGBA, String)>),
}

impl Style for Gradient {
    fn create() -> Self {
        let mut color_1 = RGBA::create();
        let mut color_2 = RGBA::create();
        let mut color_3 = RGBA::create();
        color_1.rgba[0] = "255".to_string();
        color_2.rgba[1] = "255".to_string();
        color_3.rgba[2] = "255".to_string();

        let pos_1 = "33%".to_string(); 
        let pos_2 = "67%".to_string(); 
        let pos_3 = "100%".to_string(); 

        let gradients = vec![
            (color_1, pos_1), 
            (color_2, pos_2), 
            (color_3, pos_3)
        ];

        let rotation = "9000deg,".to_string();

        Self {
            linear_gradient: (rotation, gradients),
        }
    }
}

#[derive(Reflect)]
struct Realistic<> {
    background: RGBA,
    background_image: Gradient,
}

impl Style for Realistic {
    fn create() -> Self {
        let background = RGBA::create();
        let gradient = Gradient::create();
        Self { background, background_image: gradient}
    }
}


// real world example test
#[test]
fn realistic_example() {
    let css = "background: rgba(0, 255, 250, 1); background-image: linear-gradient(90deg, rgba(5, 97, 179, 1) 29%, rgba(34, 25, 0, 1) 56%, rgba(5, 97, 179, 1) 78%, rgba(34, 25, 0, 1) 56%, rgba(5, 97, 179, 1) 78%); ";
    let output_css = "background: rgba(0, 255, 250, 1); background-image: linear-gradient(90deg, rgba(5, 97, 179, 1) 29%, rgba(34, 25, 0, 1) 56%, rgba(5, 97, 179, 1) 78%); ";

    let mut realistic_struct = Realistic::create();
    realistic_struct.set_from_inline_string(css.into());

    assert_eq!(output_css,realistic_struct.inline());
}