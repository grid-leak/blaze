use std::sync::Arc;

use tdf::{TdfDeserialize, TdfMap, TdfSerialize, TdfSerializer, TdfTyped};

use crate::session::User;

#[derive(TdfSerialize, TdfTyped, Copy, Clone, Debug)]
#[tdf(group)]
pub struct Bwps {
    #[tdf(tag = "PSA ")]
    pub psa: &'static str,
    #[tdf(tag = "PSP ")]
    pub psp: u16,
    #[tdf(tag = "SNA ")]
    pub sna: &'static str,
}

static PING_SITE_ATLAS: [(&str, Bwps); 6] = [
    (
        "bio-dub",
        Bwps {
            psa: "qos-prod-bio-dub-common-common.gos.ea.com",
            psp: 17504,
            sna: "bio-dub-prod",
        },
    ),
    (
        "bio-iad",
        Bwps {
            psa: "qos-prod-bio-iad-common-common.gos.ea.com",
            psp: 17504,
            sna: "bio-iad-prod",
        },
    ),
    (
        "bio-sjc",
        Bwps {
            psa: "qos-prod-bio-sjc-common-common.gos.ea.com",
            psp: 17504,
            sna: "bio-sjc-prod",
        },
    ),
    (
        "bio-syd",
        Bwps {
            psa: "qos-prod-bio-syd-common-common.gos.ea.com",
            psp: 17504,
            sna: "bio-syd-prod",
        },
    ),
    (
        "m3d-brz",
        Bwps {
            psa: "qos-prod-m3d-brz-common-common.gos.ea.com",
            psp: 17504,
            sna: "m3d-brz-prod",
        },
    ),
    (
        "m3d-nrt",
        Bwps {
            psa: "qos-prod-m3d-nrt-common-common.gos.ea.com",
            psp: 17504,
            sna: "m3d-nrt-prod",
        },
    ),
];

pub struct PreAuthResponse;

impl TdfSerialize for PreAuthResponse {
    fn serialize<S: tdf::TdfSerializer>(&self, w: &mut S) {
        w.tag_str(b"ASRC", "308903");
        w.tag_list_slice(
            b"CIDS",
            &[
                30728, 24, 1, 30729, 25, 30730, 27, 9, 10, 33, 63490, 15, 30720, 30722, 30723,
                30724, 21, 30726, 2000, 30727,
            ],
        );
        w.tag_str(b"CLID", "MirrorsEdgeCatalyst-SERVER-PC");
        w.group(b"CONF", |w| {
            w.tag_map_tuples(
                b"CONF",
                &[
                    ("associationListSkipInitialSet", "1"),
                    ("autoReconnectEnabled", "0"),
                    ("bytevaultHostname", "bytevault.gameservices.ea.com"),
                    ("bytevaultPort", "42210"),
                    ("bytevaultSecure", "true"),
                    ("connIdleTimeout", "40s"),
                    ("defaultRequestTimeout", "20s"),
                    ("nucleusConnect", "https://accounts.ea.com"),
                    ("nucleusConnectTrusted", "https://accounts2s.ea.com"),
                    ("nucleusPortal", "https://signin.ea.com"),
                    ("nucleusProxy", "https://gateway.ea.com"),
                    ("pingPeriod", "20s"),
                    ("userManagerMaxCachedUsers", "0"),
                    ("voipHeadsetUpdateRate", "1000"),
                    ("xblTokenUrn", "accounts.ea.com"),
                    ("xboxOneStringValidationUri", "client-strings.xboxlive.com"),
                    ("xlspConnectionIdleTimeout", "300"),
                ],
            );
        });
        w.tag_str(b"ESRC", "308903");
        w.tag_str(b"INST", "mirrorsedgecatalyst-2016-pc");
        w.tag_u32(b"MAID", 1129238128);
        w.tag_u8(b"MINR", 0);
        w.tag_str(b"NASP", "cem_ea_id");
        w.tag_str(b"PILD", "");
        w.tag_str(b"PLAT", "pc");
        w.group(b"QOSS", |w| {
            w.tag_ref(
                b"BWPS",
                &Bwps {
                    psa: "",
                    psp: 0,
                    sna: "",
                },
            );
            w.tag_u8(b"LNP ", 10);

            w.tag_map_tuples(b"LTPS", &PING_SITE_ATLAS);

            w.tag_u32(b"SVID", 1161889797);
            w.tag_u64(b"TIME", 5_000_000);
        });
        w.tag_str(b"RSRC", "308903");
        w.tag_str(b"SVER", "Blaze 15.1.1.0.5 (CL# 1893137)");
    }
}

#[derive(TdfSerialize)]
pub struct PingResponse {
    #[tdf(tag = "TIME")]
    pub time: u64,
}

#[derive(Debug, TdfDeserialize)]
pub struct ClientConfigRequest {
    #[tdf(tag = "CFID")]
    pub id: String,
}

pub struct ClientConfigResponse {
    pub config: TdfMap<String, String>,
}

impl TdfSerialize for ClientConfigResponse {
    fn serialize<S: TdfSerializer>(&self, w: &mut S) {
        let entries: Vec<(&str, &str)> = self
            .config
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        w.tag_map_tuples(b"CONF", &entries);
    }
}

pub struct PostAuthResponse {
    pub user: Arc<User>,
}

impl TdfSerialize for PostAuthResponse {
    fn serialize<S: TdfSerializer>(&self, w: &mut S) {
        w.group(b"TELE", |w| {
            w.tag_str(b"ADRS", "https://river.data.ea.com");
            w.tag_zero(b"ANON");
            w.tag_str(b"DISA", "");
            w.tag_zero(b"EDCT");
            w.tag_str(b"FILT", "-UION/****");
            w.tag_u32(b"LOC", 1701729619);
            w.tag_zero(b"MINR");
            w.tag_str(b"NOOK", "US,CA,MX");
            w.tag_u16(b"PORT", 80);
            w.tag_u16(b"SDLY", 15000);
            w.tag_str(b"SESS", "p56Xl1+oOxD");
            w.tag_str(b"SKEY", "^�×�������Џ���������������×�������������̙Ʀٰ�ʑ��ؗ��ɓ��ܹ�ȝ��������Ǯ������������͜������۪Ӕ��");
            w.tag_u16(b"SPCT", 75);
            w.tag_str(b"STIM", "Default");
            w.tag_str(b"SVNM", "telemetry-3-common");
        });
        w.group(b"TICK", |w| {
            w.tag_str(b"ADRS", "10.23.15.2");
            w.tag_u16(b"PORT", 8999);
            w.tag_str(
                b"SKEY",
                "1011786733,10.23.15.2:8999,mirrorsedgecatalyst-2016-pc,10,50,50,50,50,0,12",
            );
        });
        w.group(b"UROP", |w| {
            w.tag_zero(b"TMOP");
            w.tag_u32(b"UID", self.user.persona_id)
        });
    }
}
