use lit_cli_core::config::LitCliConfig;

use lit_core::config::envs::LitEnv;
use lit_core::config::{LitConfig, LitConfigBuilder};
use lit_os_core::config::LitOsConfig;
use lit_os_core::error::{validation_err, Result};

pub const CFG_KEY_LITOS_HOST_RESERVED_MEM: &str = "litos.host.reserved_mem";
pub const CFG_KEY_LITOS_HOST_ENV_ALLOW_INSECURE: &str = "litos.host.env.allow_insecure";
pub const CFG_KEY_LITOS_GUEST_DEFAULT_VCPUS: &str = "litos.guest.defaults.vcpus";
pub const CFG_KEY_LITOS_GUEST_DEFAULT_MEM: &str = "litos.guest.defaults.mem";
pub const CFG_KEY_LITOS_GUEST_DEFAULT_IMG_SIZE: &str = "litos.guest.defaults.img_size";
pub const CFG_KEY_LITOS_GUEST_DEFAULT_ZEROSSL_API_KEY: &str =
    "litos.guest.defaults.zerossl.api_key";
pub const CFG_KEY_LITOS_GUEST_DEFAULT_PROV_API_DOMAIN: &str =
    "litos.guest.defaults.prov.api.domain";

pub trait LitCliOsConfig {
    fn try_new() -> Result<LitConfig>;
    fn must_new() -> LitConfig;
    fn from_builder(builder: LitConfigBuilder) -> Result<LitConfig>;
    fn verify_env_available(&self, env: &LitEnv) -> Result<()>;
    fn litos_host_check_cmd(&self) -> Result<String>;
    fn litos_host_update_cmd(&self) -> Result<String>;
    fn litos_host_reserved_mem(&self) -> Result<String>;
    fn litos_host_env_allow_insecure(&self) -> Result<bool>;
    fn litos_guest_template_build_cmd(&self) -> Result<String>;
    fn litos_guest_instance_create_cmd(&self) -> Result<String>;
    fn litos_guest_instance_destroy_cmd(&self) -> Result<String>;
    fn litos_guest_instance_repair_cmd(&self) -> Result<String>;
    fn litos_guest_instance_resize_cmd(&self) -> Result<String>;
    fn litos_guest_default_vcpus(&self) -> Result<i64>;
    fn litos_guest_default_mem(&self) -> Result<String>;
    fn litos_guest_default_img_size(&self) -> Result<String>;
    fn litos_guest_default_prov_api_domain(&self) -> Result<String>;
    fn litos_guest_default_zerossl_api_key(&self) -> Result<String>;
}

impl LitCliOsConfig for LitConfig {
    fn try_new() -> Result<LitConfig> {
        <LitConfig as LitCliOsConfig>::from_builder(LitConfigBuilder::default())
    }

    fn must_new() -> Self {
        <LitConfig as LitCliOsConfig>::try_new().expect("failed to load config")
    }

    fn from_builder(mut builder: LitConfigBuilder) -> Result<Self> {
        // Set defaults
        builder = <LitConfig as LitOsConfig>::apply_defaults(builder)?
            .set_default(CFG_KEY_LITOS_HOST_RESERVED_MEM, "20G")
            .set_default(CFG_KEY_LITOS_HOST_ENV_ALLOW_INSECURE, false);

        let cfg = builder.build_cloned()?;
        match cfg.env() {
            &LitEnv::Dev => {
                builder = builder
                    .set_default(CFG_KEY_LITOS_GUEST_DEFAULT_VCPUS, 4)
                    .set_default(CFG_KEY_LITOS_GUEST_DEFAULT_MEM, "4G")
                    .set_default(CFG_KEY_LITOS_GUEST_DEFAULT_IMG_SIZE, "100G");
            }
            &LitEnv::Staging | &LitEnv::Prod => {
                builder = builder
                    .set_default(CFG_KEY_LITOS_GUEST_DEFAULT_VCPUS, 64)
                    .set_default(CFG_KEY_LITOS_GUEST_DEFAULT_MEM, "40G")
                    .set_default(CFG_KEY_LITOS_GUEST_DEFAULT_IMG_SIZE, "200G");
            }
        }

        <LitConfig as LitCliConfig>::from_builder(builder)
    }

    fn verify_env_available(&self, env: &LitEnv) -> Result<()> {
        if !self.litos_host_env_allow_insecure()?
            && !env.eq(self.env())
            && (self.is_prod() || env.eq(&LitEnv::Prod))
        {
            return Err(validation_err(
                format!(
                    "lit os host env: {} is not compatible with provided env: {}",
                    self.env(),
                    env
                ),
                None,
            ));
        }

        Ok(())
    }

    // Install (bins)
    fn litos_host_check_cmd(&self) -> Result<String> {
        Ok(format!("{}/host/check.sh", self.litos_install_dir()?))
    }

    fn litos_host_update_cmd(&self) -> Result<String> {
        Ok(format!("{}/host/update.sh", self.litos_install_dir()?))
    }

    fn litos_host_reserved_mem(&self) -> Result<String> {
        self.get_string(CFG_KEY_LITOS_HOST_RESERVED_MEM)
    }

    fn litos_host_env_allow_insecure(&self) -> Result<bool> {
        self.get_bool(CFG_KEY_LITOS_HOST_ENV_ALLOW_INSECURE)
    }

    fn litos_guest_template_build_cmd(&self) -> Result<String> {
        Ok(format!("{}/guest/build/build.sh", self.litos_install_dir()?))
    }

    fn litos_guest_instance_create_cmd(&self) -> Result<String> {
        Ok(format!("{}/guest/instance/create.sh", self.litos_install_dir()?))
    }

    fn litos_guest_instance_destroy_cmd(&self) -> Result<String> {
        Ok(format!("{}/guest/instance/destroy.sh", self.litos_install_dir()?))
    }

    fn litos_guest_instance_repair_cmd(&self) -> Result<String> {
        Ok(format!("{}/guest/instance/repair.sh", self.litos_install_dir()?))
    }

    fn litos_guest_instance_resize_cmd(&self) -> Result<String> {
        Ok(format!("{}/guest/instance/resize.sh", self.litos_install_dir()?))
    }

    // Guest
    // Defaults
    fn litos_guest_default_vcpus(&self) -> Result<i64> {
        self.get_int(CFG_KEY_LITOS_GUEST_DEFAULT_VCPUS)
    }

    fn litos_guest_default_mem(&self) -> Result<String> {
        self.get_string(CFG_KEY_LITOS_GUEST_DEFAULT_MEM)
    }

    fn litos_guest_default_img_size(&self) -> Result<String> {
        self.get_string(CFG_KEY_LITOS_GUEST_DEFAULT_IMG_SIZE)
    }

    fn litos_guest_default_prov_api_domain(&self) -> Result<String> {
        self.get_string(CFG_KEY_LITOS_GUEST_DEFAULT_PROV_API_DOMAIN)
    }

    fn litos_guest_default_zerossl_api_key(&self) -> Result<String> {
        self.get_string(CFG_KEY_LITOS_GUEST_DEFAULT_ZEROSSL_API_KEY)
    }
}
