use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Clone, Debug)]
#[rasn(tag(explicit(application, 29)))]
pub struct EncKrbCredPart {
    #[rasn(tag(explicit(0)))]
    pub ticket_info: SequenceOf<krb_cred_info::KrbCredInfo>,

    #[rasn(tag(explicit(1)))]
    pub nonce: Option<uint32::UInt32>,

    #[rasn(tag(explicit(2)))]
    pub timestamp: Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(3)))]
    pub usec: Option<microseconds::Microseconds>,

    #[rasn(tag(explicit(4)))]
    pub s_adrress: Option<host_address::HostAddress>,

    #[rasn(tag(explicit(5)))]
    pub r_address: Option<host_address::HostAddress>
}