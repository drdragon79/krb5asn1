use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct EncKdcRepPart {

    #[rasn(tag(explicit(0)))]
    pub key: encryption_key::EncryptionKey,
     
    #[rasn(tag(explicit(1)))]
    pub last_req: last_req::LastReq,
    
    #[rasn(tag(explicit(2)))]
    pub none: uint32::UInt32,

    #[rasn(tag(explicit(3)))]
    pub key_expiration: Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(4)))]
    pub flags: ticket_flags::TicketFlags,

    #[rasn(tag(explicit(5)))]
    pub authtime: kerberos_time::KerberosTime,

    #[rasn(tag(explicit(6)))]
    pub starttime: Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(7)))]
    pub endtime: kerberos_time::KerberosTime,

    #[rasn(tag(explicit(8)))]
    pub renew_till: Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(9)))]
    pub srealm: realm::Realm,

    #[rasn(tag(explicit(10)))]
    pub sname: principal_name::PrincipalName,

    #[rasn(tag(explicit(11)))]
    pub caddr: Option<host_addresses::HostAddresses>
}