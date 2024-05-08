use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
#[rasn(tag(explicit(application, 14)))]
pub struct ApReq {
    #[rasn(tag(explicit(0)))]
    pub pvno: int32::Int32,

    #[rasn(tag(explicit(1)))]
    pub msg_type: int32::Int32,

    #[rasn(tag(explicit(2)))]
    pub ap_options: ap_options::ApOptions,

    #[rasn(tag(explicit(3)))]
    pub ticket: ticket::Ticket,

    #[rasn(tag(explicit(4)))]
    pub authenticator: encrypted_data::EncryptedData
}