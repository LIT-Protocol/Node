use crate::config::{LitCliOsConfig, CFG_KEY_LITOS_HOST_RESERVED_MEM};
use crate::guest::instance::{active_guest_instance_stats, GuestInstanceItem};
use ethers::types::Address;
use human_bytes::human_bytes;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_cli_core::utils::system::parse_human_as_bytes;
use lit_core::config::LitConfig;
use lit_core::error::Result;
use lit_os_core::error::{parser_err, unexpected_err, validation_err};
use lit_os_core::guest::cloud_init::network_config::CloudInitNetworkConfig;
use lit_os_core::guest::cloud_init::CLOUD_INIT_FILE_NETWORK_CONFIG;
use lit_os_core::guest::env::instance::GuestInstanceEnv;
use lit_os_core::guest::types::GuestType;
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) trait GuestInstanceHelper {
    /// Verify whether an instance is valid (or considered dirty/invalid).
    fn is_valid(&self) -> bool;
    fn service_name(&self) -> Result<String>;
    fn service_exists(&self) -> Result<bool>;
    fn service_must_exist(&self) -> Result<()>;
    fn instance_mem_string(&self) -> Result<String>;
    fn instance_mem(&self) -> Result<u64>;
    fn is_enabled(&self) -> Result<bool>;
    /// Verify if it's safe to enable an instance with these metrics.
    fn can_enable(&self, cfg: &LitConfig) -> Result<()>;
    fn enable(&self, cfg: &LitConfig) -> Result<()>;
    fn disable(&self) -> Result<()>;
    fn start(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;
    fn restart(&self) -> Result<()>;
}

impl GuestInstanceHelper for GuestInstanceEnv {
    /// An instance is valid if it has an instance.env (which is implied here). Beyond that
    /// we perform some other checks to attempt to detect failed creations.
    fn is_valid(&self) -> bool {
        if let Ok(exists) = self.service_exists() {
            if exists {
                return true;
            }
        }

        false
    }

    fn service_name(&self) -> Result<String> {
        self.instance_service
            .as_ref()
            .ok_or_else(|| {
                unexpected_err(
                    format!(
                        "missing INSTANCE_SERVICE key in instance.env (in: {:?})",
                        &self.instance_id
                    ),
                    None,
                )
            })
            .map(|v| v.to_string())
    }

    fn service_exists(&self) -> Result<bool> {
        let instance_service = self.service_name()?;
        systemctl::exists(instance_service.as_str()).map_err(|e| {
            unexpected_err(e, Some(format!("failed to run 'systemctl status {instance_service}'")))
        })
    }

    fn service_must_exist(&self) -> Result<()> {
        if self.service_exists()? {
            Ok(())
        } else {
            Err(validation_err(
                format!("systemctl service '{}' does not exist", self.service_name()?),
                None,
            ))
        }
    }

    fn instance_mem_string(&self) -> Result<String> {
        self.instance_mem
            .as_ref()
            .ok_or_else(|| {
                unexpected_err(
                    format!(
                        "missing INSTANCE_MEM key in instance.env (in: {:?})",
                        &self.instance_id
                    ),
                    None,
                )
            })
            .map(|v| v.to_string())
    }

    fn instance_mem(&self) -> Result<u64> {
        let instance_mem = self.instance_mem_string()?;
        parse_human_as_bytes(&instance_mem).map_err(|e| {
            parser_err(
                e,
                Some(format!(
                    "failed to parse bytes from INSTANCE_MEM ('{}', in: {:?})",
                    instance_mem, &self.instance_id
                )),
            )
        })
    }

    fn is_enabled(&self) -> Result<bool> {
        let instance_service = self.service_name()?;
        if self.service_exists()? {
            return systemctl::is_enabled(instance_service.as_str()).map_err(|e| {
                unexpected_err(
                    e,
                    Some(format!("failed to run 'systemctl is-enabled {instance_service}'")),
                )
            });
        }

        Ok(false)
    }

    fn can_enable(&self, cfg: &LitConfig) -> Result<()> {
        self.service_must_exist()?;

        can_enable_instance(cfg, self.instance_mem_string()?)
    }

    fn enable(&self, cfg: &LitConfig) -> Result<()> {
        self.service_must_exist()?;

        if !self.is_enabled()? {
            self.can_enable(cfg)?;
            let instance_service = self.service_name()?;
            let _ = systemctl::enable(instance_service.as_str()).map_err(|e| {
                unexpected_err(
                    e,
                    Some(format!("failed to run systemctl enable on {instance_service}")),
                )
            })?;
        }

        Ok(())
    }

    fn disable(&self) -> Result<()> {
        if self.service_exists()? {
            let instance_service = self.service_name()?;
            let _ = systemctl::disable(instance_service.as_str()).map_err(|e| {
                unexpected_err(
                    e,
                    Some(format!("failed to run systemctl disable on {instance_service}")),
                )
            })?;
        }

        Ok(())
    }

    fn start(&self) -> Result<()> {
        self.service_must_exist()?;

        let instance_service = self.service_name()?;
        let _ = systemctl::start(instance_service.as_str()).map_err(|e| {
            unexpected_err(e, Some(format!("failed to run systemctl start on {instance_service}")))
        })?;

        Ok(())
    }

    fn stop(&self) -> Result<()> {
        self.service_must_exist()?;

        let instance_service = self.service_name()?;
        let _ = systemctl::stop(instance_service.as_str()).map_err(|e| {
            unexpected_err(e, Some(format!("failed to run systemctl stop on {instance_service}")))
        })?;

        Ok(())
    }

    fn restart(&self) -> Result<()> {
        self.service_must_exist()?;
        let instance_service = self.service_name()?;
        let _ = systemctl::restart(instance_service.as_str()).map_err(|e| {
            unexpected_err(
                e,
                Some(format!("failed to run systemctl restart on {instance_service}")),
            )
        })?;

        Ok(())
    }
}

impl GuestInstanceHelper for GuestInstanceItem {
    fn is_valid(&self) -> bool {
        self.instance_env.is_valid()
    }

    fn service_name(&self) -> Result<String> {
        self.instance_env.service_name()
    }

    fn service_exists(&self) -> Result<bool> {
        self.instance_env.service_exists()
    }

    fn service_must_exist(&self) -> Result<()> {
        self.instance_env.service_must_exist()
    }

    fn instance_mem_string(&self) -> Result<String> {
        self.instance_env.instance_mem_string()
    }

    fn instance_mem(&self) -> Result<u64> {
        self.instance_env.instance_mem()
    }

    fn is_enabled(&self) -> Result<bool> {
        self.instance_env.is_enabled()
    }

    fn can_enable(&self, cfg: &LitConfig) -> Result<()> {
        self.instance_env.can_enable(cfg)
    }

    fn enable(&self, cfg: &LitConfig) -> Result<()> {
        self.instance_env.enable(cfg)
    }

    fn disable(&self) -> Result<()> {
        self.instance_env.disable()
    }

    fn start(&self) -> Result<()> {
        self.instance_env.start()
    }

    fn stop(&self) -> Result<()> {
        self.instance_env.stop()
    }
    fn restart(&self) -> Result<()> {
        self.instance_env.restart()
    }
}

pub trait GuestInstanceItemHelper {
    fn cloud_init_path(&self) -> PathBuf;
    fn network_cfg(&self) -> Result<Option<CloudInitNetworkConfig>>;
    fn staker_address(&self) -> Result<Address>;
    async fn is_active_on_network(&self, cfg: &LitConfig) -> Result<bool>;
}

impl GuestInstanceItemHelper for GuestInstanceItem {
    fn cloud_init_path(&self) -> PathBuf {
        let mut path = self.path.clone();
        path.push("cloud-init");
        path
    }
    fn network_cfg(&self) -> Result<Option<CloudInitNetworkConfig>> {
        let mut path = self.cloud_init_path();
        path.push(CLOUD_INIT_FILE_NETWORK_CONFIG);

        if !path.exists() {
            return Ok(None);
        }

        Ok(Some(CloudInitNetworkConfig::try_from(path.as_path())?))
    }
    fn staker_address(&self) -> Result<Address> {
        let instance_config = self
            .load_config()
            .map_err(|e| validation_err("Failed to load instance config", Some(e.to_string())))?;

        let staker_address_string = instance_config
            .get("node", "staker_address")
            .ok_or_else(|| validation_err("Staker address not found in config", None))?;

        // Parse hex string to Address, staker_address is of the form "0x0123456..."
        Address::from_str(&staker_address_string)
            .map_err(|e| validation_err("Invalid staker address", Some(e.to_string())))
    }
    async fn is_active_on_network(&self, cfg: &LitConfig) -> Result<bool> {
        // Note (Harry): Only check contracts if it is a lit node
        let guest_type = self.build_env.guest_type().expect("build.env missing GuestType");
        if guest_type != GuestType::Node {
            return Err(validation_err(
                format!("Can't verify activity for instances of type {:?}", guest_type),
                None,
            ));
        }

        let resolver = ContractResolver::new(
            self.instance_env.subnet_id.clone().expect("node-type nodes must have a subnet_id"),
            self.build_env.env().expect("node-type nodes must have an env"),
            None,
        );

        let staking_contract =
            resolver.staking_contract(&cfg).await.expect("Failed to get staking contract");

        let current_epoch_validators = staking_contract
            .get_validators_in_current_epoch()
            .await
            .expect("Failed to get validators in current epoch");

        let next_epoch_validators = staking_contract
            .get_validators_in_next_epoch()
            .await
            .expect("Failed to get validators in next epoch");

        let mut validators = Vec::new();
        validators.extend(current_epoch_validators);
        validators.extend(next_epoch_validators);
        validators.sort();
        validators.dedup();

        let staker_address = self.staker_address()?;

        // Remove any validator that isn't the current, if the array isn't empty then the node hasn't requested to leave and must panic
        if !validators.into_iter().filter(|v| v == &staker_address).collect::<Vec<_>>().is_empty() {
            return Ok(true);
        }
        return Ok(false);
    }
}

/// Verify if it's safe to enable an instance with these metrics.
pub fn can_enable_instance<S: AsRef<str>>(cfg: &LitConfig, instance_mem: S) -> Result<()> {
    let meminfo = sys_info::mem_info().map_err(|e| unexpected_err(e, None))?;

    let stats = active_guest_instance_stats(cfg)?;

    let add_bytes = parse_human_as_bytes(instance_mem.as_ref()).map_err(|e| {
        parser_err(
            e,
            Some(format!("failed to parse bytes from INSTANCE_MEM ('{}')", instance_mem.as_ref())),
        )
    })? as f64;

    let reserved_mem = cfg.litos_host_reserved_mem()?;
    let reserved_bytes = parse_human_as_bytes(&reserved_mem).map_err(|e| {
        parser_err(
            e,
            Some(format!(
                "failed to parse bytes from config '{}' ('{}')",
                CFG_KEY_LITOS_HOST_RESERVED_MEM, &reserved_mem
            )),
        )
    })?;

    let cur_bytes = stats.memory + reserved_bytes;
    let total = meminfo.total * 1024;
    let avail = if total >= cur_bytes { total - cur_bytes } else { 0 } as f64;

    if avail < add_bytes {
        return Err(validation_err(
            format!(
                "instance memory ({}) exceeds available memory (avail: {}, total: {}, used: {}, reserved: {})",
                human_bytes(add_bytes),
                human_bytes(avail),
                human_bytes(total as f64),
                human_bytes(stats.memory as f64),
                human_bytes(reserved_bytes as f64)
            ),
            None,
        ));
    }

    Ok(())
}
