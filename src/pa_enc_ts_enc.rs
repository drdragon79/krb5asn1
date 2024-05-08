use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct PaEncTsEnc {
    
    #[rasn(tag(explicit(0)))]
    pub patimestamp: kerberos_time::KerberosTime,

    #[rasn(tag(explicit(1)))]
    pub pausec: Option<microseconds::Microseconds>
}