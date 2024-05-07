use rasn::prelude::*;
use super::int32::Int32;

pub struct HostAddress {
    pub addr_type: Int32,
    pub address: OctetString
}