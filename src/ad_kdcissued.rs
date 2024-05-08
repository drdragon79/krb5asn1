use rasn::prelude::*;
use crate::*;

#[derive(AsnType, Decode, Encode, Debug, Clone)]
pub struct AdKdcIssued {
    #[rasn(tag(explicit(0)))]
    pub ad_checksum: checksum::Checksum,

    #[rasn(tag(explicit(1)))]
    pub i_realm: Option<realm::Realm>,

    #[rasn(tag(explicit(2)))]
    pub i_sname: Option<principal_name::PrincipalName>,

    #[rasn(tag(explicit(3)))]
    pub elaments: authorization_data::AuthorizationData
}