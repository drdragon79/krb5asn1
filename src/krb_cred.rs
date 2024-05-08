use rasn::prelude::*;
use crate::*;


#[derive(AsnType, Decode, Encode, Clone, Debug)]
#[rasn(tag(explicit(application, 22)))]
pub struct KrbCred {
    #[rasn(tag(explicit(0)))]
    pub pnvo: int32::Int32,

    #[rasn(tag(explicit(0)))]
    pub msg_type: int32::Int32,

    #[rasn(tag(explicit(0)))]
    pub tickets: SequenceOf<ticket::Ticket>,

    #[rasn(tag(explicit(0)))]
    pub enc_part: encrypted_data::EncryptedData
}