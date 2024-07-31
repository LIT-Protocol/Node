use std::backtrace::Backtrace;
use std::panic;

use env_logger::Env;
use log::{as_error, as_serde, error};

use lit_core::error::{Error, Kind};
use lit_core::utils::backtrace::{backtrace_to_vec, extract_panic_msg};
use lit_os_guest_initrd::init;
use lit_os_guest_initrd::logging::LogFormatter;

#[tokio::main]
async fn main() {
    // Init logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| LogFormatter::format("lit-os-init", buf, record))
        .init();

    // Init panic hook
    panic::set_hook(Box::new(move |e| {
        let msg = extract_panic_msg(e);
        let backtrace = Backtrace::force_capture();
        let backtrace = backtrace_to_vec(&backtrace);

        let source: Option<String> = None;
        let err = Error::new(
            Some(Kind::Unexpected),
            "lit-os-init",
            Some(msg.clone()),
            None,
            source,
            None,
        );

        error!(error = as_error!(err), backtrace = as_serde!(backtrace);
                    "Unexpectedly panicked!: {}", msg);
    }));

    // Run init
    init::init().await;
}
