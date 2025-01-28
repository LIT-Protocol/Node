use std::backtrace::Backtrace;
use std::panic::PanicHookInfo;

pub fn backtrace_to_vec(backtrace: &Backtrace) -> Vec<String> {
    let backtrace_str = format!("{backtrace}");
    let backtrace: Vec<String> =
        backtrace_str.split('\n').map(|s| s.trim().to_string()).filter(|s| !s.eq("")).collect();

    backtrace
}

pub fn extract_panic_msg(info: &PanicHookInfo) -> String {
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
        s.to_owned()
    } else {
        "Unknown cause (payload not &str or &String)".to_string()
    }
}
