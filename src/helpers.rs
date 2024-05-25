#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

use rasn::prelude::*;
use crate::structures::*;
use crate::constants::*;

impl AsReq {
    fn new(
        padata: Option<SequenceOf<PaData>>,
        req_body: KdcReqBody
    ) -> AsReq {
        AsReq(
            KdcReq::new(protocol::pvno, 10 , padata, req_body)
        )
    }
}

impl KdcReq {
    fn new(
        pvno: Int32,
        msg_type: Int32,
        padata: Option<SequenceOf<PaData>>,
        req_body: KdcReqBody
    ) -> KdcReq {
        KdcReq {
            pvno,
            msg_type,
            padata,
            req_body
        }
    }
}

impl KdcReqBody {
    fn new(
        kdc_options: KdcOptions,
        cname: Option<PrincipalName>,
        realm: Realm,
        sname: Option<PrincipalName>,
        from: Option<KerberosTime>,
        till: KerberosTime,
        rtime: Option<KerberosTime>,
        nonce: UInt32,
        etype: SequenceOf<Int32>,
        addresses: Option<HostAddresses>,
        enc_authorization_data: Option<EncryptedData>,
        additional_tickets: Option<SequenceOf<Ticket>>
    ) -> KdcReqBody {
        KdcReqBody {
            kdc_options,
            cname,
            realm,
            sname,
            from,
            till,
            rtime,
            nonce,
            etype,
            addresses,
            enc_authorization_data,
            additional_tickets
        }
    }
}

impl PaData {
    fn new(padata_type: Int32, padata_value: OctetString) -> PaData {
        PaData {
            padata_type,
            padata_value
        }
    }
}

impl PrincipalName {
    fn new(name_type: Int32, name_string: SequenceOf<KerberosString>) -> PrincipalName {
        PrincipalName {
            name_type,
            name_string
        }
    }
}
