use super::{
    int32::Int32,
    kerberos_string::KerberosString
};
use rasn::prelude::*;

#[derive(AsnType, Decode, Encode, Clone, Debug)]
pub struct PrincipalName {

    #[rasn(tag(explicit(0)))]
    pub name_type: Int32,

    #[rasn(tag(explicit(1)))]
    pub name_string: SequenceOf<KerberosString>
}

impl PrincipalName {
    pub fn new(name_type: Int32, name_string: &[u8]) -> PrincipalName {
        todo!()
    }

    pub fn get_principal_name_string(&self) -> Vec<u8> {
        todo!()
    }

    pub fn get_principal_name_string_at(&self, index: usize) -> Result<Vec<u8>,()> {
        todo!()
    }

    pub fn get_name_type(&self) -> Int32 {
        todo!()
    }
}