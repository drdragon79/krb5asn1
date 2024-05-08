use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct KdcReqBody {

    #[rasn(tag(explicit(0)))]
    pub kdc_options: kdc_options::KdcOptions,

    #[rasn(tag(explicit(1)))]
    pub cname: Option<principal_name::PrincipalName>,

    #[rasn(tag(explicit(2)))]
    pub realm: realm::Realm,

    #[rasn(tag(explicit(3)))]
    pub sanme: Option<principal_name::PrincipalName>,

    #[rasn(tag(explicit(4)))]
    pub from: Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(5)))]
    pub till: kerberos_time::KerberosTime,

    #[rasn(tag(explicit(6)))]
    pub rtime:Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(7)))]
    pub nonce: uint32::UInt32,

    #[rasn(tag(explicit(8)))]
    pub etype: SequenceOf<int32::Int32>,

    #[rasn(tag(explicit(9)))]
    pub addresses: Option<host_addresses::HostAddresses>,

    #[rasn(tag(explicit(10)))]
    pub enc_authorization_data: Option<encrypted_data::EncryptedData>,

    #[rasn(tag(explicit(12)))]
    pub additional_tickets: Option<SequenceOf<ticket::Ticket>>
}