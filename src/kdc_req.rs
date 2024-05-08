use rasn::prelude::*;
use crate::*;

use self::int32::Int32;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct KdcReq {

    #[rasn(tag(explicit(1)))]
    pub pvno: Int32,

    #[rasn(tag(explicit(2)))]
    pub msg_type: Int32,

    #[rasn(tag(explicit(3)))]
    pub padata: Option<SequenceOf<pa_data::PaData>>,

    #[rasn(tag(explicit(4)))]
    pub req_body: kdc_req_body::KdcReqBody

}