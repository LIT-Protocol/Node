use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value::String;

use lit_core::config::envs::LitEnv;

use crate::error::{validation_err, Result};
use crate::guest::types::GuestType;

pub const VALID_HOSTNAME_SUFFIX: &str = ".litos-guest.litnet.io";

const VALID_HOSTNAME_PART_RE_STR: &str = "[a-z0-9]+[a-z0-9-]*[a-z0-9]*";

pub static VALID_HOSTNAME_PART_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(format!(r"^{}$", VALID_HOSTNAME_PART_RE_STR).as_str())
        .expect("failed to construct regex for hostname part validation")
});

pub static VALID_HOSTNAME_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        format!(
            r"^(prov|build|node)[-](dev|staging|prod)[-]({})[.]litos-guest[.]litnet[.]io$",
            VALID_HOSTNAME_PART_RE_STR
        )
        .as_str(),
    )
    .expect("failed to construct regex for hostname validation")
});

pub static VALID_CUSTOM_HOSTNAME_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        format!(
            r"^(.+?)[-](dev|staging|prod)[-]({})[.]litos-guest[.]litnet[.]io$",
            VALID_HOSTNAME_PART_RE_STR
        )
        .as_str(),
    )
    .expect("failed to construct regex for hostname validation")
});

pub static VALID_IMAGE_SIZE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(\d+)(G)$").expect("failed to construct regex for hostname validation")
});

pub static VALID_LABEL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9:_-]+").expect("failed to construct regex for label validation")
});

pub fn validate_host_name_part(part: &str, max_len: Option<usize>) -> Result<()> {
    if part.is_empty() {
        return Err(validation_err("invalid length for hostname part", None));
    }

    if let Some(max_len) = max_len {
        if part.len() > max_len {
            return Err(validation_err(
                format!("invalid length for hostname part, must be < {} chars long", max_len),
                None,
            )
            .add_field("hostname_part", String(part.to_string())));
        }
    }

    if !VALID_HOSTNAME_PART_RE.is_match(part) {
        return Err(validation_err("hostname part is not a valid for lit-os guests", None)
            .add_field("hostname_part", String(part.to_string())));
    }

    Ok(())
}

pub fn validate_host_name(
    host: &str, guest_type: Option<&GuestType>, env: Option<&LitEnv>, instance_id: Option<&str>,
    guest_kind: Option<&str>,
) -> Result<()> {
    if host.is_empty() {
        return Err(validation_err("invalid length for hostname", None));
    }
    if !host.ends_with(VALID_HOSTNAME_SUFFIX) {
        return Err(validation_err(
            "hostname is not a valid for lit-os guests (invalid domain)", None,
        ));
    }

    let hostname_re = if matches!(guest_type, Some(GuestType::Custom)) {
        VALID_CUSTOM_HOSTNAME_RE.clone()
    } else {
        VALID_HOSTNAME_RE.clone()
    };

    if let Some(matches) = hostname_re.captures(host) {
        // Verify guest_type
        if let Some(guest_type) = guest_type {
            if let Some(matched) = matches.get(1) {
                if GuestType::Custom.eq(guest_type) {
                    if let Some(guest_kind) = guest_kind {
                        if !guest_kind.eq(matched.as_str()) {
                            return Err(validation_err(
                                format!(
                                    "hostname '{host}' does not contain provided guest_kind '{guest_kind}'"
                                ),
                                None,
                            )
                                .add_field("hostname", String(host.to_string())));
                        }
                    }
                } else if !guest_type.eq_str(matched.as_str()) {
                    return Err(validation_err(
                        format!(
                            "hostname '{host}' does not contain provided guest_type '{guest_type}'"
                        ),
                        None,
                    )
                    .add_field("hostname", String(host.to_string())));
                }
            } else {
                return Err(validation_err(
                    "hostname is not a valid for lit-os guests (failed to extract guest_type)",
                    None,
                )
                .add_field("hostname", String(host.to_string())));
            }
        }

        // Verify env
        if let Some(env) = env {
            if let Some(matched) = matches.get(2) {
                if !env.eq_str(matched.as_str()) {
                    return Err(validation_err(
                        format!("hostname '{host}' does not contain provided env '{env}'"),
                        None,
                    )
                    .add_field("hostname", String(host.to_string())));
                }
            } else {
                return Err(validation_err(
                    "hostname is not a valid for lit-os guests (failed to extract env)",
                    None,
                )
                .add_field("hostname", String(host.to_string())));
            }
        }

        if let Some(instance_id) = instance_id {
            // Verify instance_id
            if let Some(matched) = matches.get(3) {
                if let Some(m_instance_id) = matched.as_str().split('-').next() {
                    if !m_instance_id.eq(instance_id) {
                        return Err(validation_err(
                            format!(
                                "hostname '{host}' does not contain provided instance id '{instance_id}'"
                            ),
                            None,
                        ).add_field("hostname", String(host.to_string())));
                    }
                } else {
                    return Err(validation_err(
                        "hostname is not a valid for lit-os guests (failed to extract instance-id, split on '-' failed)",
                        None,
                        ).add_field("hostname", String(host.to_string())
                    ));
                }
            } else {
                return Err(validation_err(
                    "hostname is not a valid for lit-os guests (failed to extract instance-id)",
                    None,
                )
                .add_field("hostname", String(host.to_string())));
            }
        } else {
            // Verify instance_id or name is at least present
            if matches.get(3).is_none() {
                return Err(validation_err(
                    "hostname is not a valid for lit-os guests (does not contain instance-id / name)",
                    None,
                ).add_field("hostname", String(host.to_string())));
            }
        }
    } else {
        return Err(validation_err(
            "hostname is not a valid for lit-os guests (does not match)", None,
        )
        .add_field("hostname", String(host.to_string())));
    }

    Ok(())
}

pub fn validate_image_size(size: &str) -> Result<()> {
    if size.is_empty() {
        return Err(validation_err("invalid image size str", None));
    }

    if !VALID_IMAGE_SIZE_RE.is_match(size) {
        return Err(validation_err("img size is invalid (must be of format: 0-9[G])", None)
            .add_field("size", String(size.to_string())));
    }

    Ok(())
}

pub fn validate_label(label: &str) -> Result<()> {
    if label.is_empty() {
        return Err(validation_err("invalid label str", None));
    }

    if label.len() > 20 {
        return Err(validation_err("invalid label str, must be < 20 chars long", None)
            .add_field("label", String(label.to_string())));
    }

    if !VALID_LABEL_RE.is_match(label) {
        return Err(validation_err("label is invalid (must only contain a-zA-Z0-9:_-)", None)
            .add_field("label", String(label.to_string())));
    }

    Ok(())
}
