pub fn bool_option_to_bool(val: Option<&bool>) -> bool {
    match val {
        Some(val) => *val,
        None => false,
    }
}
