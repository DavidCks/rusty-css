//! This crate can be used to convert rust structs into css, while maintaining
//! the ability to access each field individually.

use bevy_reflect::{Reflect, Struct};
use std::num::ParseFloatError;

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
            if property_name != "pseudo_class" {
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

    fn as_class(&mut self) -> Result<String, &'static str> where Self: Sized {

        // stores the structs ident
        let class_name_opt = self.type_name().strip_prefix("as_class::"); 
        if class_name_opt.is_none() { return Err("(Internal Error) couldn't strip as_class:: prefix"); }
        
        let mut class_name = class_name_opt.unwrap().to_owned();

        // append pseudo-class name to the class name (i.e. .struct_ident:pseudo_class)
        let pseudo = self.field("pseudo_class");
        if !pseudo.is_none() {
            class_name.push(':');
            class_name.push_str(
                pseudo.unwrap().downcast_ref::<String>().unwrap()
            );
        }
           
        let class_string = format!(".{} {{ {}}}", class_name, self.inline());
        Ok(class_string)
    }

    fn debug(self) -> Self where Self: Sized {
        
        wasm_logger::init(wasm_logger::Config::default());

        for (_i, value_reflect) in self.iter_fields().enumerate() {
            log::info!("{:?}", value_reflect.get_type_info());
        }

        self
    }
}