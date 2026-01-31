use tdf::{ObjectId, ObjectType, TdfSerialize, TdfType, TdfTyped};

pub struct AssociationList {
    name: &'static str,
    ty: u8,
    flgs: u8,
    lms: u32,
    pnam: &'static str,
    prid: u8,
    prms: u32,
}

impl AssociationList {
    pub const fn friend_list() -> Self {
        Self {
            name: "friendList",
            ty: 1,
            flgs: 4,
            lms: 2000,
            pnam: "",
            prid: 0,
            prms: 0,
        }
    }

    pub const fn follow_list() -> Self {
        Self {
            name: "followList",
            ty: 5,
            flgs: 2,
            lms: 200,
            pnam: "followerList",
            prid: 6,
            prms: 50000,
        }
    }

    pub const fn block_list() -> Self {
        Self {
            name: "communicationBlockList",
            ty: 4,
            flgs: 0,
            lms: 100,
            pnam: "",
            prid: 0,
            prms: 0,
        }
    }
}

impl TdfSerialize for AssociationList {
    fn serialize<S: tdf::TdfSerializer>(&self, w: &mut S) {
        w.group(b"INFO", |w| {
            w.tag_alt(
                b"BOID",
                ObjectId::new(ObjectType::new(25, self.ty as u16), 1011786733),
            );
            w.tag_u8(b"FLGS", self.flgs);
            w.group(b"LID", |w| {
                w.tag_str(b"LNM", self.name);
                w.tag_u8(b"TYPE", self.ty);
            });
            w.tag_u32(b"LMS", self.lms);
            w.tag_str(b"PNAM", self.pnam);
            w.tag_u8(b"PRID", self.prid);
            w.tag_u32(b"PRMS", self.prms);
        });
        w.tag_zero(b"OFRC");
        w.tag_zero(b"TOCT");
        w.tag_group_end();
    }
}

impl TdfTyped for AssociationList {
    const TYPE: TdfType = TdfType::Group;
}

#[derive(TdfSerialize)]
pub struct GetListsResponse {
    #[tdf(tag = "LMAP")]
    pub list: &'static [AssociationList],
}
