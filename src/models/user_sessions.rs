use std::sync::Arc;

use tdf::{ObjectId, ObjectType, TdfSerialize, TdfSerializer};

use crate::session::User;

pub struct UpdateHardwareFlags {
    pub user: Arc<User>,
}

impl TdfSerialize for UpdateHardwareFlags {
    fn serialize<S: tdf::TdfSerializer>(&self, w: &mut S) {
        w.tag_zero(b"1CON");
        w.tag_u32(b"ALOC", 1920292161);
        w.tag_u32(b"BUID", self.user.persona_id);
        w.tag_alt(b"CGID", ObjectId::new(ObjectType::new(30722, 2), 88123840));
        w.tag_str(b"DSNM", &self.user.username);
        w.tag_zero(b"FRST");
        w.tag_str(b"KEY", "0");
        w.tag_u32(b"LAST", 1700769152);
        w.tag_u32(b"LLOG", 1700769258);
        w.tag_str(b"MAIL", "******@beatrevival.me");
        w.tag_str(b"NASP", "cem_ea_id");
        w.tag_owned(b"PID", self.user.persona_id);
        w.tag_u8(b"PLAT", 4);
        w.tag_owned(b"UID", self.user.user_id);
        w.tag_u8(b"USTP", 0);
        w.tag_owned(b"XREF", self.user.user_id);
    }
}

pub struct UpdateExtendedDataAttribute {
    pub user: Arc<User>,
}

impl TdfSerialize for UpdateExtendedDataAttribute {
    fn serialize<S: tdf::TdfSerializer>(&self, w: &mut S) {
        w.tag_u8(b"FLGS", 3);
        w.tag_u32(b"ID", self.user.persona_id);
    }
}

pub struct UserSessionExtendedData {
    pub user: Arc<User>,
}

impl TdfSerialize for UserSessionExtendedData {
    fn serialize<S: tdf::TdfSerializer>(&self, w: &mut S) {
        w.group(b"DATA", |w| {
            w.tag_union_value(b"ADDR", 0, b"BPS", &"");
            w.tag_str(b"CTY", "");
            w.tag_var_int_list_empty(b"CVAR");
            w.tag_map_tuples(b"DMAP", &[(2013396993, 0)]);
            w.tag_u32(b"HWFG", 0);
            w.tag_str(b"ISP", "");
            w.group(b"QDAT", |w| {
                w.tag_u32(b"BWHR", 0);
                w.tag_u32(b"DBPS", 0);
                w.tag_u32(b"NAHR", 0);
                w.tag_u32(b"NATT", 0);
                w.tag_u32(b"UBPS", 0);
            });
            w.tag_str(b"TZ", "");
            w.tag_u32(b"UATT", 0);

            w.tag_var_int_list(b"ULST", &[30722, 2, 88123840]);

            w.group(b"USER", |w| {
                w.tag_u32(b"AID", self.user.user_id);
                w.tag_u32(b"ALOC", 1920292161);
                w.tag_blob_empty(b"EXBB");
                w.tag_u32(b"EXID", self.user.user_id);
                w.tag_u32(b"ID", self.user.persona_id);
                w.tag_str(b"NAME", &self.user.username);
                w.tag_str(b"NASP", "cem_ea_id");
                w.tag_u32(b"ORIG", self.user.persona_id);
                w.tag_u32(b"PIDI", 0);
            });
        });
    }
}

pub struct ValidateSessionKey {
    pub user: Arc<User>,
}

impl TdfSerialize for ValidateSessionKey {
    fn serialize<S: tdf::TdfSerializer>(&self, w: &mut S) {
        w.group(b"DATA", |w| {
            w.tag_union_value(b"ADDR", 2, b"VALU", &{
                let mut w = Vec::new();
                w.group(b"EXIP", |w| {
                    w.tag_u32(b"IP", 2130706433);
                    w.tag_u32(b"MACI", 0);
                    w.tag_u16(b"PORT", 3659);
                });
                w.group(b"INIP", |w| {
                    w.tag_u32(b"IP", 3232235776);
                    w.tag_u32(b"MACI", 0);
                    w.tag_u16(b"PORT", 3659);
                });
                w.tag_u32(b"MACI", 1129238128);
                w
            });
            w.tag_str(b"BPS", "");
            w.tag_str(b"CTY", "");
            w.tag_var_int_list_empty(b"CVAR");
            w.tag_map_tuples(b"DMAP", &[(2013396993, 0)]);
            w.tag_u32(b"HWFG", 0);
            w.tag_str(b"ISP", "");
            w.group(b"QDAT", |w| {
                w.tag_u32(b"BWHR", 0);
                w.tag_u32(b"DBPS", 0);
                w.tag_u32(b"NAHR", 0);
                w.tag_u32(b"NATT", 0);
                w.tag_u32(b"UBPS", 0);
            });
            w.tag_str(b"TZ", "");
            w.tag_u32(b"UATT", 0);
            w.tag_alt(b"ULST", ObjectId::new(ObjectType::new(30722, 2), 88123840));
        });

        w.tag_u32(b"SUBS", 1);
        w.tag_u32(b"USID", self.user.persona_id);
    }
}
