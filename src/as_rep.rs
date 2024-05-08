use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
#[rasn(delegate, tag(explicit(application, 11)))]
pub struct AsRep(pub kdc_rep::KdcRep);