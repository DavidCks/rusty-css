<h1><center><b>rusty-css</b></center></h1>

<center>
    <p>
    <a href="https://github.com/DavidCks/rusty-css/actions?query=branch%3Amain">
        <img src="https://github.com/DavidCks/rusty-css/workflows/Rust%20CI/badge.svg"
            alt="Rust CI badge">
    </a>
    </p>
</center>

## a rusty way to implement css in rust

rusty-css offers a solution to create and export css styles in a familiar way, but without leaving the rust syntax.
You can access and manipulate every value you define on an individual basis.

The identifiers are taken as-is and converted into strings, with the exception of the underscore token (_), which is converted into the minus token (-) when the struct is to be converted into its corresponding css values. 

Example:

```rust
struct A {
    css_property: "css value" 
}
```
will be converted into
```rust
"css-property: css value"
```

regardless of the property names' or values' validity. If you have an error in your css, it will still compile!

### Roadmap

- [x] simple conversion from rust structs to inline css code
- [ ] more reliable extraction of numeric values inside of a String
- [x] exporting structs as classes inside a style sheet
    - [ ] support for queries
    - [ ] make exporting classes less boilerplate-y
- [ ] validating the written css code at compile time
    - [ ] automated implementation according to the css spec 
- [ ] second layer implementation of a system with strict typing 
(such as enums for all possible units for a given property)
- [ ] more abstraction for less boilderplate

## How to use

As of now this crate uses the bevy_reflect crate to convert the structs into css "property: value" strings, so all structs that you wish to convert must derive [Reflect](https://docs.rs/bevy/latest/bevy/reflect/)

```rust
use bevy_reflect::{Reflect};

#[derive(Reflect)]
struct ExampleStruct {
    width: String,
    height: String,
    background: String,
    transform: String,
}
```

to convert this struct into an inline css string we have to first implement the struct and its initial state. For this we implement the **Style** Trait from this crate.

```rust
use rusty_css::*;

impl Style for ExampleStruct {
    fn create() -> Self {
        // return an instance of Self, so in this case an instance of ExampleStruct
        Self { 
            width: "4em".to_string(),
            height: "2rem".to_string(),
            background: "rgb(69,13,37)".to_string(),
            transform: "skewX(20deg) skewY(30deg)".to_string(),
        }
    }
}
```

now we can create an instance of ExampleStruct and convert it into a css inline string.

```rust

let example_struct = ExampleStruct::create();
let inline_css: String = example_struct.inline();
// "width: 4em; height: 2rem; background: rgb(69,13,37); transform: skewX(20deg) skewY(30deg);"
```
## Developer experiance improvements

since it ca be hard to access values of a property that can take multiple values such as **transform**, we can instead implement a nested struct into our original struct.
By doing so, the fields of the struct in the second layer will no longer be treated as if they're css properties but rather css fuctions that take an argument.

```rust
#[allow(non_snake_case)] // so the skewX field doesn't throw a warning for being in snake case, which css uses
#[derive(Reflect)]
struct NestedTransformStruct {
    skewX: String,
    skewY: String,
}

#[derive(Reflect)]
struct ExampleStruct {
    width: String,
    height: String,
    background: String,
    transform: NestedTransformStruct,
}

impl Style for ExampleStruct {
    fn create() -> Self {
        // return an instance of Self, so in this case an instance of ExampleStruct
        Self { 
            width: "4em".to_string(),
            height: "2rem".to_string(),
            background: "rgb(69,13,37)".to_string(),
            transform: NestedTransformStruct {
                skewX: "20deg".to_string(),
                skewY: "30deg".to_string(),
            },
        }
    }
}

let example_struct = ExampleStruct::create();
let inline_css: String = example_struct.inline();
let skewX: String = example_struct.transform.skewX; // can access this field, wuhu!
// "width: 4em; height: 2rem; background: rgb(69,13,37); transform: skewX(20deg) skewY(30deg);"
```
The output is equivalend but whe can access the elements skewX and skewY fields individually now.
Following that logic, you should be able to write the background fields value similarly, so lets try it:

```rust
#[derive(Reflect)]
struct Background {
    rgb: String,
}

#[derive(Reflect)]
struct ExampleStruct {
    width: String,
    height: String,
    background: Background,
}

impl Style for ExampleStruct {
    fn create() -> Self {
        // return an instance of Self, so in this case an instance of ExampleStruct
        Self { 
            width: "4em".to_string(),
            height: "2rem".to_string(),
            background: Background {
                rgb: "69,13,37".to_string(),
            }
        }
    }
}

let example_struct = ExampleStruct::create();
let inline_css: String = example_struct.inline();
// "width: 4em; height: 2rem; background: rgb(69,13,37);"
```
Works just fine!

You might have noticed that we're appending a lot of .to_string() calls. At scale this can become quite cumbersome, so i created the [append_to_string](https://crates.io/crates/append_to_string) crate, which helps with that.

## Complete Example

with all that out of the way, here's what your code might look like:

```rust
use rusty_css::*;
use append_to_string::*;
use bevy_reflect::{Reflect};

// define all the structs we want to be css-ified 

#[allow(non_snake_case)]
#[derive(Reflect)]
struct NestedTransformStruct {
    skewX: String,
    skewY: String,
}

#[derive(Reflect)]
struct ExampleStruct {
    width: String,
    height: String,
    background: String,
    transform: NestedTransformStruct,
}

impl Style for ExampleStruct {
    fn create() -> Self {
        // return an instance of Self, so in this case an instance of ExampleStruct
        append_to_string!( 
            Self { 
                width: "4em",
                height: "2rem",
                background: "rgb(69,13,37)",
                transform: NestedTransformStruct {
                    skewX: "20deg",
                    skewY: "30deg",
                },
            }
        )
    }
}

let example_struct = ExampleStruct::create();
let inline_css: String = example_struct.inline();
// "width: 4em; height: 2rem; background: rgb(69,13,37); transform: skewX(20deg) skewY(30deg);"
```

## Implementing Styles as Classes

Use the ```.as_class()``` function on your struct to export its style into the ```<style>``` tag of your app. For now you'll have to pass it a reference to the ```web_sys::Document``` you want to export the style into like so:
```rust
let style_struct = ExampleStruct::create();

// grab the current document
let window = web_sys::window().expect("No global `window` found");
let document = window.document().expect("couldn't get `document");

// export the style to the <style> tag
let style_struct = ExampleStruct::create();
let class_name = style_struct.as_class(&document).unwrap();

assert_eq!("ExampleStruct", class_name); //true
```

in a yew component it might look like this:
```rust
fn view(&self, ctx: &Context<Self>) -> Html {
    let window = window().expect("No global `window` found");
    let document = window.document().expect("couldn't get `document");

    let class_name = self.style.as_class(&document).unwrap();

    html! {
        <div class={class_name}
        </div>
    }
}
```

### Crate implements:

```rust
trait ExtractNums {
    // tries to extract the numbers from whatever string is given.
    fn try_extract_nums(&self) -> Option<String>;

    // tries to cast any given string to an f64
    fn try_to_f64(&self) -> Result<f64, ParseFloatError>;
}

trait Style {
    // returns the inline css representaton of a struct
    fn inline(&self) -> String 

    // Sets a structs' fields' values equivalent to the given inline css string
    // Struct { struct_field: "value" }.set_from_inline_string("struct-field: different value");
    // will result in:
    // Struct { struct_field: "different value"}
    // (also works for nested structs)
    fn set_from_inline_string(&self, style: String) -> &Self

    // retruns the String Representation of a fields value
    fn create_value_string(reflect: &dyn Reflect) -> String;

    // - returns the class name to put into the class attribute
    // - inserts the style as a class into the style sheet
    fn as_class(&mut self, document: &Document) -> Result<String, &'static str> ;

    // retruns the struct as a css class String like so: .StructIdent { property: value }
    fn as_class_string(&mut self, mut class_name: String) -> Result<String, &'static str>;

    // returns the struct name
    fn get_struct_name(&self) -> Result<String, &'static str>;

    // logs the Reflects of the given objects fields to the browser console with wasm_logger 
    fn debug(self) -> Self;
}
```

### Reserved field names:

```rust
append: "this will be appended to the class name" // for classes that are supposed to be exported with a psuedo-class (i.e. :before, :active, etc.) 

```