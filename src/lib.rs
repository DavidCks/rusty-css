//! This crate can be used to convert rust structs into css, while maintaining
//! the ability to access each field individually.

//mod keywords;
//use keywords::Pseudo;
use bevy_reflect::{Reflect, Struct};
use std::num::ParseFloatError;
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

                style_string.push_str( &format!("{property}:{value}; ", property = property_name, value = value) );
            }
        }

        style_string
    }

    // creates a string for the values behind the css property
    fn create_value_string(reflect: &dyn Reflect) -> String {
        let mut value = "".to_owned();

        match &reflect.reflect_ref() {
            //check if the field is a value type (i.e. String, i32, f32, etc.)
            bevy_reflect::ReflectRef::Value(v) => {
                let value_string = " ".to_owned() + v.downcast_ref::<String>().unwrap();
                value.push_str( &value_string );
            }

            //check if the field is a nested struct (i.e. Transform, etc.)
            bevy_reflect::ReflectRef::Struct(fields) => {

                //loop over the nested struct                    
                for (i, value_reflect) in fields.iter_fields().enumerate() {
                    //function names like skewX, skewY, etc.
                    let function_name = fields.name_at(i).unwrap();
                    let function_param = Self::create_value_string(value_reflect);
                    let value_string = format!(" {function}({parameter})", function = &function_name, parameter = &function_param);
                    value.push_str(&value_string);
                }
            }
            _ => {
                log::warn!("The given Object is only allowed to have String fields or Structs with String fields. \nGot: {:?}", &reflect.get_type_info());
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