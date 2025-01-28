use std::cmp::Reverse;
use std::path::{Path, PathBuf};

use lit_cli_core::cmd::OutputType;
use term_table::row::Row;
use term_table::table_cell::TableCell;

use lit_cli_core::config::LitCliConfig;
use lit_cli_core::utils::dhcpd::get_dhcpd_leases;
use lit_cli_core::utils::filter::matches_str_filter;
use lit_core::config::LitConfig;
use lit_core::error::{Error, Result};
use lit_core::utils::toml::SimpleToml;
use lit_os_core::config::LitOsConfig;
use lit_os_core::error::{parser_err, unexpected_err};
use lit_os_core::guest::cloud_init::network_config::NET_IF_EXTERNAL;
use lit_os_core::guest::env::build::{read_guest_build_env, GuestBuildEnv};
use lit_os_core::guest::env::instance::{read_guest_instance_env, GuestInstanceEnv};
use lit_os_core::guest::env::release::{
    find_instance_release_path, read_guest_release_env, GuestReleaseEnv,
};

use crate::guest::common::print_describe_string;
use crate::guest::instance::helper::{GuestInstanceHelper, GuestInstanceItemHelper};

pub(crate) mod common;
pub mod helper;
pub(crate) mod oneshot;
pub(crate) mod release;

pub struct GuestInstanceItem {
    pub instance_env: GuestInstanceEnv,
    pub build_env: GuestBuildEnv,
    pub release_env: Option<GuestReleaseEnv>,
    pub path: PathBuf,
}

impl GuestInstanceItem {
    pub(crate) fn load_config(&self) -> Result<SimpleToml> {
        let mut cfg_path = self.path.clone();
        cfg_path.push("cloud-init/config.toml");

        SimpleToml::try_from(cfg_path.as_path())
    }
}

#[derive(Default)]
pub struct GuestInstancesStats {
    pub memory: u64,
}

pub fn walk_guest_instances<F>(cfg: &LitConfig, mut cb: F)
where
    F: FnMut(PathBuf, Result<Option<GuestInstanceEnv>>) -> bool,
{
    let instance_dir = PathBuf::from(
        cfg.litos_var_guest_instances_dir().expect("failed to determine instances dir"),
    );

    if instance_dir.exists() {
        for rel_de in instance_dir.read_dir().unwrap() {
            for type_de in rel_de.unwrap().path().read_dir().unwrap() {
                for instance_path in type_de.unwrap().path().read_dir().unwrap() {
                    let instance_path = instance_path.unwrap().path();
                    let res = read_guest_instance_env(&instance_path).map_err(|e| {
                        parser_err(
                            e,
                            Some(format!("Failed to read instance env in: {:?}", instance_path)),
                        )
                    });

                    if cb(instance_path, res) {
                        break;
                    }
                }
            }
        }
    }
}

/// walk only the valid instances
pub fn walk_guest_instance_items<F>(cfg: &LitConfig, mut cb: F)
where
    F: FnMut(GuestInstanceItem) -> bool,
{
    walk_guest_instances(cfg, {
        move |instance_path, res| {
            match res {
                Ok(Some(instance_env)) => {
                    if instance_env.is_valid() {
                        match get_guest_instance_item(&instance_path, &instance_env) {
                            Ok(item) => {
                                if cb(item) {
                                    return true;
                                }
                            }
                            Err(e) => {
                                panic!("Failed to read instance build/release envs: {e:?}");
                            }
                        }
                    }
                }
                Ok(None) => {}
                Err(_) => {}
            }

            false
        }
    });
}

pub(crate) fn get_guest_instance_item(
    instance_path: &Path, instance_env: &GuestInstanceEnv,
) -> Result<GuestInstanceItem> {
    let mut instance_build_pb = instance_path.to_path_buf();
    instance_build_pb.push("build");

    match read_guest_build_env(&instance_build_pb) {
        Ok(Some(build_env)) => {
            let mut item = GuestInstanceItem {
                instance_env: instance_env.clone(),
                build_env,
                release_env: None,
                path: instance_path.to_path_buf(),
            };

            if let Some(release_path) = find_instance_release_path(instance_path, instance_env) {
                match read_guest_release_env(release_path.as_path()) {
                    Ok(Some(release_env)) => {
                        item.release_env = Some(release_env);
                    }
                    Err(e) => {
                        return Err(unexpected_err(
                            e,
                            Some(format!("failed to read release.env in: {:?}", &release_path)),
                        ))
                    }
                    _ => {}
                }
            }

            Ok(item)
        }
        Ok(None) => {
            Err(unexpected_err(format!("build.env not found in: {:?}", &instance_build_pb), None))
        }
        Err(e) => Err(unexpected_err(
            e,
            Some(format!("failed to read build.env in: {:?}", &instance_build_pb)),
        )),
    }
}

pub fn find_guest_instances(
    cfg: &LitConfig, type_filter: Option<&str>, release_filter: Option<&str>,
    id_filter: Option<&str>,
) -> Option<Vec<GuestInstanceItem>> {
    let mut items: Vec<GuestInstanceItem> = Vec::new();
    walk_guest_instances(cfg, {
        let items = &mut items;
        move |instance_path, res| {
            match res {
                Ok(Some(instance_env)) => {
                    match get_guest_instance_item(&instance_path, &instance_env) {
                        Ok(item) => {
                            if !matches_str_filter(id_filter, instance_env.instance_id.as_deref()) {
                                return false;
                            }
                            if !matches_str_filter(
                                type_filter,
                                item.build_env.build_type.as_deref(),
                            ) {
                                return false;
                            }
                            if !matches_str_filter(
                                release_filter,
                                item.build_env.build_release.as_deref(),
                            ) {
                                return false;
                            }

                            items.push(item);
                        }
                        Err(e) => {
                            panic!("Failed to read instance build/release envs: {e:?}");
                        }
                    }
                }
                Err(e) => {
                    panic!("Failed to walk instance: {e:?}");
                }
                _ => {}
            }

            false
        }
    });

    if !items.is_empty() {
        // Sort
        items.sort_by_key(|it| {
            Reverse(it.instance_env.instance_unix.clone().unwrap_or("".to_string()))
        });

        Some(items)
    } else {
        None
    }
}

pub fn find_guest_dead_instances(cfg: &LitConfig) -> Option<Vec<PathBuf>> {
    let mut items: Vec<PathBuf> = Vec::new();
    walk_guest_instances(cfg, {
        let items = &mut items;
        move |instance_path, res| {
            match res {
                Ok(Some(instance_env)) => {
                    if !instance_env.is_valid() {
                        items.push(instance_path);
                    }
                }
                Ok(None) => items.push(instance_path),
                Err(_) => items.push(instance_path),
            }

            false
        }
    });

    if !items.is_empty() {
        Some(items)
    } else {
        None
    }
}

pub fn find_one_guest_instance(
    cfg: &LitConfig, type_filter: Option<&str>, release_filter: Option<&str>,
    id_filter: Option<&str>,
) -> Option<GuestInstanceItem> {
    let mut items: Option<Vec<GuestInstanceItem>> =
        find_guest_instances(cfg, type_filter, release_filter, id_filter);

    if items.is_some() {
        let mut items = items.take().unwrap();
        match items.len() {
            1 => {
                return Some(items.remove(0));
            }
            _ => {
                print_guest_instances(cfg, items, None);

                eprintln!();
                eprintln!("Too many matching guest instances found");
            }
        }
    } else {
        eprintln!(
            "Guest instance not found (type_filter: {:?}), release_filter: {:?}, id_filter: {:?})",
            type_filter, release_filter, id_filter
        );
    }

    None
}

pub fn find_latest_guest_instance(
    cfg: &LitConfig, type_filter: Option<&str>, release_filter: Option<&str>,
) -> Option<GuestInstanceItem> {
    let mut items: Option<Vec<GuestInstanceItem>> =
        find_guest_instances(cfg, type_filter, release_filter, None);

    if items.is_some() {
        let mut items = items.take().unwrap();
        return Some(items.remove(0));
    } else {
        eprintln!(
            "Guest instance not found (type_filter: {:?}), release_filter: {:?})",
            type_filter, release_filter
        );
    }

    None
}

pub fn active_guest_instance_stats(cfg: &LitConfig) -> Result<GuestInstancesStats> {
    let mut stats: GuestInstancesStats = GuestInstancesStats::default();
    let mut err: Option<Error> = None;
    walk_guest_instances(cfg, {
        let stats = &mut stats;
        let err = &mut err;
        move |_instance_path, res| {
            match res {
                Ok(Some(instance_env)) => {
                    if let Err(e) = push_instance_stats(cfg, stats, &instance_env) {
                        let _ = err.insert(e);
                        return true;
                    }
                }
                Err(e) => {
                    let _ = err.insert(e);
                    return true;
                }
                _ => {}
            }

            false
        }
    });

    if let Some(e) = err.take() {
        return Err(e);
    }

    Ok(stats)
}

pub fn push_instance_stats(
    _cfg: &LitConfig, stats: &mut GuestInstancesStats, instance_env: &GuestInstanceEnv,
) -> Result<()> {
    if instance_env.is_enabled()? {
        stats.memory += instance_env.instance_mem()?;
    }

    Ok(())
}

pub fn print_guest_instances(
    cfg: &LitConfig, items: Vec<GuestInstanceItem>, output: Option<OutputType>,
) {
    let mut table = cfg.default_ascii_table();
    let null_val = "NULL".to_string();
    let is_wide = matches!(output, Some(OutputType::Wide));

    if is_wide {
        table.add_row(Row::new(vec![
            TableCell::new("id"),
            TableCell::new("name"),
            TableCell::new("labels"),
            TableCell::new("date"),
            TableCell::new("type"),
            TableCell::new("kind"),
            TableCell::new("env"),
            TableCell::new("release"),
            TableCell::new("subnet"),
        ]));
    } else {
        table.add_row(Row::new(vec![
            TableCell::new("id"),
            TableCell::new("name"),
            TableCell::new("date"),
            TableCell::new("type"),
            TableCell::new("env"),
            TableCell::new("release"),
        ]));
    }

    for (i, item) in items.iter().enumerate() {
        let instance_env = &item.instance_env;
        let build_env = &item.build_env;
        let release = if let Some(release_env) = item.release_env.as_ref() {
            release_env.release_id.as_ref().unwrap_or(&null_val).clone()
        } else {
            format!("LOCAL:{}", build_env.build_id.as_ref().unwrap_or(&null_val))
        };

        let mut row = if is_wide {
            Row::new(vec![
                TableCell::new(instance_env.instance_id.as_ref().unwrap_or(&null_val)),
                TableCell::new(instance_env.instance_name.as_ref().unwrap_or(&null_val)),
                TableCell::new(
                    instance_env
                        .instance_labels
                        .as_ref()
                        .map(|v| v.replace(' ', "\n"))
                        .unwrap_or(null_val.clone()),
                ),
                TableCell::new(instance_env.instance_date.as_ref().unwrap_or(&null_val)),
                TableCell::new(build_env.build_type.as_ref().unwrap_or(&null_val)),
                TableCell::new(build_env.build_kind.as_ref().unwrap_or(&null_val)),
                TableCell::new(build_env.build_release.as_ref().unwrap_or(&null_val)),
                TableCell::new(release),
                TableCell::new(instance_env.subnet_id.as_ref().unwrap_or(&null_val)),
            ])
        } else {
            Row::new(vec![
                TableCell::new(instance_env.instance_id.as_ref().unwrap_or(&null_val)),
                TableCell::new(instance_env.instance_name.as_ref().unwrap_or(&null_val)),
                TableCell::new(instance_env.instance_date.as_ref().unwrap_or(&null_val)),
                TableCell::new(build_env.build_type.as_ref().unwrap_or(&null_val)),
                TableCell::new(build_env.build_release.as_ref().unwrap_or(&null_val)),
                TableCell::new(release),
            ])
        };

        if i != 0 {
            row.has_separator = false;
        }
        table.add_row(row);
    }

    print!("{}", table.render());
}

pub fn print_guest_instance_processes(
    cfg: &LitConfig, items: Vec<GuestInstanceItem>, output: Option<OutputType>,
) {
    let mut table = cfg.default_ascii_table();
    let null_val = "NULL".to_string();
    let is_wide = matches!(output, Some(OutputType::Wide));

    let dhcp_leases = get_dhcpd_leases().expect("failed to get dhcpd leases");

    if is_wide {
        table.add_row(Row::new(vec![
            TableCell::new("pid"),
            TableCell::new("id"),
            TableCell::new("name"),
            TableCell::new("labels"),
            TableCell::new("ip int."),
            TableCell::new("ip ext."),
            TableCell::new("gw ext."),
            TableCell::new("vcpus"),
            TableCell::new("memory"),
            TableCell::new("subnet"),
        ]));
    } else {
        table.add_row(Row::new(vec![
            TableCell::new("pid"),
            TableCell::new("id"),
            TableCell::new("name"),
            TableCell::new("ip"),
            TableCell::new("vcpus"),
            TableCell::new("memory"),
        ]));
    }

    for (i, item) in items.iter().enumerate() {
        let instance_env = &item.instance_env;
        if !instance_env.is_valid() {
            continue;
        }

        if let Some(instance_service) = &instance_env.instance_service {
            let unit = systemctl::Unit::from_systemctl(instance_service.as_str())
                .unwrap_or_else(|_| panic!("failed to get systemctl unit for: {instance_service}"));

            if !unit.active {
                continue;
            }

            let mut int_ip: Option<String> = None;
            if let Some(int_mac) = instance_env.instance_net_int_mac.as_ref() {
                if let Some(lease) = dhcp_leases.get(int_mac) {
                    int_ip = Some(lease.ip.clone());
                }
            }

            let mut ext_ip: Option<String> = None;
            let mut ext_gw: Option<String> = None;
            if is_wide {
                if let Ok(Some(cfg)) = item.network_cfg() {
                    if let Some(iface) = cfg.ethernets().get(NET_IF_EXTERNAL) {
                        if let Some(addrs) = iface.addresses() {
                            ext_ip = Some(addrs.join(", "));
                        }
                        if let Some(gw) = iface.gateway4().or_else(|| iface.gateway6()) {
                            ext_gw = Some(gw.clone());
                        }
                    }
                }
            }

            let mut row = if is_wide {
                Row::new(vec![
                    TableCell::new(unit.pid.unwrap().to_string()),
                    TableCell::new(instance_env.instance_id.as_ref().unwrap_or(&null_val)),
                    TableCell::new(instance_env.instance_name.as_ref().unwrap_or(&null_val)),
                    TableCell::new(
                        instance_env
                            .instance_labels
                            .as_ref()
                            .map(|v| v.replace(' ', "\n"))
                            .unwrap_or(null_val.clone()),
                    ),
                    TableCell::new(int_ip.as_ref().unwrap_or(&null_val)),
                    TableCell::new(ext_ip.as_ref().unwrap_or(&null_val)),
                    TableCell::new(ext_gw.as_ref().unwrap_or(&null_val)),
                    TableCell::new(instance_env.instance_vcpus.as_ref().unwrap_or(&null_val)),
                    TableCell::new(format!(
                        "{} / {}",
                        instance_env.instance_mem.as_ref().unwrap_or(&null_val),
                        unit.memory.unwrap()
                    )),
                    TableCell::new(instance_env.subnet_id.as_ref().unwrap_or(&null_val)),
                ])
            } else {
                Row::new(vec![
                    TableCell::new(unit.pid.unwrap().to_string()),
                    TableCell::new(instance_env.instance_id.as_ref().unwrap_or(&null_val)),
                    TableCell::new(instance_env.instance_name.as_ref().unwrap_or(&null_val)),
                    TableCell::new(int_ip.as_ref().unwrap_or(&null_val)),
                    TableCell::new(instance_env.instance_vcpus.as_ref().unwrap_or(&null_val)),
                    TableCell::new(format!(
                        "{} / {}",
                        instance_env.instance_mem.as_ref().unwrap_or(&null_val),
                        unit.memory.unwrap()
                    )),
                ])
            };
            if i != 0 {
                row.has_separator = false;
            }
            table.add_row(row);
        }
    }

    if !table.rows.is_empty() {
        print!("{}", table.render());
    } else {
        eprintln!("No running guest instances found");
    }
}

pub fn print_guest_release_env(env: &GuestReleaseEnv) {
    print_describe_string("Release ID:", env.release_id.as_ref());
    print_describe_string("Release Subnet:", env.release_subnet_id.as_ref());
    print_describe_string("Release Unix:", env.release_unix.as_ref());
}

pub fn print_guest_instance_env(env: &GuestInstanceEnv) {
    print_describe_string("Instance ID:", env.instance_id.as_ref());
    print_describe_string("Instance Name:", env.instance_name.as_ref());
    print_describe_string("Instance Labels:", env.instance_labels.as_ref());
    print_describe_string("Instance Service:", env.instance_service.as_ref());
    print_describe_string("Instance vCPUs:", env.instance_vcpus.as_ref());
    print_describe_string("Instance Memory:", env.instance_mem.as_ref());
    print_describe_string("Instance Size:", env.instance_img_size.as_ref());
    print_describe_string("Created Date:", env.instance_date.as_ref());
}
