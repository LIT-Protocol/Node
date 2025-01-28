pub mod restart;
pub mod upgrade;
use crate::error::Error;
use ethers::types::{Address, U256};
use lit_cli_os::guest::instance::helper::GuestInstanceItemHelper;
use lit_cli_os::guest::instance::GuestInstanceItem;
pub use restart::Restart;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tracing::debug;
pub use upgrade::Upgrade;

pub(crate) trait EventHandler {
    // called by Event implementations
    fn expiration_time(&self) -> U256;
    fn stake_address(&self) -> Address;
    fn handle(&self, instance_item: &GuestInstanceItem) -> Result<(), Error>;
    fn is_event_for_instance(&self, instance_item: &GuestInstanceItem) -> bool {
        let my_stake_address =
            instance_item.staker_address().expect("node type node must have a stakeaddress");
        let event_stake_address = self.stake_address();
        debug!("my address {:?}, event address {:?}", my_stake_address, event_stake_address);
        my_stake_address == event_stake_address
    }
    fn is_unexpired(&self) -> bool {
        let now: U256 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system should have a time")
            .as_secs()
            .into();
        let expires = self.expiration_time();
        debug!("system {:?}, expiration {:?}", now, expires);
        now < expires
    }
}
