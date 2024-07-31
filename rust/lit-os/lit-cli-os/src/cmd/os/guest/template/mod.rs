use crate::cmd::os::guest::template::cleanup::handle_cmd_os_guest_template_cleanup;
use clap::{Args, Subcommand};
use lit_cli_core::cmd::CliGlobalOpts;
use lit_core::config::LitConfig;

use crate::cmd::os::guest::template::create::{
    handle_cmd_os_guest_template_create, GuestTemplateCreate,
};
use crate::cmd::os::guest::template::delete::{
    handle_cmd_os_guest_template_delete, GuestTemplateDelete,
};
use crate::cmd::os::guest::template::describe::{
    handle_cmd_os_guest_template_describe, GuestTemplateDescribe,
};
use crate::cmd::os::guest::template::ls::handle_cmd_os_guest_template_ls;
use crate::cmd::os::guest::template::release::{
    handle_cmd_os_guest_template_release, GuestTemplateRelease,
};

pub(crate) mod cleanup;
pub(crate) mod create;
pub(crate) mod delete;
pub(crate) mod describe;
pub(crate) mod ls;
pub(crate) mod release;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub(crate) struct GuestTemplate {
    #[command(subcommand)]
    command: GuestTemplateCommands,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum GuestTemplateCommands {
    /// Create guest template (release)
    #[command(arg_required_else_help = true)]
    Create(GuestTemplateCreate),
    /// Delete a guest template
    #[command(arg_required_else_help = true)]
    Delete(GuestTemplateDelete),
    /// Show details for a guest template
    #[command(arg_required_else_help = true)]
    Describe(GuestTemplateDescribe),
    /// List all guest templates
    Ls {},
    /// Release a template
    Release(GuestTemplateRelease),
    /// Clean up failed builds
    CleanUp {},
}

pub(crate) async fn handle_cmd_os_guest_template(
    cfg: LitConfig, opts: CliGlobalOpts, args: GuestTemplate,
) -> bool {
    match args.command {
        GuestTemplateCommands::Create(args) => {
            handle_cmd_os_guest_template_create(cfg, opts, args).await
        }
        GuestTemplateCommands::Delete(args) => {
            handle_cmd_os_guest_template_delete(&cfg, &opts, args)
        }
        GuestTemplateCommands::Describe(args) => {
            handle_cmd_os_guest_template_describe(cfg, opts, args)
        }
        GuestTemplateCommands::Ls {} => handle_cmd_os_guest_template_ls(&cfg, &opts),
        GuestTemplateCommands::Release(args) => {
            handle_cmd_os_guest_template_release(cfg, opts, args).await
        }
        GuestTemplateCommands::CleanUp {} => handle_cmd_os_guest_template_cleanup(&cfg, &opts),
    }
}
