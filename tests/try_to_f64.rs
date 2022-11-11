use rusty_css::*;

#[test]
fn test() {

    assert_eq!("123px".to_owned().try_to_f64().unwrap(), 123_f64);
    assert_eq!("-123px".to_owned().try_to_f64().unwrap(), -123_f64);
}