use rasn::prelude::*;
use super::int32::Int32;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct PaData {

    #[rasn(tag(explicit(1)))]
    padata_type: Int32,

    #[rasn(tag(explicit(2)))]
    padata_value: OctetString
}