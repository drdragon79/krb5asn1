use rasn::prelude::*;

use super::{
    int32::Int32,
    uint32::UInt32
};

#[derive(AsnType, Decode, Encode, Clone, Debug)]
pub struct EncryptedData {

    #[rasn(tag(explicit(0)))]
    etype: Int32,

    #[rasn(tag(explicit(1)))]
    kvno: Option<UInt32>,

    #[rasn(tag(explicit(2)))]
    cipher: OctetString
}