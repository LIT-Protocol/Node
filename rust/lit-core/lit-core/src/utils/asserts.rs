pub fn string_option_is_defined(val: Option<&String>) -> bool {
    match val {
        Some(val) => !val.is_empty(),
        _ => false,
    }
}

pub fn string_options_match(a: Option<&String>, b: Option<&String>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a.eq(b),
        (None, None) => true,
        _ => false,
    }
}

pub fn bool_options_match(a: Option<&bool>, b: Option<&bool>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a == b,
        (None, None) => true,
        _ => false,
    }
}
