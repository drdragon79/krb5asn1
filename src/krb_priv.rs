use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
#[rasn(tag(explicit(application, 21)))]
pub struct KrbPriv {
    #[rasn(tag(explicit(0)))]
    pub pvno: int32::Int32,

    #[rasn(tag(explicit(1)))]
    pub msg_type: int32::Int32,

    #[rasn(tag(explicit(2)))]
    pub enc_part: encrypted_data::EncryptedData
}