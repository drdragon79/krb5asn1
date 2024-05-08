use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
#[rasn(tag(explicit(application, 2)))]
pub struct Authenticator {
    #[rasn(tag(explicit(0)))]
    pub authenticator_vno: int32::Int32,
    
    #[rasn(tag(explicit(1)))]
    pub creaml: realm::Realm,

    #[rasn(tag(explicit(2)))]
    pub cname: principal_name::PrincipalName,

    #[rasn(tag(explicit(3)))]
    pub cksum: Option<checksum::Checksum>,

    #[rasn(tag(explicit(4)))]
    pub cusec: microseconds::Microseconds,

    #[rasn(tag(explicit(5)))]
    pub ctime: kerberos_time::KerberosTime,

    #[rasn(tag(explicit(6)))]
    pub subkey: Option<encryption_key::EncryptionKey>,

    #[rasn(tag(explicit(7)))]
    pub seq_number: Option<uint32::UInt32>,

    #[rasn(tag(explicit(8)))]
    pub authorization_data: Option<authorization_data::AuthorizationData>
}