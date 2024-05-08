use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct KdcRep {

    #[rasn(tag(explicit(0)))]
    pub pvno: int32::Int32,

    #[rasn(tag(explicit(1)))]
    pub mgs_type: int32::Int32,

    #[rasn(tag(explicit(2)))]
    pub padata: Option<SequenceOf<pa_data::PaData>>,

    #[rasn(tag(explicit(3)))]
    pub crealm: realm::Realm,

    #[rasn(tag(explicit(4)))]
    pub cname: principal_name::PrincipalName,

    #[rasn(tag(explicit(5)))]
    pub ticket: ticket::Ticket,

    #[rasn(tag(explicit(6)))]
    pub enc_part: encrypted_data::EncryptedData
}