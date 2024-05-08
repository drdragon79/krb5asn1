use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct KrbCredInfo {
    #[rasn(tag(explicit(0)))]
    pub key: encryption_key::EncryptionKey,

    #[rasn(tag(explicit(1)))]
    pub prealm: Option<realm::Realm>,

    #[rasn(tag(explicit(2)))]
    pub pname: Option<principal_name::PrincipalName>,

    #[rasn(tag(explicit(3)))]
    pub flags: Option<ticket_flags::TicketFlags>,

    #[rasn(tag(explicit(4)))]
    pub authtime: Option<kerberos_time::KerberosTime>,
    
    #[rasn(tag(explicit(5)))]
    pub starttime: Option<kerberos_time::KerberosTime>,
    
    #[rasn(tag(explicit(6)))]
    pub endtime: Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(7)))]
    pub renew_till: Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(8)))]
    pub srealm: Option<realm::Realm>,

    #[rasn(tag(explicit(9)))]
    pub sname: Option<principal_name::PrincipalName>,

    #[rasn(tag(explicit(10)))]
    pub caddr: Option<host_addresses::HostAddresses>
}