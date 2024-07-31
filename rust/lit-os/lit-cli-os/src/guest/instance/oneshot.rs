use crate::cmd::os::guest::instance::create::GuestInstanceCreateArgsProv;
#[cfg(feature = "guest-build")]
use crate::cmd::os::guest::template::release::{do_release_template, GuestTemplateRelease};
#[cfg(feature = "guest-build")]
use crate::guest::instance::common::write_password;
use crate::guest::template::GuestTemplateItem;
#[cfg(feature = "guest-build")]
use lit_attestation::attestation::TryGenerate;
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;
#[allow(unused_imports)]
use lit_os_core::guest::oneshot::config::{
    ActionEntry, OneShotConfig, ACTION_SETTINGS_KEY_GUEST_VCPUS, ACTION_SETTINGS_KEY_REQUEST,
    ACTION_TYPE_BOOTSTRAP,
};
use lit_os_core::guest::types::GuestType;
#[allow(unused_imports)]
use lit_os_prov_core::release::create::types::CreateReleaseRequest;
use std::path::Path;
#[allow(unused_imports)]
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(unused_variables, clippy::too_many_arguments)]
pub(crate) async fn create_oneshot_actions(
    cfg: &LitConfig, opts: &CliGlobalOpts, subnet_id: &str, instance_type: GuestType,
    template: Option<&GuestTemplateItem>, instance_ci_dir: &Path,
    prov_args: Option<&GuestInstanceCreateArgsProv>, guest_vcpus: u16, admin_key: Option<String>,
) -> OneShotConfig {
    #[allow(unused_mut)]
    let mut oneshot_cfg = OneShotConfig::new();

    #[allow(clippy::single_match)]
    match instance_type {
        GuestType::Prov => {
            #[allow(unused_variables)]
            let prov_args = prov_args.unwrap();

            #[cfg(feature = "guest-build")]
            if prov_args.bootstrap {
                let template = template.expect("prov can only be bootstrapped from a template");

                let args = GuestTemplateRelease {
                    id: template.build_env.build_id.clone().unwrap(),
                    subnet_id: Some(subnet_id.to_string()),
                    no_pinning: prov_args.no_pinning,
                    push_only: true,
                };

                let create_body =
                    do_release_template(cfg, opts, args, template, admin_key.clone()).await;
                let password = create_body.password().to_vec();

                let unix_time =
                    SystemTime::now().duration_since(UNIX_EPOCH).expect("failed to get unix time");
                let noonce = unix_time.as_millis().to_le_bytes().to_vec();

                // Create attested request
                let request = CreateReleaseRequest::try_generate(
                    cfg as &LitConfig,
                    (create_body, Some(noonce), admin_key),
                )
                .await
                .expect("failed to create attested create release request");

                // Push action into oneshot
                let mut entry = ActionEntry::new();
                entry
                    .insert_setting(ACTION_SETTINGS_KEY_REQUEST.into(), request.to_vec())
                    .insert_setting(
                        ACTION_SETTINGS_KEY_GUEST_VCPUS.into(),
                        guest_vcpus.to_le_bytes().to_vec(),
                    );
                oneshot_cfg.insert_action(ACTION_TYPE_BOOTSTRAP.into(), entry);

                // Write password file
                write_password(instance_ci_dir, &password);

                // Add extra line to output (to separate from the 'do_release_template' call)
                if !opts.quiet() {
                    println!();
                }
            }
        }
        _ => {}
    }

    oneshot_cfg
}
