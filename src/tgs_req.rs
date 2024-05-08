use rasn::prelude::*;
use crate::*;


#[derive(AsnType, Decode, Encode, Debug, Clone)]
#[rasn(delegate, tag(explicit(application, 12)))]
pub struct AsReq(pub kdc_req::KdcReq);