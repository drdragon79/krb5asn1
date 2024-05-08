use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct AdAndOr {
    #[rasn(tag(explicit(0)))]
    pub condition_count: int32::Int32,

    #[rasn(tag(explicit(1)))]
    pub elements: authorization_data::AuthorizationData
}