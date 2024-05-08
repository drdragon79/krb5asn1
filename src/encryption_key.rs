use rasn::prelude::*;
use super::int32::Int32;

#[derive(AsnType, Debug, Decode, Encode, Clone)]
pub struct EncryptionKey {

    #[rasn(tag(explicit(0)))]
    pub keytype: Int32,
    
    #[rasn(tag(explicit(1)))]
    pub keyvalue: OctetString
}