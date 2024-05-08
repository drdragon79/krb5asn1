use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct LastReq {

    #[rasn(tag(explicit(0)))]
    lr_type: int32::Int32,

    #[rasn(tag(explicit(1)))]
    lr_value: kerberos_time::KerberosTime
}