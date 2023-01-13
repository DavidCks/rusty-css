//! This crate can be used to convert rust structs into css, while maintaining
//! the ability to access each field individually.

//mod keywords;
//use keywords::Pseudo;
mod warning;
use bevy_reflect::{Reflect, Struct, ReflectMut, List, Tuple};
use std::{ num::ParseFloatError, cell::Ref };
use web_sys::{ Document };
use substring::*;

// add a smart way to extract the containing float value within a string 
pub trait ExtractNums {

    // tries to extract the numbers from whatever string is given.
    fn try_extract_nums(&self) -> Option<String>;

    // tries to cast any given string to an f64
    fn try_to_f64(&self) -> Result<f64, ParseFloatError>;
}

impl ExtractNums for String {
    fn try_extract_nums(&self) -> Option<String> {
        Some( self.chars().filter(|c| c.is_digit(10) || c == &'.' || c == &'-').collect() )
    }

    fn try_to_f64(&self) -> Result<f64, ParseFloatError> {
        self.try_extract_nums().unwrap().parse::<f64>()
    }
}

// implementations for converting fields in a struct to a css string
pub trait Style: Reflect + Struct {

    // constructor
    fn create() -> Self;

    // Example highly nested css and equivalent rust struct layout
    //                                                                            [, i    def(   lmn (  o,  p,  q  )  j  k  )   g    h )   o    p]
    //    prop:      abc(      i    def(   lmn (  o,  p,  q  )  j  k  )   g    h )   o    p    )   q   r, 
    //   field: Vec<( S { Vec<(S,   (S {  ( S  {  S,  S,  S  }, S, S  ),  S,   S },  S    S)>  },  S,  S)>
    
    fn set_string_reflect(string_reflect: &mut dyn Reflect, value: &str) {
        let new_value = value.trim();
        *string_reflect.downcast_mut::<String>().unwrap() = new_value.into();
    }

    fn set_reflect_caller(reflect_mut: ReflectMut, value: &str) {
        match reflect_mut {
            ReflectMut::Struct(struct_reflect) => {
                Self::set_struct_reflect(struct_reflect, value);
            },
            ReflectMut::Tuple(tuple_reflect) => {
                Self::set_tuple_reflect(tuple_reflect, value);
            },
            ReflectMut::List(list_reflect) => {
                Self::set_list_reflect(list_reflect, value)
            },
            ReflectMut::TupleStruct(_) => todo!(),
            ReflectMut::Array(_) => todo!(),
            ReflectMut::Map(_) => todo!(),
            ReflectMut::Enum(_) => todo!(),
            ReflectMut::Value(string_reflect) => {
                Self::set_string_reflect(string_reflect, value);
            },
        }
    }

    fn set_tuple_reflect(tuple_reflect: &mut dyn Tuple, value: &str) {
        // separate string into vec of tuple values
        let mut breaks = 0;
        let max_breaks = tuple_reflect.field_len() - 1;
        let mut paren_count = 0;
        let formated_tuple_str = value.trim().chars().map(| c | {
            if c == ' ' && paren_count == 0 && breaks < max_breaks{ 
                breaks += 1;
                return "<RustyCssTupleBreak>".to_string();
            } else {
                if c == '(' { paren_count += 1 }
                if c == ')' { paren_count -= 1 }
                return format!("{}", c);
            }
        }).collect::<String>();
        let new_values = formated_tuple_str.split("<RustyCssTupleBreak>").collect::<Vec<&str>>();

        for i in 0..tuple_reflect.field_len() {
            let reflect_mut = tuple_reflect.field_mut(i).unwrap().reflect_mut();
            Self::set_reflect_caller(reflect_mut, new_values[i]);
        }
    }

    fn set_list_reflect(list_reflect: &mut dyn List, value: &str) {
        // separate string into vec of string
        let mut paren_count = 0;
        let formated_vec_str = value.chars().map(| c | {
            if c == '(' { paren_count += 1 }
            if c == ')' { paren_count -= 1 }
            if c == ',' && paren_count == 0 { 
                return "<RustyCssVecBreak>".to_string();
            } else {
                return format!("{}", c);
            }
        }).collect::<String>();
        let new_values = formated_vec_str.split("<RustyCssVecBreak>").collect::<Vec<&str>>();

        // Pop elements according to given css string
        if list_reflect.len() > new_values.len() {
            for _i in new_values.len()..list_reflect.len() {
                list_reflect.pop();
            }
        }

        // iterate list elements and call type appropriate function
        for i in 0..list_reflect.len() {
            let reflect_mut = list_reflect.get_mut(i).unwrap().reflect_mut();
            Self::set_reflect_caller(reflect_mut, new_values[i]);
        }
    }
    
    fn set_struct_reflect(struct_reflect: &mut dyn Struct, value: &str) {
        //value = abc(i def(lmn(o,p,q) j k) g h, i def(lmn() j k) g h) l m,    abc(i def(lmn(o,p,q) j k) g h, i def(lmn(o,p,q) j k) g h) l m
        let func_rest = value.split_once("(").unwrap(); //  "ab-c" "...)"
        let func_name = func_rest.0.trim().replace("-", "_");  //   "ab_c" "...)"
        let field = struct_reflect.field_mut(&func_name);

        // extract argument to css function
        let mut paren_count = 1;
        let mut end_of_argument = 0;
        func_rest.1.chars().for_each(| c | {
            if paren_count == 0 { return; }
            if c == '(' { paren_count += 1 }
            if c == ')' { paren_count -= 1 }
            end_of_argument += 1;
        });
        let args_rest = func_rest.1.split_at(end_of_argument);
        let args = args_rest.0.strip_suffix(")").unwrap();

        // handle css that isnt represented in the struct
        if field.is_none() { println!("no field named {}", func_name); return; }
        
        // call type appropriate function for the field
        let reflect_mut = field.unwrap().reflect_mut();
        Self::set_reflect_caller(reflect_mut, args);

        // recurse if theres "func(args)" left (multiple fields in a struct field of type struct)
        let rest = args_rest.1.trim();
        if !rest.is_empty() { Self::set_struct_reflect(struct_reflect, rest) }
    }

    // mutates a given objects fields to match a given inline css string
    fn set_from_inline_string(&mut self, style: String) where Self: Sized {
        let prop_value = style.split(";"); //[" a: b", " c: d"]
        prop_value.into_iter().for_each(|pv| {
            if pv.trim().is_empty() { return; } // end of the css string

            let prop_value = pv.split_once(":").unwrap();
            let field_name = prop_value.0.replace("-", "_").replace(" ", "").replace("\n", "");

            // if the prop name corresponds to a field name
            if let Some(field) = self.field_mut(&field_name) {
                // call the type appropriate function for the field
                let reflect_mut = field.reflect_mut();
                Self::set_reflect_caller(reflect_mut, prop_value.1);
            }
        });
    }

    // creates a string in the form of an inline style css string
    fn inline(&self) -> String where Self: Sized {
        let mut style_string = "".to_owned();

        //iterate over fields of the component
        for (i, value_reflect) in self.iter_fields().enumerate() {

            //get the name of the structs field as a String
            let mut property_name = self.name_at(i).unwrap().to_owned();
            
            //++++++++++++++++++++++++++++++++++++++++++++
            // horrendous way of implementing reserved keywords
            // but I'm tired now so this will have to do
            //++++++++++++++++++++++++++++++++++++++++++++
            if property_name != "append" {
                property_name = property_name.replace("_", "-");

                //initialize the value to be given for the property in property_name (i.e. width, height, transform, etc) 
                let value = Self::create_value_string(value_reflect);

                style_string.push_str( &format!("{property}: {value}; ", property = property_name, value = value) );
            }
        }

        style_string
    }

    // creates a string for the values behind the css property
    fn create_value_string(reflect: &dyn Reflect) -> String {
        let mut value = "".to_owned();

        match &reflect.reflect_ref() {
            
            //check if the field is a nested struct (i.e. Transform, etc.)
            bevy_reflect::ReflectRef::Struct(fields) => {
                
                //loop over the nested struct                    
                for (i, value_reflect) in fields.iter_fields().enumerate() {
                    //function names like skewX, skewY, etc.
                    let function_name = fields.name_at(i).unwrap().replace("_", "-");
                    let function_param = Self::create_value_string(value_reflect);
                    let value_string = format!(" {function}({parameter})", function = &function_name, parameter = &function_param);
                    value.push_str(&value_string);
                }
            },
            bevy_reflect::ReflectRef::List(arr) => {
                //loop over the vector                    
                for (i, value_reflect) in arr.iter().enumerate() {
                    // dont set the leading comma if its the first element
                    let mut comma = "".to_string();
                    if i != 0 {
                        comma = ", ".to_string();
                    }
                    
                    let value_string = format!("{comma}{value}", value = Self::create_value_string(value_reflect), comma = comma);
                    value.push_str(&value_string);
                }
            },
            bevy_reflect::ReflectRef::Tuple(tuple) => {
                //loop over the Tuple                    
                for (i, value_reflect) in tuple.iter_fields().enumerate() {
                    // dont set the leading comma if its the first element
                    let mut space = "".to_string();
                    if i != 0 {
                        space = " ".to_string();
                    }
                    
                    let value_string = format!("{space}{value}", value = Self::create_value_string(value_reflect), space = space);
                    value.push_str(&value_string);
                }
            }
            //check if the field is a value type (i.e. String, i32, f32, etc.)
            bevy_reflect::ReflectRef::Value(v) => {
                let value_string = v.downcast_ref::<String>().unwrap();
                value.push_str( value_string );
            }
            _ => {
                warning::rust_parse_error::throw(&format!("{:?}", reflect.get_type_info()));
            }
        }

        value
    }


    fn as_class_string(&self, class_name: &String) -> Result<String, &'static str> where Self: Sized {

        let mut class_name_appended = class_name.clone();
        // append pseudo-class name to the class name (i.e. .struct_ident:pseudo_class)
        let append = self.field("append");
        if !append.is_none() {
            class_name_appended.push_str(
                append.unwrap().downcast_ref::<String>().unwrap()
            );
        }
           
        Ok( format!(".{} {{ {}}}", class_name_appended, self.inline()) )
    }

    fn as_class(&self, document: &Document) -> Result<String, &'static str> where Self: Sized {
        
        // get struct name as class name
        let class_name = self.get_struct_name().unwrap();

        // create the string that is supposed to be inserted into the head as class
        let class_string = self.as_class_string(&class_name).expect("Class string could not be created");

        // insert the class
        self.append_to_head(&document, &class_name, &class_string);
        
        // return just the class name
        Ok(class_name)
    }

    fn append_to_head(&self, document: &Document, class_name: &String, class_string: &String) where Self: Sized {
        let head = document.head().expect("No <head> element found in the document");
        let new_style_element = document.create_element("style").expect("couldn't create <style> element in this document");
        let style_id = format!("rusty-css-{}", class_name.replace(":", "_"));
        new_style_element.set_attribute("id", &style_id ).expect("couldn't set attribute of internally created style tag");
        new_style_element.set_text_content(Some(&class_string));

        if let Some(existent_style) = head.query_selector(&format!("#{}", style_id) ).expect("an error occured while trying to fetch the element with id `rusty-css` in head") {
            head.remove_child(&existent_style).expect("couldn't remove child element with id `rusty-css` in head");
        }

        head.append_child(&new_style_element).expect("couldn't append internally created `style` element with id `rusty-css` to head");
    }

    fn add_as_pseudo_class(&self, document: &Document) where Self: Sized {
        
        let mut class_name = self.get_struct_name().unwrap();
        class_name = class_name.replace("_", ":");

        let class_string = self.as_class_string(&class_name).expect("Class string could not be created");

        self.append_to_head(document, &class_name, &class_string);
    }

    fn get_struct_name(&self) -> Result<String, &'static str> where Self: Sized {
        
        // cuts off the "arbitrary_caller_name::" from "arbitrary_caller_name::StructIdent and returns just the StructIdent"
        let class_name_pos = self.type_name().rfind("::").expect("(Internal Error) couldn't find position of `::` in type_name");
        let class_name_slice = self.type_name().substring(class_name_pos + 2, self.type_name().len());
        if class_name_slice == "" { return Err("(Internal Error) couldn't strip arbitrary_caller_name:: prefix"); }
        
        Ok(class_name_slice.to_owned())
    }

    fn debug(self) -> Self where Self: Sized {
        
        wasm_logger::init(wasm_logger::Config::default());

        for (_i, value_reflect) in self.iter_fields().enumerate() {
            log::info!("{:?}", value_reflect.get_type_info());
        }

        self
    }
}