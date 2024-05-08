use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
#[rasn(tag(explicit(application, 20)))]
pub struct KrbSafe {
    #[rasn(tag(explicit(0)))]
    pub pvno: int32::Int32,

    #[rasn(tag(explicit(1)))]
    pub msg_type: int32::Int32,

    #[rasn(tag(explicit(2)))]
    pub safe_body: krb_safe_body::KrbSafeBody,

    #[rasn(tag(explicit(3)))]
    pub cksum: checksum::Checksum
}