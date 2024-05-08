use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct EtypeInfoEntry {
    
    #[rasn(tag(explicit(0)))]
    pub etype: int32::Int32,

    #[rasn(tag(explicit(1)))]
    pub salt: Option<OctetString>
}