use rasn::prelude::*;
use super::int32::Int32;

#[derive(AsnType, Decode, Encode, Clone, Debug)]
pub struct AdEntry {

    #[rasn(tag(explicit(1)))]
    pub ad_type: Int32,

    #[rasn(tag(explicit(2)))]
    pub ad_data: OctetString
}

pub type AuthorizationData = SequenceOf<AdEntry>;