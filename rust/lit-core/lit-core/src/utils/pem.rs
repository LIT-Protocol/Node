// Taken from pkix which is not compatible with OSX M1.
// Modified to use Result vs Option (Option is impossible to debug).

#[cfg(feature = "asn1")]
use asn1_rs::{BitString, FromBer, Integer, OctetString, Sequence};

#[allow(unused_imports)]
use crate::error::{conversion_err, parser_err, Result};

/// Type of the various `PEM_*` constants supplied to `pem_to_der` / `der_to_pem`.
pub struct PemGuard {
    begin: &'static str,
    end: &'static str,
}

macro_rules! pem_guard {
    ($n:expr) => {
        &PemGuard {
            begin: concat!("-----BEGIN ", $n, "-----"),
            end: concat!("-----END ", $n, "-----"),
        }
    };
}

// Ref. RFC7468, although these are not universally respected.
pub const PEM_CERTIFICATE: &PemGuard = pem_guard!("CERTIFICATE");
pub const PEM_CERTIFICATE_REQUEST: &PemGuard = pem_guard!("CERTIFICATE REQUEST");
pub const PEM_ENCRYPTED_PRIVATE_KEY: &PemGuard = pem_guard!("ENCRYPTED PRIVATE KEY");
pub const PEM_PRIVATE_KEY: &PemGuard = pem_guard!("PRIVATE KEY");
pub const PEM_EC_PRIVATE_KEY: &PemGuard = pem_guard!("EC PRIVATE KEY");
pub const PEM_PUBLIC_KEY: &PemGuard = pem_guard!("PUBLIC KEY");
pub const PEM_CMS: &PemGuard = pem_guard!("CMS");

/// Convert PEM to DER. If `guard` is specified (e.g. as PEM_CERTIFICATE), then the guardlines are
/// verified to match the expected string. Otherwise, the guardlines are verified to generally have
/// the correct form.
///
/// On failure (due to guardlines syntax or an illegal PEM character), returns None.
pub fn pem_to_der(pem: &str, guard: Option<&PemGuard>) -> Result<Vec<u8>> {
    let mut lines = pem.lines();

    let begin = match lines.next() {
        Some(l) => l,
        None => return Err(parser_err("missing first line", None)),
    };
    let end = match lines.last() {
        Some(l) => l,
        None => return Err(parser_err("missing last line", None)),
    };

    if let Some(g) = guard {
        if begin != g.begin || end != g.end {
            return Err(parser_err("does not match guard", None));
        }
    } else if !begin.starts_with("-----BEGIN ")
        || !begin.ends_with("-----")
        || !end.starts_with("-----END ")
        || !end.ends_with("-----")
    {
        return Err(parser_err("does not have correct BEGIN or END lines", None));
    }

    // TODO: Address this.
    #[allow(clippy::skip_while_next)]
    let body_start =
        pem.char_indices().skip(begin.len()).skip_while(|t| t.1.is_whitespace()).next().unwrap().0;
    let body_end = pem.rmatch_indices(&end).next().unwrap().0;

    // Extra processing (had some issues otherwise).
    let body = &pem[body_start..body_end].trim().replace(['\r', '\n'], "");

    base64::decode(body).map_err(|e| parser_err(e, Some("failed to decode base64".into())))
}

/// Convert DER to PEM. The guardlines use the identifying string chosen by `guard`
/// (e.g. PEM_CERTIFICATE).
pub fn der_to_pem<T: ?Sized + AsRef<[u8]>>(der: &T, guard: &PemGuard) -> String {
    let mut pem = String::new();
    let b64_string = base64::encode(der.as_ref());

    pem.push_str(guard.begin);
    pem.push('\n');
    if !der.as_ref().is_empty() {
        pem.push_str(b64_string.as_str());
        pem.push('\n');
    }
    pem.push_str(guard.end);
    pem.push('\n');

    pem
}

#[cfg(feature = "asn1")]
pub fn ec_key_pem_to_bytes(pem: &str) -> Result<Vec<u8>> {
    let der = pem_to_der(pem, Some(PEM_EC_PRIVATE_KEY))?;

    let (_rem, sequence) = Sequence::from_ber(&der[..])
        .map_err(|e| conversion_err(e, Some("failed to extract Sequence from der".into())))?;

    let (rem, _integer) = Integer::from_ber(&sequence.content).map_err(|e| {
        conversion_err(e, Some("failed to extract Integer from der Sequence".into()))
    })?;

    let (_rem, octet) = OctetString::from_ber(rem).map_err(|e| {
        conversion_err(e, Some("failed to extract OctetString from der Sequence".into()))
    })?;

    Ok(octet.as_ref().to_vec())
}

#[cfg(feature = "asn1")]
pub fn ec_pub_pem_to_bytes(pem: &str) -> Result<Vec<u8>> {
    let der = pem_to_der(pem, Some(PEM_PUBLIC_KEY))?;

    let (_rem, sequence) = Sequence::from_ber(&der[..])
        .map_err(|e| conversion_err(e, Some("failed to extract Sequence from der".into())))?;

    let (rem, _sequence) = Sequence::from_ber(&sequence.content).map_err(|e| {
        conversion_err(e, Some("failed to extract Sequence from der Sequence".into()))
    })?;

    let (_rem, bs) = BitString::from_ber(rem).map_err(|e| {
        conversion_err(e, Some("failed to extract BitString from der Sequence".into()))
    })?;

    Ok(bs.as_ref().to_vec())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use lit_core::utils::pem::pem_to_der;

    #[allow(unused_imports)]
    use crate::utils::binary::bytes_to_hex;
    use crate::utils::pem::PEM_EC_PRIVATE_KEY;
    #[cfg(feature = "asn1")]
    use crate::utils::pem::{ec_key_pem_to_bytes, ec_pub_pem_to_bytes};

    const RESOURCES_TEST_DIR: &str = "resources/test/utils/pem";

    #[test]
    fn pem_to_der_test() {
        let test_file = get_test_path("build.pem");
        let test_pem = fs::read_to_string(test_file.as_path()).unwrap();

        let res = pem_to_der(test_pem.as_str(), Some(PEM_EC_PRIVATE_KEY)).unwrap();

        // ASN1 in DER
        assert_eq!(res.len(), 118);
    }

    #[test]
    #[cfg(feature = "asn1")]
    fn ec_key_pem_to_bytes_test() {
        let test_file = get_test_path("build.pem");
        let test_pem = fs::read_to_string(test_file.as_path()).unwrap();

        let res = ec_key_pem_to_bytes(test_pem.as_str()).unwrap();

        assert_eq!(
            bytes_to_hex(&res),
            "7521bfc19cba9ab641797a518bbae94fa6eb7af67dcd3d2abdffdb09e9fbe3ad"
        );
    }

    #[test]
    #[cfg(feature = "asn1")]
    fn ec_pub_pem_to_bytes_test() {
        let test_file = get_test_path("build-pub.pem");
        let test_pem = fs::read_to_string(test_file.as_path()).unwrap();

        let res = ec_pub_pem_to_bytes(test_pem.as_str()).unwrap();

        assert_eq!(bytes_to_hex(&res), "04ee0b2578a435fd327c803698bb7c75e463f92629e0b25189ff37396b6a5c656077d25f4d7fa23fb28196430b6e238e80a50cd05b775e8513cd2045222c9a4fd6");
    }

    // Util
    fn get_test_path(path: &str) -> PathBuf {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push(RESOURCES_TEST_DIR);
        test_path.push(path);
        test_path
    }
}
