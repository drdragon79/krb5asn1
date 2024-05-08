use rasn::prelude::*;
use crate::*;


#[derive(AsnType, Decode, Encode, Clone, Debug)]
pub struct TypedDataEntry {
    #[rasn(tag(explicit(0)))]
    pub data_type: int32::Int32,

    #[rasn(tag(explicit(1)))]
    pub data_value: Option<OctetString>
}

pub type TypedData = SequenceOf<TypedDataEntry>;