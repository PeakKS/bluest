pub mod adapter;
pub mod characteristic;
pub mod descriptor;
pub mod device;
pub mod l2cap_channel;
pub mod service;

mod error;

use std::str::FromStr;

/// A platform-specific device identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeviceId(bluer::Address);

impl FromStr for DeviceId {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match bluer::Address::from_str(s) {
            Ok(addr) => Ok(DeviceId(addr)),
            Err(e) => Err(e)
        }
    }
    type Err = bluer::InvalidAddress;
}
