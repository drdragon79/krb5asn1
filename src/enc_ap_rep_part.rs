use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
#[rasn(tag(explicit(application, 27)))]
pub struct EncApRepPart {
    #[rasn(tag(explicit(0)))]
    pub ctime: kerberos_time::KerberosTime,
    
    #[rasn(tag(explicit(1)))]
    pub cusec: microseconds::Microseconds,

    #[rasn(tag(explicit(2)))]
    pub subkey: Option<encryption_key::EncryptionKey>,

    #[rasn(tag(explicit(3)))]
    pub seq_number: Option<uint32::UInt32>
}