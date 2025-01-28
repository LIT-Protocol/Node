pub mod error;
pub mod events;
pub mod host_commands_listener;

use lit_cli_os::guest::instance::{find_guest_instances, GuestInstanceItem};
use lit_core::config::LitConfig;
use tracing::{debug, error, info, trace};

pub fn get_instance_item(cfg: &LitConfig) -> GuestInstanceItem {
    match find_guest_instances(cfg, Some("node"), None, None) {
        Some(mut instances) => match instances.len() {
            1 => instances.pop().expect("we ensured there's exactly 1 item to pop"),
            _ => {
                error!(
                    "Found more than one guest instance: {:?}",
                    instances
                        .iter()
                        .filter_map(|item| item.instance_env.instance_id.clone())
                        .collect::<Vec<_>>()
                );
                std::process::exit(1);
            }
        },
        None => {
            trace!("No instance deployed (yet?), exiting");
            std::process::exit(0);
        }
    }
}
