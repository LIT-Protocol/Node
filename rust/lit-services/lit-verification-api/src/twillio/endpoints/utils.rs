use log::trace;
use moka::future::Cache;
use rocket::State;

use crate::{client_ip::ClientRealAddr, models::CacheEntry};

pub fn check_cache_for_client_ip(
    cache: &State<Cache<String, CacheEntry>>, request_ip: String,
) -> Option<String> {
    let itr = cache.into_iter();

    for cache_item in itr {
        trace!("Querying session cache item: key {}, value: {:?}", cache_item.0, cache_item.1);
        if cache_item.1.client_ip == request_ip {
            trace!(
                "Found client ip in cache: {} matching request ip address {}",
                cache_item.1.client_ip,
                request_ip
            );

            // copy the value out
            return Some(cache_item.0.to_string());
        }
    }

    trace!("No request found matching ip in cache");
    None
}

pub fn get_ip_from_request(client_ip: &ClientRealAddr) -> String {
    // prefer an ipv4 address. if None is returned, use the ipv6
    match client_ip.get_ipv4_string() {
        Some(ip) => ip,
        None => client_ip.get_ipv6_string(),
    }
}
