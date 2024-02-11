use ctrlc;
use std::fs::OpenOptions;
use std::io::{stderr, stdout, Read, Write};
use std::process::{Child, ExitStatus};
use std::sync::mpsc::channel;
use std::{thread, time};

pub fn child_wait_with_tee(
    child: &mut Child, out_file: &str, err_file: &str, quiet: bool,
) -> Option<ExitStatus> {
    fn io_pipe_reader(
        mut stream: impl Read, filename: String, mut output: impl Write, quiet: bool,
    ) -> std::io::Result<()> {
        let mut file = OpenOptions::new().write(true).create(true).append(true).open(filename)?;

        let mut buf = [0u8; 1024];
        loop {
            let num_read = stream.read(&mut buf)?;
            if num_read == 0 {
                break;
            }

            let buf = &buf[..num_read];
            file.write_all(buf)?;

            if !quiet {
                output.write_all(buf)?;
            }
        }

        Ok(())
    }

    // Register CTRL+C
    let (ctrlc_tx, ctrlc_rx) = channel();

    ctrlc::set_handler(move || ctrlc_tx.send(()).expect("failed to send on ctrlc tx channel"))
        .expect("failed to set Ctrl-C handler");

    // Read Child Output
    let child_out = std::mem::take(&mut child.stdout).expect("cannot attach to child stdout");
    let child_err = std::mem::take(&mut child.stderr).expect("cannot attach to child stderr");

    let out_file = out_file.to_string();
    let err_file = err_file.to_string();

    let thread_out = thread::spawn(move || {
        io_pipe_reader(child_out, out_file, stdout(), quiet)
            .expect("error communicating with child stdout")
    });
    let thread_err = thread::spawn(move || {
        io_pipe_reader(child_err, err_file, stderr(), quiet)
            .expect("error communicating with child stderr")
    });

    let mut status: Option<ExitStatus> = None;
    while status.is_none() {
        if ctrlc_rx.try_recv().is_ok() {
            eprintln!("Ctrl-C handled, killing ...");

            child.kill().expect("failed to kill child process");

            break;
        }

        match child.try_wait() {
            Ok(None) => {
                // Sleep 100ms.
                thread::sleep(time::Duration::from_millis(100));
            }
            Ok(s) => {
                status = s;
            }
            Err(err) => {
                eprintln!("error while waiting on child: {err:?}");
                break;
            }
        }
    }

    thread_out.join().unwrap();
    thread_err.join().unwrap();

    status
}
