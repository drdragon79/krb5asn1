use rasn::prelude::*;
use crate::{
    int32::Int32,
    principal_name::PrincipalName,
    realm::Realm,
    encrypted_data::EncryptedData
};

#[derive(AsnType, Decode, Encode, Clone, Debug)]
#[rasn(tag(explicit(application, 1)))]
pub struct Ticket {

    #[rasn(tag(explicit(0)), value("1"))]
    pub tkt_vno: Int32,

    #[rasn(tag(explicit(0)))]
    pub realm: Realm,

    #[rasn(tag(explicit(0)))]
    pub sname: PrincipalName,

    #[rasn(tag(explicit(0)))]
    pub enc_part: EncryptedData
}