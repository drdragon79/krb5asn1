use rasn::prelude::*;
use crate::*;

use self::realm::Realm;

#[derive(AsnType, Decode, Encode, Clone, Debug)]
#[rasn(tag(explicit(application, 30)))]
pub struct KrbError {
    #[rasn(tag(explicit(0)))]
    pub pvno: int32::Int32,

    #[rasn(tag(explicit(1)))]
    pub msg_type: int32::Int32,

    #[rasn(tag(explicit(2)))]
    pub ctime: Option<kerberos_time::KerberosTime>,

    #[rasn(tag(explicit(3)))]
    pub usec: Option<microseconds::Microseconds>,

    #[rasn(tag(explicit(4)))]
    pub stime: kerberos_time::KerberosTime,

    #[rasn(tag(explicit(5)))]
    pub susec: microseconds::Microseconds,

    #[rasn(tag(explicit(6)))]
    pub error_code: int32::Int32,

    #[rasn(tag(explicit(7)))]
    pub crealm: Option<realm::Realm>,

    #[rasn(tag(explicit(8)))]
    pub cname: Option<principal_name::PrincipalName>,

    #[rasn(tag(explicit(9)))]
    pub realm: Realm,

    #[rasn(tag(explicit(10)))]
    pub sname: principal_name::PrincipalName,

    #[rasn(tag(explicit(11)))]
    pub e_text: Option<kerberos_string::KerberosString>,

    #[rasn(tag(explicit(12)))]
    pub e_data: Option<OctetString>
}