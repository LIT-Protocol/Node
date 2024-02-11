use crate::error::{generic_err, Result};
use std::collections::HashMap;

pub fn parse_cmdline(cmdline: &str) -> Result<HashMap<String, String>> {
    let mut res: HashMap<String, String> = HashMap::new();

    for part in cmdline.split(' ') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let mut kv_parts = part.split('=');

        match (kv_parts.next(), kv_parts.next()) {
            (Some(key), Some(val)) => {
                res.insert(key.to_string(), val.to_string());
            }
            (Some(key), None) => {
                res.insert(key.to_string(), "".to_string());
            }
            _ => {
                return Err(generic_err(
                    format!("parse_cmdline failed: part without '=' found: {part}"),
                    None,
                ));
            }
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::utils::cmdline::parse_cmdline;

    const TEST_CMDLINE: &str = "console=ttyS0 earlyprintk=serial root=/dev/disk/by-uuid/3D96BCA8-3054-4D3C-B000-B2386B3784E1 ro litos.build_id=0e71e9a6 litos.type=prov litos.env=dev \
    litos.roothash=e3a6fb7acc3e1ce7781d3ed65ff7b28b205332e219009dddd25501aaa6715dfe litos.opt_ro=0 litos.opt_users=1 litos.opt_ssh=1 litos.subnet_id=aA7aD6F5EAc8bF4bAe5CC03295559723677EcA6c \
    litos.release_id=0e71e9a6aA7aD6F5EAc8bF4bAe5CC03295559723677EcA6c litos.release_env=bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi initrd=initrd";

    #[test]
    fn parse_cmdline_test() {
        let res = parse_cmdline(TEST_CMDLINE).expect("failed to parse cmdline");

        assert_eq!(res.get("console"), Some(&"ttyS0".to_string()));
        assert_eq!(res.get("earlyprintk"), Some(&"serial".to_string()));
        assert_eq!(
            res.get("root"),
            Some(&"/dev/disk/by-uuid/3D96BCA8-3054-4D3C-B000-B2386B3784E1".to_string())
        );
        assert_eq!(res.get("ro"), Some(&"".to_string()));
        assert_eq!(res.get("litos.build_id"), Some(&"0e71e9a6".to_string()));
        assert_eq!(res.get("litos.type"), Some(&"prov".to_string()));
        assert_eq!(res.get("litos.env"), Some(&"dev".to_string()));
        assert_eq!(
            res.get("litos.roothash"),
            Some(&"e3a6fb7acc3e1ce7781d3ed65ff7b28b205332e219009dddd25501aaa6715dfe".to_string())
        );
        assert_eq!(res.get("litos.opt_ro"), Some(&"0".to_string()));
        assert_eq!(res.get("litos.opt_users"), Some(&"1".to_string()));
        assert_eq!(res.get("litos.opt_ssh"), Some(&"1".to_string()));
        assert_eq!(
            res.get("litos.subnet_id"),
            Some(&"aA7aD6F5EAc8bF4bAe5CC03295559723677EcA6c".to_string())
        );
        assert_eq!(
            res.get("litos.release_id"),
            Some(&"0e71e9a6aA7aD6F5EAc8bF4bAe5CC03295559723677EcA6c".to_string())
        );
        assert_eq!(
            res.get("litos.release_env"),
            Some(&"bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string())
        );
        assert_eq!(res.get("initrd"), Some(&"initrd".to_string()));
    }
}
