use lit_os_core::guest::env::build::{read_guest_build_env, GuestBuildEnv};
use std::cmp::Reverse;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use lit_cli_core::config::LitCliConfig;
use lit_cli_core::utils::filter::matches_str_filter;
use term_table::row::Row;
use term_table::table_cell::TableCell;

use lit_core::config::LitConfig;
use lit_core::error::Result;
use lit_os_core::config::LitOsConfig;
use lit_os_core::error::parser_err;

use crate::config::LitCliOsConfig;
use crate::guest::common::print_describe_string;
use crate::guest::template::helper::GuestTemplateHelper;

pub(crate) mod helper;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[allow(unused)]
pub struct GuestTemplateItem {
    pub build_env: GuestBuildEnv,
    pub path: PathBuf,
}

pub fn walk_guest_templates<F>(cfg: &LitConfig, mut cb: F)
where
    F: FnMut(PathBuf, Result<Option<GuestBuildEnv>>) -> bool,
{
    let tmpl_dir = PathBuf::from(
        cfg.litos_var_guest_templates_dir().expect("failed to determine templates dir"),
    );

    if tmpl_dir.exists() {
        for rel_de in tmpl_dir.read_dir().unwrap() {
            for type_de in rel_de.unwrap().path().read_dir().unwrap() {
                for build_de in type_de.unwrap().path().read_dir().unwrap() {
                    let build_de = build_de.unwrap();
                    let build_path = build_de.path();

                    let res = read_guest_build_env(&build_path).map_err(|e| {
                        parser_err(
                            e,
                            Some(format!("Failed to read build env in: {:?}", &build_path)),
                        )
                    });

                    if cb(build_path, res) {
                        break;
                    }
                }
            }
        }
    }
}

pub(crate) fn get_guest_template_item(
    build_path: &Path, build_env: &GuestBuildEnv,
) -> Result<GuestTemplateItem> {
    Ok(GuestTemplateItem { build_env: build_env.to_owned(), path: build_path.to_path_buf() })
}

pub fn find_guest_templates(
    cfg: &LitConfig, type_filter: Option<&str>, release_filter: Option<&str>,
    id_filter: Option<&str>, kind_filter: Option<&str>,
) -> Option<Vec<GuestTemplateItem>> {
    let mut items: Vec<GuestTemplateItem> = Vec::new();
    walk_guest_templates(cfg, {
        let items = &mut items;
        move |build_path, res| {
            match res {
                Ok(Some(build_env)) => match get_guest_template_item(&build_path, &build_env) {
                    Ok(item) => {
                        if !matches_str_filter(id_filter, build_env.build_id.as_deref()) {
                            return false;
                        }
                        if !matches_str_filter(type_filter, build_env.build_type.as_deref()) {
                            return false;
                        }
                        if !matches_str_filter(release_filter, build_env.build_release.as_deref()) {
                            return false;
                        }
                        if !matches_str_filter(kind_filter, build_env.build_kind.as_deref()) {
                            return false;
                        }

                        items.push(item);
                    }
                    Err(e) => {
                        panic!("Failed to read build env: {e:?}");
                    }
                },
                Err(e) => {
                    panic!("Failed to walk template: {e:?}");
                }
                _ => {}
            }

            false
        }
    });

    if !items.is_empty() {
        // Sort
        items.sort_by_key(|it| Reverse(it.build_env.build_unix.clone().unwrap_or("".to_string())));

        Some(items)
    } else {
        None
    }
}

pub fn find_guest_dead_templates(cfg: &LitConfig) -> Option<Vec<PathBuf>> {
    let mut items: Vec<PathBuf> = Vec::new();
    walk_guest_templates(cfg, {
        let items = &mut items;
        move |build_path, res| {
            match res {
                Ok(Some(build_env)) => {
                    if !build_env.is_valid() {
                        items.push(build_path);
                    }
                }
                Ok(None) => items.push(build_path),
                Err(_) => items.push(build_path),
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

pub fn find_one_guest_template(
    cfg: &LitConfig, type_filter: Option<&str>, release_filter: Option<&str>,
    id_filter: Option<&str>, kind_filter: Option<&str>,
) -> Option<GuestTemplateItem> {
    let mut items: Option<Vec<GuestTemplateItem>> =
        find_guest_templates(cfg, type_filter, release_filter, id_filter, kind_filter);

    if items.is_some() {
        let mut items = items.take().unwrap();
        match items.len() {
            1 => {
                return Some(items.remove(0));
            }
            _ => {
                print_guest_templates(cfg, items);

                eprintln!();
                eprintln!("Too many matching guest templates found");
            }
        }
    } else {
        eprintln!("Guest template not found");
    }

    None
}

pub fn print_guest_templates(cfg: &LitConfig, items: Vec<GuestTemplateItem>) {
    let mut table = cfg.default_ascii_table();
    let null_val = "NULL".to_string();

    table.add_row(Row::new(vec![
        TableCell::new("id"),
        TableCell::new("date"),
        TableCell::new("type"),
        TableCell::new("kind"),
        TableCell::new("release"),
        TableCell::new("size"),
        TableCell::new("options"),
    ]));

    for (i, item) in items.iter().enumerate() {
        let build_env = &item.build_env;

        let mut row = Row::new(vec![
            TableCell::new(build_env.build_id.as_ref().unwrap_or(&null_val)),
            TableCell::new(build_env.build_date.as_ref().unwrap_or(&null_val)),
            TableCell::new(build_env.build_type.as_ref().unwrap_or(&null_val)),
            TableCell::new(build_env.build_kind.as_ref().unwrap_or(&null_val)),
            TableCell::new(build_env.build_release.as_ref().unwrap_or(&null_val)),
            TableCell::new(build_env.build_img_size.as_ref().unwrap_or(&null_val)),
            TableCell::new(build_env.build_options().join(", ")),
        ]);
        if i != 0 {
            row.has_separator = false;
        }
        table.add_row(row);
    }

    print!("{}", table.render());
}

pub fn print_guest_build_env(env: &GuestBuildEnv, short: bool) {
    print_describe_string("Build ID:", env.build_id.as_ref());
    print_describe_string("Build Date:", env.build_date.as_ref());
    print_describe_string("Type:", env.build_type.as_ref());
    print_describe_string("Kind:", env.build_kind.as_ref());
    print_describe_string("Release:", env.build_release.as_ref());
    print_describe_string("Options:", Some(&env.build_options().join(", ")));

    if !short {
        print_describe_string("Hostname:", env.build_hostname.as_ref());
        print_describe_string("Reference Image:", env.build_ref_img.as_ref());
        print_describe_string("Image Name:", env.build_img_name.as_ref());
        print_describe_string("Image Size:", env.build_img_size.as_ref());
        print_describe_string("Root Size:", env.build_root_size.as_ref());
        print_describe_string("Luks Root UUID:", env.build_luks_root_uuid.as_ref());
        print_describe_string("Luks Var UUID:", env.build_luks_var_uuid.as_ref());
        print_describe_string("Root UUID:", env.build_root_uuid.as_ref());
        print_describe_string("Root Hash:", env.build_root_hash.as_ref());
        print_describe_string("Var UUID:", env.build_var_uuid.as_ref());
        print_describe_string("Var Hash:", env.build_var_hash.as_ref());
    }
}

#[allow(dead_code)]
pub(crate) fn cleanup_guest_build(cfg: &LitConfig, path: &Path, quiet: bool) {
    let build_cmd = cfg
        .litos_guest_template_build_cmd()
        .expect("failed to determine the lit os install location");

    let mut cmd = Command::new(build_cmd);
    cmd.arg("-path").arg(path.to_str().unwrap()).arg("-cleanup");

    cmd.stderr(Stdio::inherit());

    if quiet {
        cmd.stdout(Stdio::null());
    } else {
        cmd.stdout(Stdio::inherit());
    }

    cmd.status().expect("failed to run instance build cleanup command");
}
