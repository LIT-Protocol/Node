pub fn unescape_debug_output<S>(val: S) -> String
where
    S: AsRef<str>,
{
    let val = val.as_ref();
    if let Some(v) = val.strip_prefix('\"') {
        if let Some(v) = v.strip_suffix('\"') {
            return v.to_string();
        }
    }

    val.to_string()
}
