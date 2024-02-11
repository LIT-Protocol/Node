use lit_core::config::envs::LitEnv;
use lit_core::config::LitConfig;
use lit_core::utils::asserts::{
    bool_options_match, string_option_is_defined, string_options_match,
};
use lit_core::utils::option::bool_option_to_bool;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use crate::config::LitOsGuestConfig;
use crate::error::{config_err, conversion_err, generic_err, io_err, Result};
use crate::guest::env::build::GuestBuildEnv;
use crate::guest::types::GuestType;
use crate::utils::cmdline::parse_cmdline;

pub const PROC_CMDLINE: &str = "/proc/cmdline";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[allow(unused)]
pub struct GuestCmdLine {
    pub console: Option<String>,
    pub earlyprintk: Option<String>,
    pub initrd: Option<String>,
    pub root: Option<String>,
    pub ro: bool,
    pub build_id: Option<String>,
    pub build_type: Option<String>,
    pub build_kind: Option<String>,
    pub build_env: Option<String>,
    pub build_roothash: Option<String>,
    pub build_varhhash: Option<String>,
    pub build_opt_ro: Option<bool>,
    pub build_opt_users: Option<bool>,
    pub build_opt_ssh: Option<bool>,
    pub subnet_id: Option<String>,
    pub release_id: Option<String>,
    pub release_env: Option<String>,
}

impl GuestCmdLine {
    pub fn verify(
        &self, expect_varhash: bool, expect_subnet: bool, expect_release: bool,
    ) -> Result<()> {
        if !string_option_is_defined(self.root.as_ref()) {
            return Err(generic_err("GuestCmdLine root is not defined", None));
        }
        if !string_option_is_defined(self.build_id.as_ref()) {
            return Err(generic_err("GuestCmdLine build_id is not defined", None));
        }
        if !string_option_is_defined(self.build_type.as_ref()) {
            return Err(generic_err("GuestCmdLine build_type is not defined", None));
        }
        if !string_option_is_defined(self.build_env.as_ref()) {
            return Err(generic_err("GuestCmdLine build_env is not defined", None));
        }
        if !string_option_is_defined(self.build_roothash.as_ref()) {
            return Err(generic_err("GuestCmdLine build_roothash is not defined", None));
        }
        if bool_option_to_bool(self.build_opt_ro.as_ref()) != self.ro {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_opt_ro ('{:?}') != ro ('{}')",
                    self.build_opt_ro.as_ref(),
                    self.ro
                ),
                None,
            ));
        }
        if expect_varhash && !string_option_is_defined(self.build_varhhash.as_ref()) {
            return Err(generic_err("GuestCmdLine build_varhhash is not defined", None));
        }
        if expect_subnet && !string_option_is_defined(self.subnet_id.as_ref()) {
            return Err(generic_err("GuestCmdLine subnet_id is not defined", None));
        }
        if expect_release {
            if !string_option_is_defined(self.release_id.as_ref()) {
                return Err(generic_err("GuestCmdLine release_id is not defined", None));
            }
            if !string_option_is_defined(self.release_env.as_ref()) {
                return Err(generic_err("GuestCmdLine release_env is not defined", None));
            }

            let build_id = self.build_id.as_ref().unwrap();
            let release_id = self.release_id.as_ref().unwrap();

            if !release_id.starts_with(build_id) {
                return Err(generic_err(
                    format!(
                        "GuestCmdLine release_id does not match build_id prefix: '{release_id}' vs '{build_id:?}'"
                    ),
                    None,
                ));
            }

            if expect_subnet {
                let subnet_id = self.subnet_id.as_ref().unwrap();
                let expected_prefix = format!("{build_id}{subnet_id}");

                if !release_id.starts_with(&expected_prefix) {
                    return Err(generic_err(format!("GuestCmdLine release_id does not match '<build_id><subnet_id>' prefix: '{release_id}' vs '{expected_prefix:?}'"), None));
                }
            }
        }

        // Custom
        let guest_type = self.guest_type()?;
        if GuestType::Custom.eq(&guest_type) && !string_option_is_defined(self.build_kind.as_ref())
        {
            return Err(generic_err(
                "GuestCmdLine build_kind is not defined (required for custom)",
                None,
            ));
        }

        Ok(())
    }

    pub fn matches_build_env(&self, build_env: &GuestBuildEnv) -> Result<()> {
        if !string_options_match(self.build_id.as_ref(), build_env.build_id.as_ref()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_id does not match build.env: '{:?}' vs '{:?}'",
                    self.build_id.as_ref(),
                    build_env.build_id.as_ref()
                ),
                None,
            ));
        }
        if !string_options_match(self.build_type.as_ref(), build_env.build_type.as_ref()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_type does not match build.env: '{:?}' vs '{:?}'",
                    self.build_type.as_ref(),
                    build_env.build_type.as_ref()
                ),
                None,
            ));
        }
        if !string_options_match(self.build_kind.as_ref(), build_env.build_kind.as_ref()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_kind does not match build.env: '{:?}' vs '{:?}'",
                    self.build_kind.as_ref(),
                    build_env.build_kind.as_ref()
                ),
                None,
            ));
        }
        if !string_options_match(self.build_env.as_ref(), build_env.build_release.as_ref()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_env does not match build.env: '{:?}' vs '{:?}'",
                    self.build_env.as_ref(),
                    build_env.build_release.as_ref()
                ),
                None,
            ));
        }
        if !bool_options_match(self.build_opt_ro.as_ref(), build_env.build_opt_ro.as_ref()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_opt_ro does not match build.env: '{:?}' vs '{:?}'",
                    self.build_opt_ro.as_ref(),
                    build_env.build_opt_ro.as_ref()
                ),
                None,
            ));
        }
        if !bool_options_match(self.build_opt_users.as_ref(), build_env.build_opt_users.as_ref()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_opt_users does not match build.env: '{:?}' vs '{:?}'",
                    self.build_opt_users.as_ref(),
                    build_env.build_opt_users.as_ref()
                ),
                None,
            ));
        }
        if !bool_options_match(self.build_opt_ssh.as_ref(), build_env.build_opt_ssh.as_ref()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_opt_ssh does not match build.env: '{:?}' vs '{:?}'",
                    self.build_opt_ssh.as_ref(),
                    build_env.build_opt_ssh.as_ref()
                ),
                None,
            ));
        }

        Ok(())
    }

    pub fn matches_lit_cfg(&self, cfg: &LitConfig) -> Result<()> {
        if !cfg.env().eq_str(self.build_env.as_ref().unwrap()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_env does not match lit cfg: '{:?}' vs '{:?}'",
                    self.build_env.as_ref(),
                    cfg.env()
                ),
                None,
            ));
        }
        if !string_options_match(cfg.litos_build_id().ok().as_ref(), self.build_id.as_ref()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_id does not match lit cfg: '{:?}' vs '{:?}'",
                    self.build_id.as_ref(),
                    cfg.litos_build_id().ok().as_ref()
                ),
                None,
            ));
        }
        if !string_options_match(cfg.litos_build_type().ok().as_ref(), self.build_type.as_ref()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_type does not match lit cfg: '{:?}' vs '{:?}'",
                    self.build_type.as_ref(),
                    cfg.litos_build_type().ok().as_ref()
                ),
                None,
            ));
        }
        if !string_options_match(cfg.litos_build_kind().ok().as_ref(), self.build_kind.as_ref()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_kind does not match lit cfg: '{:?}' vs '{:?}'",
                    self.build_kind.as_ref(),
                    cfg.litos_build_kind().ok().as_ref()
                ),
                None,
            ));
        }
        if !bool_options_match(cfg.litos_build_opt_ro().ok().as_ref(), self.build_opt_ro.as_ref()) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_opt_ro does not match lit cfg: '{:?}' vs '{:?}'",
                    self.build_opt_ro.as_ref(),
                    cfg.litos_build_opt_ro().ok().as_ref()
                ),
                None,
            ));
        }
        if !bool_options_match(
            cfg.litos_build_opt_users().ok().as_ref(),
            self.build_opt_users.as_ref(),
        ) {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_opt_users does not match lit cfg: '{:?}' vs '{:?}'",
                    self.build_opt_users.as_ref(),
                    cfg.litos_build_opt_users().ok().as_ref()
                ),
                None,
            ));
        }
        if !bool_options_match(cfg.litos_build_opt_ssh().ok().as_ref(), self.build_opt_ssh.as_ref())
        {
            return Err(generic_err(
                format!(
                    "GuestCmdLine build_opt_ssh does not match lit cfg: '{:?}' vs '{:?}'",
                    self.build_opt_ssh.as_ref(),
                    cfg.litos_build_opt_ssh().ok().as_ref()
                ),
                None,
            ));
        }

        Ok(())
    }

    // Accessors
    pub fn env(&self) -> Result<LitEnv> {
        let env_str = self
            .build_env
            .as_ref()
            .ok_or_else(|| config_err("unexpected: GuestCmdLine missing build_env", None))?
            .as_str();

        LitEnv::from_str(env_str).map_err(|e| conversion_err(e, None))
    }

    pub fn guest_type(&self) -> Result<GuestType> {
        let build_type_str = self
            .build_type
            .as_ref()
            .ok_or_else(|| config_err("unexpected: GuestCmdLine missing build_type", None))?
            .as_str();

        GuestType::from_str(build_type_str).map_err(|e| conversion_err(e, None))
    }
}

impl Default for GuestCmdLine {
    fn default() -> Self {
        Self {
            console: None,
            earlyprintk: None,
            initrd: None,
            root: None,
            ro: false,
            build_id: None,
            build_type: None,
            build_kind: None,
            build_env: None,
            build_roothash: None,
            build_varhhash: None,
            build_opt_ro: Some(true),
            build_opt_users: Some(false),
            build_opt_ssh: Some(false),
            subnet_id: None,
            release_id: None,
            release_env: None,
        }
    }
}

pub fn read_guest_cmdline() -> Result<Option<GuestCmdLine>> {
    let filename = PathBuf::from(PROC_CMDLINE);

    if !filename.exists() {
        return Ok(None);
    }

    let cmdline = fs::read_to_string(filename).map_err(|e| io_err(e, None))?;

    load_guest_cmdline(cmdline)
}

pub fn load_guest_cmdline(cmdline: String) -> Result<Option<GuestCmdLine>> {
    let mut env = GuestCmdLine::default();

    let env_data = parse_cmdline(cmdline.as_str())?;

    for (key, value) in env_data {
        match key.as_str() {
            "console" => env.console = Some(value),
            "earlyprintk" => env.earlyprintk = Some(value),
            "initrd" => env.initrd = Some(value),
            "root" => env.root = Some(value),
            "ro" => env.ro = true,
            "litos.build_id" => env.build_id = Some(value),
            "litos.type" => env.build_type = Some(value),
            "litos.kind" => env.build_kind = Some(value),
            "litos.env" => env.build_env = Some(value),
            "litos.roothash" => env.build_roothash = Some(value),
            "litos.varhash" => env.build_varhhash = Some(value),
            "litos.opt_ro" => env.build_opt_ro = Some(value != "0"),
            "litos.opt_users" => env.build_opt_users = Some(value == "1"),
            "litos.opt_ssh" => env.build_opt_ssh = Some(value == "1"),
            "litos.subnet_id" => env.subnet_id = Some(value),
            "litos.release_id" => env.release_id = Some(value),
            "litos.release_env" => env.release_env = Some(value),
            _ => {}
        }
    }

    Ok(Some(env))
}

#[cfg(test)]
mod tests {
    use crate::guest::env::cmdline::load_guest_cmdline;

    const TEST_CMDLINE: &str = "console=ttyS0 earlyprintk=serial root=/dev/disk/by-uuid/3D96BCA8-3054-4D3C-B000-B2386B3784E1 litos.build_id=0e71e9a6 litos.type=prov litos.kind=salt-master litos.env=dev \
    litos.roothash=e3a6fb7acc3e1ce7781d3ed65ff7b28b205332e219009dddd25501aaa6715dfe litos.opt_ro=0 litos.opt_users=1 litos.opt_ssh=1 litos.subnet_id=aA7aD6F5EAc8bF4bAe5CC03295559723677EcA6c \
    litos.release_id=0e71e9a6aA7aD6F5EAc8bF4bAe5CC03295559723677EcA6c litos.release_env=bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi initrd=initrd";

    #[test]
    fn load_guest_cmdline_test() {
        let cmdline_res =
            load_guest_cmdline(TEST_CMDLINE.to_string()).expect("failed to load guest cmdline");
        assert!(cmdline_res.is_some());

        let cmdline_env = cmdline_res.unwrap();

        assert_eq!(cmdline_env.console, Some("ttyS0".to_string()));
        assert_eq!(cmdline_env.earlyprintk, Some("serial".to_string()));
        assert_eq!(cmdline_env.initrd, Some("initrd".to_string()));
        assert_eq!(
            cmdline_env.root,
            Some("/dev/disk/by-uuid/3D96BCA8-3054-4D3C-B000-B2386B3784E1".to_string())
        );
        assert_eq!(cmdline_env.build_id, Some("0e71e9a6".to_string()));
        assert_eq!(cmdline_env.build_type, Some("prov".to_string()));
        assert_eq!(cmdline_env.build_kind, Some("salt-master".to_string()));
        assert_eq!(cmdline_env.build_env, Some("dev".to_string()));
        assert_eq!(
            cmdline_env.build_roothash,
            Some("e3a6fb7acc3e1ce7781d3ed65ff7b28b205332e219009dddd25501aaa6715dfe".to_string())
        );
        assert_eq!(cmdline_env.build_opt_ro, Some(false));
        assert_eq!(cmdline_env.build_opt_users, Some(true));
        assert_eq!(cmdline_env.build_opt_ssh, Some(true));
        assert_eq!(
            cmdline_env.subnet_id,
            Some("aA7aD6F5EAc8bF4bAe5CC03295559723677EcA6c".to_string())
        );
        assert_eq!(
            cmdline_env.release_id,
            Some("0e71e9a6aA7aD6F5EAc8bF4bAe5CC03295559723677EcA6c".to_string())
        );
        assert_eq!(
            cmdline_env.release_env,
            Some("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string())
        );
    }
}
