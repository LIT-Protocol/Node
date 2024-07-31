use std::collections::HashMap;
use std::io;
use std::process::{Command, Stdio};

// MAC 02:2b:a7:a4:78:df IP 172.30.0.107 HOSTNAME prov-dev-dd472725 BEGIN 2022-10-26 17:47:59 END 2022-10-26 17:57:59 MANUFACTURER -NA-
pub struct DhcpLease {
    pub mac: String,
    pub ip: String,
    pub hostname: String,
}

pub fn get_dhcpd_leases() -> io::Result<HashMap<String, DhcpLease>> {
    let mut cmd = Command::new("dhcp-lease-list");
    cmd.arg("--parsable").stdout(Stdio::piped()).stderr(Stdio::piped());

    let out = cmd.output()?;
    let lines = String::from_utf8(out.stdout).expect("failed to convert output to utf8");

    // Ensure consistency.
    let lines = lines.replace("\r\n", "\n");

    // Read output
    let mut res: HashMap<String, DhcpLease> = HashMap::new();
    for line in lines.split('\n') {
        let mut mac: Option<String> = None;
        let mut ip: Option<String> = None;
        let mut hostname: Option<String> = None;

        let parts: Vec<&str> = line.split(' ').collect();
        let pc = parts.len();
        let mut pi = 0;

        // Test against one back from the end to ensure parts[pi+1] works.
        while pi < (pc - 1) {
            let cur = parts[pi];
            let next = parts[pi + 1].to_string();

            match cur {
                "MAC" => mac = Some(next),
                "IP" => ip = Some(next),
                "HOSTNAME" => hostname = Some(next),
                _ => {
                    if mac.is_some() && ip.is_some() && hostname.is_some() {
                        break;
                    }
                }
            }

            // Next
            pi += 2;
        }

        if mac.is_some() && ip.is_some() && hostname.is_some() {
            let mac = mac.take().unwrap();

            res.insert(
                mac.clone(),
                DhcpLease { mac, ip: ip.take().unwrap(), hostname: hostname.take().unwrap() },
            );
        }
    }

    Ok(res)
}
