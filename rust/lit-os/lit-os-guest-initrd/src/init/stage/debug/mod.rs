use std::net::{IpAddr, ToSocketAddrs};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;

use async_std_resolver::config::{ResolverConfig, ResolverOpts};
use async_std_resolver::{resolver, AsyncStdResolver};
use log::{error, info};
use surge_ping::IcmpPacket;

use lit_os_core::error::{io_err, Result};

use crate::init::context::InitContext;
use crate::init::stage::Outcome;

const TEST_PING_HOSTS: [&str; 5] =
    ["172.30.0.1", "1.1.1.1", "8.8.8.8", "polygon-rpc.com", "arb-mainnet.g.alchemy.com"];
const TEST_RESOLVE_HOSTS: [&str; 3] =
    ["litprotocol.com", "polygon-rpc.com", "arb-mainnet.g.alchemy.com"];

pub(crate) async fn run(ctx: &mut InitContext) -> Result<Outcome> {
    info!("Attempting to diagnose fault");

    // Dump network state
    dump_network(ctx).await?;

    // Test network connectivity
    test_network(ctx).await?;

    // Dump disks
    dump_disks(ctx).await?;

    // Safety, ensure if this is used it will not cause harm line Continue would.
    Ok(Outcome::Halt)
}

async fn test_network(_ctx: &mut InitContext) -> Result<()> {
    // First try and resolve via the rust default DNS resolver.
    for host in TEST_RESOLVE_HOSTS {
        match (host, 80_u16).to_socket_addrs() {
            Ok(r) => {
                let resolved: Vec<String> = r.map(|s| s.ip().to_string()).collect();

                info!("Resolved OK: {} ({})", host, resolved.join(", "));
            }
            Err(e) => {
                error!("Failed to resolve: {} - {:?}", host, e);
            }
        }
    }

    // This resolver is more resilient than the ToSocketAddrs (but ToSocketAddrs fails sometimes).
    let mut resolver = resolver(ResolverConfig::cloudflare(), ResolverOpts::default())
        .await
        .map_err(|e| io_err(e, Some("failed to construct DNS resolver".into())))?;

    for host in TEST_PING_HOSTS {
        let ip = resolve_host(&mut resolver, host).await?;

        match surge_ping::ping(ip, &[1, 2, 3, 4, 5, 6, 7, 8]).await {
            Ok((IcmpPacket::V4(packet), duration)) => {
                info!(
                    "Ping OK: {} bytes from {} ({}): icmp_seq={} ttl={:?} time={:.2?}",
                    packet.get_size(),
                    host,
                    packet.get_source(),
                    packet.get_sequence(),
                    packet.get_ttl(),
                    duration
                );
            }
            Ok(_) => {
                error!("Failed to ping: {} ({}) (no response)", host, ip.to_string())
            }
            Err(e) => {
                error!("Failed to ping: {} ({}) - {:?}", host, ip.to_string(), e)
            }
        }
    }

    Ok(())
}

async fn resolve_host(resolver: &mut AsyncStdResolver, host: &str) -> Result<IpAddr> {
    // Is already IP?
    let res = IpAddr::from_str(host);
    if let Ok(ip) = res {
        return Ok(ip);
    }

    // Resolve host
    let lookup = resolver
        .lookup_ip(host)
        .await
        .map_err(|e| io_err(e, Some(format!("failed to lookup IP for: {host}"))))?;

    let address =
        lookup.iter().next().ok_or_else(|| io_err(format!("no IPs found for: {host}"), None))?;

    Ok(address)
}

async fn dump_network(_ctx: &mut InitContext) -> Result<()> {
    info!("Dumping network state");

    for (cmd, args) in [
        ("ip", vec!["addr"]),
        ("ip", vec!["route"]),
        //("cat", vec!["/etc/hosts"]),
        ("cat", vec!["/etc/resolv.conf"]),
    ] {
        info!("Exec: {} {}", cmd, args.join(" "));

        let _ =
            Command::new(cmd).args(args).stderr(Stdio::inherit()).stdout(Stdio::inherit()).output();
    }

    Ok(())
}

async fn dump_disks(_ctx: &mut InitContext) -> Result<()> {
    info!("Dumping disk devices");

    let cmds = if PathBuf::from("/dev/disk/by-uuid").exists() {
        vec![("ls", vec!["/dev/disk/by-uuid"]), ("ls", vec!["/dev/disk/by-partlabel"])]
    } else {
        vec![("ls", vec!["-R", "/dev/disk"])]
    };

    for (cmd, args) in cmds {
        info!("Exec: {} {}", cmd, args.join(" "));

        let _ =
            Command::new(cmd).args(args).stderr(Stdio::inherit()).stdout(Stdio::inherit()).output();
    }

    Ok(())
}
