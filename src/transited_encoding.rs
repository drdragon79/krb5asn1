use rasn::prelude::*;
use crate::int32::Int32;

#[derive(AsnType, Decode, Encode, Clone, Debug)]
pub struct TransitedEncoding {

    #[rasn(tag(explicit(0)))]
    pub tr_type: Int32,

    #[rasn(tag(explicit(1)))]
    pub contents: OctetString
}