use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct KrbSafeBody {
    #[rasn(tag(explicit(0)))]
    pub user_data: OctetString,

    #[rasn(tag(explicit(1)))]
    pub timestamp: Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(2)))]
    pub usec: Option<microseconds::Microseconds>,

    #[rasn(tag(explicit(3)))]
    pub seq_number: Option<uint32::UInt32>,

    #[rasn(tag(explicit(4)))]
    pub s_address: host_address::HostAddress,

    #[rasn(tag(explicit(5)))]
    pub r_address: Option<host_addresses::HostAddresses>
}