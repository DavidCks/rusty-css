pub fn throw(css_string: &str) {
    let warning_message = "rusty-css:\nWarning: couldn't parse this part of the css string:";
    let suggestion_1 = "Suggestion 1: Check your parentheses";
    let suggestion_2 = "Suggestion 2: Check your css with your browsers DevTools for validity";
    
    let warning = format!("{} {}\n{}\n{}", warning_message, css_string, suggestion_1, suggestion_2);
    log::warn!("{}", warning);
    println!("{}", warning);
}