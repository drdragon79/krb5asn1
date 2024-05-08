use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct EtypeInfo2Entry {
    
    #[rasn(tag(explicit(0)))]
    pub etype: int32::Int32,

    #[rasn(tag(explicit(1)))]
    pub salt: Option<kerberos_string::KerberosString>,

    #[rasn(tag(explicit(2)))]
    pub s2kparams: Option<OctetString>

}