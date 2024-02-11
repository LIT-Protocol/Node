use crate::error::{io_err, Result};
use std::io::{stdin, stdout, Write};
use termion::input::TermRead;

pub fn confirm<S: AsRef<str>>(prompt: S, default: bool) -> Result<bool> {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut hint = "[Y/n]";
    if !default {
        hint = "[N/y]";
    }

    stdout
        .write_fmt(format_args!("{} {}: ", prompt.as_ref(), hint))
        .map_err(|e| io_err(e, None))?;
    stdout.flush().map_err(|e| io_err(e, None))?;

    if let Ok(Some(val)) = stdin.read_line() {
        if default {
            if "n".eq_ignore_ascii_case(&val) || "no".eq_ignore_ascii_case(&val) {
                return Ok(false);
            }
        } else if "y".eq_ignore_ascii_case(&val) || "yes".eq_ignore_ascii_case(&val) {
            return Ok(true);
        }
    }

    Ok(default)
}
