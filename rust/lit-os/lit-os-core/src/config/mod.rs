use config::Value;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

use crate::error::{config_err, Result};
use lit_core::config::{LitConfig, LitConfigBuilder};
use lit_core::utils::toml::SimpleToml;

pub const BUILD_PRIV_KEY_PATH: &str = "/etc/ssl/private/build.pem";

pub const CFG_KEY_LITOS_INSTALL_DIR: &str = "litos.install_dir";
pub const CFG_KEY_LITOS_VAR_DIR: &str = "litos.var_dir";

pub const CFG_KEY_GUEST_ALLOWED_CFG_KEYS: &str = "guest.allowed_cfg_keys";
pub const CFG_KEY_GUEST_INSTANCE_ID: &str = "guest.instance.id";
pub const CFG_KEY_GUEST_RELEASE_ID: &str = "guest.release.id";

pub const DEFAULT_LITOS_INSTALL_DIR: &str = "/opt/lit/os";
pub const DEFAULT_LITOS_VAR_DIR: &str = "/var/lit/os";

pub const DEV_DISK: &str = "/dev/sda";
pub const DEV_CLOUD_INIT: &str = "/dev/vda";
pub const DEV_ONESHOT: &str = "/dev/disk/by-partlabel/one-shot";

pub const MNT_CLOUT_INIT: &str = "/mnt/cloud-init";
pub const MNT_ONESHOT: &str = "/mnt/oneshot";

pub const MNT_INITRD_ROOT: &str = "/mnt/root";

pub const MAX_USER_CONFIG_VAL_LEN: usize = 4096;
pub const GUEST_INSTANCE_ID_LEN: usize = 8;

pub const ENV_BUILD_PRIV_KEY_PATH: &str = "LIT_BUILD_PRIV_KEY_PATH";

pub trait LitOsConfig {
    fn apply_defaults(builder: LitConfigBuilder) -> Result<LitConfigBuilder>;
    fn litos_install_dir(&self) -> Result<String>;
    fn litos_var_dir(&self) -> Result<String>;
    fn litos_var_guest_dir(&self) -> Result<String>;
    fn litos_var_guest_build_dir(&self) -> Result<String>;
    fn litos_var_guest_images_dir(&self) -> Result<String>;
    fn litos_var_guest_templates_dir(&self) -> Result<String>;
    fn litos_var_guest_instances_dir(&self) -> Result<String>;
    fn litos_var_guest_releases_dir(&self) -> Result<String>;
}

impl LitOsConfig for LitConfig {
    #[inline]
    fn apply_defaults(mut builder: LitConfigBuilder) -> Result<LitConfigBuilder> {
        // Set defaults
        builder = builder
            .set_default(CFG_KEY_LITOS_INSTALL_DIR, DEFAULT_LITOS_INSTALL_DIR)
            .set_default(CFG_KEY_LITOS_VAR_DIR, DEFAULT_LITOS_VAR_DIR);

        Ok(builder)
    }

    // Install (bins)
    #[inline]
    fn litos_install_dir(&self) -> Result<String> {
        self.get_string(CFG_KEY_LITOS_INSTALL_DIR)
    }

    // Var (data)
    #[inline]
    fn litos_var_dir(&self) -> Result<String> {
        self.get_string(CFG_KEY_LITOS_VAR_DIR)
    }

    #[inline]
    fn litos_var_guest_dir(&self) -> Result<String> {
        Ok(format!("{}/guest", self.litos_var_dir()?))
    }

    #[inline]
    fn litos_var_guest_build_dir(&self) -> Result<String> {
        Ok(format!("{}/build", self.litos_var_guest_dir()?))
    }

    #[inline]
    fn litos_var_guest_images_dir(&self) -> Result<String> {
        Ok(format!("{}/images", self.litos_var_guest_dir()?))
    }

    #[inline]
    fn litos_var_guest_templates_dir(&self) -> Result<String> {
        Ok(format!("{}/templates", self.litos_var_guest_dir()?))
    }

    #[inline]
    fn litos_var_guest_instances_dir(&self) -> Result<String> {
        Ok(format!("{}/instances", self.litos_var_guest_dir()?))
    }

    #[inline]
    fn litos_var_guest_releases_dir(&self) -> Result<String> {
        Ok(format!("{}/releases", self.litos_var_guest_dir()?))
    }
}

pub trait LitOsGuestConfig {
    fn apply_defaults(builder: LitConfigBuilder) -> Result<LitConfigBuilder>;
    fn litos_guest_allowed_cfg_keys(&self) -> Result<Vec<Value>>;
    fn litos_guest_instance_id(&self) -> Result<String>;
    fn litos_guest_release_id(&self) -> Result<String>;
    fn litos_build_id(&self) -> Result<String>;
    fn litos_build_type(&self) -> Result<String>;
    fn litos_build_kind(&self) -> Result<String>;
    fn litos_build_unix(&self) -> Result<String>;
    fn litos_build_date(&self) -> Result<String>;
    fn litos_build_priv_key_path(&self) -> PathBuf;
    fn litos_build_opt_ro(&self) -> Result<bool>;
    fn litos_build_opt_users(&self) -> Result<bool>;
    fn litos_build_opt_ssh(&self) -> Result<bool>;
    fn litos_disk_dev(&self) -> PathBuf;
    fn litos_cloud_init_dev(&self) -> PathBuf;
    fn litos_cloud_init_mnt(&self) -> PathBuf;
    fn litos_oneshot_dev(&self) -> PathBuf;
    fn litos_oneshot_mnt(&self) -> PathBuf;
    fn litos_initrd_root_mnt(&self) -> PathBuf;
    fn litos_initrd_var_mnt(&self) -> PathBuf;
    fn verify_litos_guest_user_cfg_file(&self, path: &Path) -> Result<()>;
    fn verify_litos_guest_user_cfg(&self, user_cfg_map: &SimpleToml) -> Result<()>;
    fn verify_litos_guest_instance_id(&self) -> Result<()>;
}

impl LitOsGuestConfig for LitConfig {
    #[inline]
    fn apply_defaults(mut builder: LitConfigBuilder) -> Result<LitConfigBuilder> {
        // Set defaults
        builder = <LitConfig as LitOsConfig>::apply_defaults(builder)?;

        Ok(builder)
    }

    #[inline]
    fn litos_guest_allowed_cfg_keys(&self) -> Result<Vec<Value>> {
        self.get_array(CFG_KEY_GUEST_ALLOWED_CFG_KEYS)
    }

    #[inline]
    fn litos_guest_instance_id(&self) -> Result<String> {
        self.get_string(CFG_KEY_GUEST_INSTANCE_ID)
    }

    #[inline]
    fn litos_guest_release_id(&self) -> Result<String> {
        self.get_string(CFG_KEY_GUEST_RELEASE_ID)
    }

    #[inline]
    fn litos_build_id(&self) -> Result<String> {
        self.get_string("build.id")
    }

    #[inline]
    fn litos_build_type(&self) -> Result<String> {
        self.get_string("build.type")
    }

    #[inline]
    fn litos_build_kind(&self) -> Result<String> {
        self.get_string("build.kind")
    }

    #[inline]
    fn litos_build_unix(&self) -> Result<String> {
        self.get_string("build.unix")
    }

    #[inline]
    fn litos_build_date(&self) -> Result<String> {
        self.get_string("build.date")
    }

    #[inline]
    fn litos_build_priv_key_path(&self) -> PathBuf {
        if let Ok(path) = env::var(ENV_BUILD_PRIV_KEY_PATH) {
            PathBuf::from(path)
        } else {
            PathBuf::from(BUILD_PRIV_KEY_PATH)
        }
    }

    fn litos_build_opt_ro(&self) -> Result<bool> {
        self.get_bool("build.opt_ro")
    }

    #[inline]
    fn litos_build_opt_users(&self) -> Result<bool> {
        self.get_bool("build.opt_users")
    }

    #[inline]
    fn litos_build_opt_ssh(&self) -> Result<bool> {
        self.get_bool("build.opt_ssh")
    }

    #[inline]
    fn litos_disk_dev(&self) -> PathBuf {
        PathBuf::from(DEV_DISK)
    }

    #[inline]
    fn litos_cloud_init_dev(&self) -> PathBuf {
        PathBuf::from(DEV_CLOUD_INIT)
    }

    #[inline]
    fn litos_cloud_init_mnt(&self) -> PathBuf {
        PathBuf::from(MNT_CLOUT_INIT)
    }

    #[inline]
    fn litos_oneshot_dev(&self) -> PathBuf {
        PathBuf::from(DEV_ONESHOT)
    }

    #[inline]
    fn litos_oneshot_mnt(&self) -> PathBuf {
        PathBuf::from(MNT_ONESHOT)
    }

    #[inline]
    fn litos_initrd_root_mnt(&self) -> PathBuf {
        PathBuf::from(MNT_INITRD_ROOT)
    }

    #[inline]
    fn litos_initrd_var_mnt(&self) -> PathBuf {
        PathBuf::from(MNT_INITRD_ROOT).join("var")
    }

    // Verify

    fn verify_litos_guest_user_cfg_file(&self, path: &Path) -> Result<()> {
        let user_cfg_map = SimpleToml::try_from(path)
            .map_err(|e| config_err(e, Some("error loading user cfg".into())))?;

        self.verify_litos_guest_user_cfg(&user_cfg_map)
    }

    fn verify_litos_guest_user_cfg(&self, user_cfg_map: &SimpleToml) -> Result<()> {
        let allowed_keys = self.litos_guest_allowed_cfg_keys().map_err(|e| {
            config_err(e, Some(format!("missing required key: '{CFG_KEY_GUEST_ALLOWED_CFG_KEYS}'")))
        })?;

        // Load allowed keys into a map
        let mut allowed: HashMap<String, bool> = HashMap::new();
        for val in allowed_keys {
            let val = val.into_string().map_err(|e| {
                config_err(
                    e,
                    Some(format!(
                        "invalid value in key: '{CFG_KEY_GUEST_ALLOWED_CFG_KEYS}' (not a string)"
                    )),
                )
            })?;

            allowed.insert(val, true);
        }

        // Verify the keys found
        for (key, val) in user_cfg_map.data() {
            for (sub_key, val) in val {
                let full_key = format!("{key}.{sub_key}");
                if !allowed.contains_key(&full_key) {
                    return Err(config_err(format!("contains prohibited key '{full_key}'"), None));
                }

                if val.len() > MAX_USER_CONFIG_VAL_LEN {
                    return Err(config_err(
                        format!("contains value exceeding allowed size for '{full_key}'"),
                        None,
                    ));
                }
            }
        }

        Ok(())
    }

    fn verify_litos_guest_instance_id(&self) -> Result<()> {
        let instance_id_missing_err = format!("missing config: {CFG_KEY_GUEST_INSTANCE_ID}");
        let instance_id = self
            .litos_guest_instance_id()
            .map_err(|e| config_err(e, Some(instance_id_missing_err.clone())))?;
        if instance_id.is_empty() {
            return Err(config_err(instance_id_missing_err, None));
        }
        if instance_id.len() != GUEST_INSTANCE_ID_LEN {
            return Err(config_err(
                format!("invalid config: {CFG_KEY_GUEST_INSTANCE_ID} (invalid length)"),
                None,
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::LitOsGuestConfig;
    use lit_core::config::envs::LitEnv;
    use lit_core::config::{LitConfigBuilder, CFG_EXT, CFG_NAME};
    use std::path::PathBuf;

    const RESOURCES_TEST_DIR: &str = "resources/test/config";

    #[test]
    fn verify_litos_guest_user_cfg_ok_test() {
        let system_path = get_test_path("guest_init/system");
        let system_path_str = system_path.to_str().unwrap();

        let mut init_path = get_test_path("guest_init/user-ok");
        init_path.push(format!("{CFG_NAME}.{CFG_EXT}"));

        // Note init path isn't given here.
        let cfg = LitConfigBuilder::new_with_paths(
            Some("keyed".to_string()),
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            "/tmp/fake/nope",
        )
        .build()
        .expect("failed to load config");

        assert_eq!(cfg.env().clone(), LitEnv::Dev);

        // Test verify
        cfg.verify_litos_guest_user_cfg_file(init_path.as_path())
            .expect("failed to verify guest config");
    }

    #[test]
    fn verify_litos_guest_user_cfg_invalid_test() {
        let system_path = get_test_path("guest_init/system");
        let system_path_str = system_path.to_str().unwrap();

        for (test, exp_err) in [
            ("user-invalid-types1", "invalid type: boolean `false`"),
            ("user-invalid-types2", "invalid type: integer `90210`"),
            ("user-invalid-types3", "invalid type: sequence, expected a map"),
            ("user-invalid-types4", "invalid type: map, expected a string"),
            ("user-invalid-prohibited", "contains prohibited key 'top.five'"),
        ] {
            println!("test: {test}");

            let mut init_path = get_test_path(format!("guest_init/{test}").as_str());
            init_path.push(format!("{CFG_NAME}.{CFG_EXT}"));

            // Note init path isn't given here.
            let cfg = LitConfigBuilder::new_with_paths(
                Some("keyed".to_string()),
                Some("/tmp/fake/nope".to_string()),
                system_path_str,
                "/tmp/fake/nope",
            )
            .build()
            .expect("failed to load config");

            assert_eq!(cfg.env().clone(), LitEnv::Dev);

            // Test verify
            let res = cfg.verify_litos_guest_user_cfg_file(init_path.as_path());

            assert!(res.is_err());

            let err = res.err().unwrap();
            if !err.to_string().contains(exp_err) {
                panic!("err does not contain '{exp_err}': {err:?}");
            }
        }
    }

    #[test]
    fn verify_litos_guest_instance_id_ok_test() {
        let system_path = get_test_path("guest_init/system");
        let system_path_str = system_path.to_str().unwrap();

        let init_path = get_test_path("guest_init/instance-id-ok");
        let init_path_str = init_path.to_str().unwrap();

        let cfg = LitConfigBuilder::new_with_paths(
            Some("keyed".to_string()),
            Some("/tmp/fake/nope".to_string()),
            system_path_str,
            init_path_str,
        )
        .build()
        .expect("failed to load config");

        assert_eq!(cfg.env().clone(), LitEnv::Dev);

        // Test verify
        cfg.verify_litos_guest_instance_id().expect("failed to verify guest instance id");
    }

    #[test]
    fn verify_litos_guest_instance_id_invalid_test() {
        let system_path = get_test_path("guest_init/system");
        let system_path_str = system_path.to_str().unwrap();

        for (test, exp_err) in [
            ("instance-id-missing", "configuration property \"guest.instance.id\" not found"),
            ("instance-id-len1", "invalid config: guest.instance.id (invalid length)"),
            ("instance-id-len2", "invalid config: guest.instance.id (invalid length)"),
        ] {
            println!("test: {test}");

            let init_path = get_test_path(format!("guest_init/{test}").as_str());
            let init_path_str = init_path.to_str().unwrap();

            // Note init path isn't given here.
            let cfg = LitConfigBuilder::new_with_paths(
                Some("keyed".to_string()),
                Some("/tmp/fake/nope".to_string()),
                system_path_str,
                init_path_str,
            )
            .build()
            .expect("failed to load config");

            assert_eq!(cfg.env().clone(), LitEnv::Dev);

            // Test verify
            let res = cfg.verify_litos_guest_instance_id();

            assert!(res.is_err());

            let err = res.err().unwrap();
            if !err.to_string().contains(exp_err) {
                panic!("err does not contain '{exp_err}': {err:?}");
            }
        }
    }

    // Util
    fn get_test_path(path: &str) -> PathBuf {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push(RESOURCES_TEST_DIR);
        test_path.push(path);
        test_path
    }
}
