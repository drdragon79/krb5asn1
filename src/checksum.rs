use rasn::prelude::*;
use crate::int32::Int32;


#[derive(AsnType, Decode, Encode, Clone, Debug)]
pub struct Checksum {

    #[rasn(tag(explicit(0)))]
    pub cksumtype: Int32,

    #[rasn(tag(explicit(1)))]
    pub checksum: OctetString

}