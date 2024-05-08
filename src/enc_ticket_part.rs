use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Clone, Debug)]
#[rasn(tag(explicit(application, 3)))]
pub struct EncTicketPart {

    #[rasn(tag(explicit(0)))]
    pub flags: ticket_flags::TicketFlags,

    #[rasn(tag(explicit(1)))]
    pub key: encryption_key::EncryptionKey,

    #[rasn(tag(explicit(2)))]
    pub crealm: realm::Realm,

    #[rasn(tag(explicit(3)))]
    pub cname: principal_name::PrincipalName,

    #[rasn(tag(explicit(4)))]
    pub transited: transited_encoding::TransitedEncoding,

    #[rasn(tag(explicit(5)))]
    pub authtime: kerberos_time::KerberosTime,

    #[rasn(tag(explicit(6)))]
    pub starttime: Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(7)))]
    pub endtime: kerberos_time::KerberosTime,

    #[rasn(tag(explicit(8)))]
    pub renew_till: Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(9)))]
    pub caddr: Option<host_addresses::HostAddresses>,

    #[rasn(tag(explicit(10)))]
    pub authorization_data: Option<authorization_data::AuthorizationData>
}