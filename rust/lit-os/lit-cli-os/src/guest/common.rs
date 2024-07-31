pub(crate) const MANIFEST_FILE: &str = "release-mf.toml";
pub(crate) const MANIFEST_PROOF_FILE: &str = "release-mf.proof";

pub fn print_describe_string(key: &str, val: Option<&String>) {
    if let Some(val) = val {
        println!("{key:<18} {val}");
    }
}

pub fn print_describe_str(key: &str, val: Option<&str>) {
    if let Some(val) = val {
        println!("{key:<18} {val}");
    }
}
