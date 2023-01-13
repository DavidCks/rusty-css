pub fn throw(rust_type_info: &str) {
    let warning_message = "rusty-css:\nWarning: couldn't translate this part of the rust code:";
    let suggestion_1 = "Suggestion 1: You can only use Strings, Structs, Vecs and Tuples.\n".to_owned() + 
                       "Here are the rules for rust to css conversion:\n" + 
                       "|                       Rust                       |     CSS      |\n" +
                       "| Struct { a: String = 'value'                   } | a: value;    |\n" +
                       "| Struct { a: (String, String) = ('val', 'ue')   } | a: val ue;   |\n" +
                       "| Struct { a: Vec<String> = vec!['v', 'a', 'l']  } | a: v, a, l;  |\n" +
                       "| Struct { a: Nested { b: String = value }       } | a: b(value); |\n";
    
    let warning = format!("{} {}\n{}", warning_message, rust_type_info, suggestion_1);
    log::warn!("{}", warning);
    println!("{}", warning);
}