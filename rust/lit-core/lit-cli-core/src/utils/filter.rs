pub fn matches_str_filter(filter: Option<&str>, value: Option<&str>) -> bool {
    match filter {
        Some(filter) => {
            match value {
                Some(value) => {
                    // TODO: Support pattern matching.
                    filter.eq_ignore_ascii_case(value)
                }
                _ => false,
            }
        }
        _ => true,
    }
}
