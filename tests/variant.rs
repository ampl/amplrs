use amplrs::Variant;

#[test]
fn numeric_variant_round_trip() {
    let v = Variant::new_from_double(3.5);
    assert_eq!(3.5, v.get_numeric());
    assert_eq!("", v.get_string());
}

#[test]
fn string_variant_round_trip() {
    let v = Variant::new_from_string("hello");
    assert_eq!("hello", v.get_string());
}

#[test]
fn string_variant_format_is_quoted() {
    let v = Variant::new_from_string("hello");
    assert_eq!("'hello'", v.format());
}

#[test]
fn numeric_variant_format() {
    let v = Variant::new_from_double(42.0);
    assert_eq!("42", v.format());
}

#[test]
fn empty_variant_get_string_is_safe() {
    let v = Variant::new();
    assert_eq!("", v.get_string());
    assert_eq!("EMPTY", v.format());
}
