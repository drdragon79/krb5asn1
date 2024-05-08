use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Clone, Debug)]
#[rasn(delegate, tag(explicit(application, 25)))]
pub struct EncAsRepPart(pub enc_kdc_rep_part::EncKdcRepPart);