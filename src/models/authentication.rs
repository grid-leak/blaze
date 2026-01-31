use std::sync::Arc;

use tdf::{TdfDeserialize, TdfSerialize, TdfType, TdfTyped};

use crate::session::User;

#[derive(Debug, TdfDeserialize)]
pub struct AuthRequest {
    #[tdf(tag = "AUTH")]
    pub token: String,
}

pub struct AuthResponse {
    pub user: Arc<User>,
}

impl TdfSerialize for AuthResponse {
    fn serialize<S: tdf::TdfSerializer>(&self, w: &mut S) {
        w.tag_zero(b"CNTX");
        w.tag_zero(b"ERRC");
        w.tag_str(b"SKEY", "0");
        w.tag_zero(b"ANON");
        w.tag_zero(b"NTOS");
        w.group(b"SESS", |w| {
            w.tag_zero(b"1CON");
            w.tag_u32(b"BUID", self.user.persona_id);
            w.tag_zero(b"FRST");
            w.tag_str(b"KEY", "0");
            w.tag_u32(b"LLOG", 1700769352);
            w.tag_str(b"MAIL", "******@beatrevival.me");
            w.group(b"PDTL", |w| {
                w.tag_str(b"DSNM", &self.user.username);
                w.tag_zero(b"LAST");
                w.tag_u32(b"PID", self.user.persona_id);
                w.tag_u8(b"PLAT", 4);
                w.tag_zero(b"STAS");
                w.tag_u32(b"XREF", self.user.user_id);
            });
            w.tag_u32(b"UID", self.user.user_id);
        });
        w.tag_zero(b"SPAM");
        w.tag_zero(b"UNDR");
    }
}

pub struct Entitlement {
    pub name: &'static str,
    pub id: u64,
    pub pjid: &'static str,
    pub prca: u8,
    pub prid: &'static str,
    pub tag: &'static str,
    pub ty: u8,
}

impl Entitlement {
    pub const fn pc(
        id: u64,
        pjid: &'static str,
        prca: u8,
        prid: &'static str,
        tag: &'static str,
    ) -> Self {
        Self {
            name: "MECATPC",
            id,
            pjid,
            prca,
            prid,
            tag,
            ty: 1,
        }
    }
}

impl TdfSerialize for Entitlement {
    fn serialize<S: tdf::TdfSerializer>(&self, w: &mut S) {
        w.tag_str_empty(b"DEVI");
        w.tag_str(b"GDAY", "2016-06-12T14:35Z");
        w.tag_str(b"GNAM", self.name);
        w.tag_u64(b"ID", self.id);
        w.tag_u8(b"ISCO", 0);
        w.tag_u8(b"PID", 0);
        w.tag_str(b"PJID", self.pjid);
        w.tag_u8(b"PRCA", self.prca);
        w.tag_str(b"PRID", self.prid);
        w.tag_u8(b"STAT", 1);
        w.tag_u8(b"STRC", 0);
        w.tag_str(b"TAG", self.tag);
        w.tag_str_empty(b"TDAY");
        w.tag_u8(b"TYPE", self.ty);
        w.tag_u8(b"UCNT", 0);
        w.tag_u8(b"VER", 0);
        w.tag_group_end();
    }
}

impl TdfTyped for Entitlement {
    const TYPE: TdfType = TdfType::Group;
}

#[derive(TdfSerialize)]
pub struct ListEntitlementsResponse {
    #[tdf(tag = "NLST")]
    pub list: &'static [Entitlement],
}
