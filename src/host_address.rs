use rasn::prelude::*;
use super::int32::Int32;

#[derive(AsnType, Decode, Encode, Clone, Debug)]
pub struct HostAddress {

    #[rasn(tag(explicit(0)))]
    pub addr_type: Int32,

    #[rasn(tag(explicit(1)))]
    pub address: OctetString
}