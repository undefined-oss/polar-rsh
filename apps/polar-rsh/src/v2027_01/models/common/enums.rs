use serde::{Deserialize, Serialize};
/// The permission level to grant. Read more about roles and their permissions on [GitHub documentation](https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization#permissions-for-each-role).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum BenefitGitHubRepositoryPropertiesPermission {
    #[serde(rename = "pull")]
    Pull,
    #[serde(rename = "triage")]
    Triage,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "maintain")]
    Maintain,
    #[serde(rename = "admin")]
    Admin,
}
impl BenefitGitHubRepositoryPropertiesPermission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pull => "pull",
            Self::Triage => "triage",
            Self::Push => "push",
            Self::Maintain => "maintain",
            Self::Admin => "admin",
        }
    }
}
impl ::std::fmt::Display for BenefitGitHubRepositoryPropertiesPermission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for BenefitGitHubRepositoryPropertiesPermission {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum BenefitGrantGitHubRepositoryPropertiesPermission {
    #[serde(rename = "pull")]
    Pull,
    #[serde(rename = "triage")]
    Triage,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "maintain")]
    Maintain,
    #[serde(rename = "admin")]
    Admin,
}
impl BenefitGrantGitHubRepositoryPropertiesPermission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pull => "pull",
            Self::Triage => "triage",
            Self::Push => "push",
            Self::Maintain => "maintain",
            Self::Admin => "admin",
        }
    }
}
impl ::std::fmt::Display for BenefitGrantGitHubRepositoryPropertiesPermission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for BenefitGrantGitHubRepositoryPropertiesPermission {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum BenefitLicenseKeyExpirationPropertiesTimeframe {
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "day")]
    Day,
}
impl BenefitLicenseKeyExpirationPropertiesTimeframe {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Year => "year",
            Self::Month => "month",
            Self::Day => "day",
        }
    }
}
impl ::std::fmt::Display for BenefitLicenseKeyExpirationPropertiesTimeframe {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for BenefitLicenseKeyExpirationPropertiesTimeframe {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum BenefitType {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "github_repository")]
    GithubRepository,
    #[serde(rename = "downloadables")]
    Downloadables,
    #[serde(rename = "license_keys")]
    LicenseKeys,
    #[serde(rename = "meter_credit")]
    MeterCredit,
    #[serde(rename = "feature_flag")]
    FeatureFlag,
    #[serde(rename = "slack_shared_channel")]
    SlackSharedChannel,
}
impl BenefitType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Custom => "custom",
            Self::Discord => "discord",
            Self::GithubRepository => "github_repository",
            Self::Downloadables => "downloadables",
            Self::LicenseKeys => "license_keys",
            Self::MeterCredit => "meter_credit",
            Self::FeatureFlag => "feature_flag",
            Self::SlackSharedChannel => "slack_shared_channel",
        }
    }
}
impl ::std::fmt::Display for BenefitType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for BenefitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum BenefitVisibility {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}
impl BenefitVisibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Private => "private",
            Self::Public => "public",
        }
    }
}
impl ::std::fmt::Display for BenefitVisibility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for BenefitVisibility {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CountryAlpha2 {
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AS")]
    As,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CC")]
    Cc,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CU")]
    Cu,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CX")]
    Cx,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FM")]
    Fm,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HM")]
    Hm,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IR")]
    Ir,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KP")]
    Kp,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MH")]
    Mh,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MP")]
    Mp,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NF")]
    Nf,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PW")]
    Pw,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SD")]
    Sd,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SY")]
    Sy,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "UM")]
    Um,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VI")]
    Vi,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
}
impl CountryAlpha2 {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ad => "AD",
            Self::Ae => "AE",
            Self::Af => "AF",
            Self::Ag => "AG",
            Self::Ai => "AI",
            Self::Al => "AL",
            Self::Am => "AM",
            Self::Ao => "AO",
            Self::Aq => "AQ",
            Self::Ar => "AR",
            Self::As => "AS",
            Self::At => "AT",
            Self::Au => "AU",
            Self::Aw => "AW",
            Self::Ax => "AX",
            Self::Az => "AZ",
            Self::Ba => "BA",
            Self::Bb => "BB",
            Self::Bd => "BD",
            Self::Be => "BE",
            Self::Bf => "BF",
            Self::Bg => "BG",
            Self::Bh => "BH",
            Self::Bi => "BI",
            Self::Bj => "BJ",
            Self::Bl => "BL",
            Self::Bm => "BM",
            Self::Bn => "BN",
            Self::Bo => "BO",
            Self::Bq => "BQ",
            Self::Br => "BR",
            Self::Bs => "BS",
            Self::Bt => "BT",
            Self::Bv => "BV",
            Self::Bw => "BW",
            Self::By => "BY",
            Self::Bz => "BZ",
            Self::Ca => "CA",
            Self::Cc => "CC",
            Self::Cd => "CD",
            Self::Cf => "CF",
            Self::Cg => "CG",
            Self::Ch => "CH",
            Self::Ci => "CI",
            Self::Ck => "CK",
            Self::Cl => "CL",
            Self::Cm => "CM",
            Self::Cn => "CN",
            Self::Co => "CO",
            Self::Cr => "CR",
            Self::Cu => "CU",
            Self::Cv => "CV",
            Self::Cw => "CW",
            Self::Cx => "CX",
            Self::Cy => "CY",
            Self::Cz => "CZ",
            Self::De => "DE",
            Self::Dj => "DJ",
            Self::Dk => "DK",
            Self::Dm => "DM",
            Self::Do => "DO",
            Self::Dz => "DZ",
            Self::Ec => "EC",
            Self::Ee => "EE",
            Self::Eg => "EG",
            Self::Eh => "EH",
            Self::Er => "ER",
            Self::Es => "ES",
            Self::Et => "ET",
            Self::Fi => "FI",
            Self::Fj => "FJ",
            Self::Fk => "FK",
            Self::Fm => "FM",
            Self::Fo => "FO",
            Self::Fr => "FR",
            Self::Ga => "GA",
            Self::Gb => "GB",
            Self::Gd => "GD",
            Self::Ge => "GE",
            Self::Gf => "GF",
            Self::Gg => "GG",
            Self::Gh => "GH",
            Self::Gi => "GI",
            Self::Gl => "GL",
            Self::Gm => "GM",
            Self::Gn => "GN",
            Self::Gp => "GP",
            Self::Gq => "GQ",
            Self::Gr => "GR",
            Self::Gs => "GS",
            Self::Gt => "GT",
            Self::Gu => "GU",
            Self::Gw => "GW",
            Self::Gy => "GY",
            Self::Hk => "HK",
            Self::Hm => "HM",
            Self::Hn => "HN",
            Self::Hr => "HR",
            Self::Ht => "HT",
            Self::Hu => "HU",
            Self::Id => "ID",
            Self::Ie => "IE",
            Self::Il => "IL",
            Self::Im => "IM",
            Self::In => "IN",
            Self::Io => "IO",
            Self::Iq => "IQ",
            Self::Ir => "IR",
            Self::Is => "IS",
            Self::It => "IT",
            Self::Je => "JE",
            Self::Jm => "JM",
            Self::Jo => "JO",
            Self::Jp => "JP",
            Self::Ke => "KE",
            Self::Kg => "KG",
            Self::Kh => "KH",
            Self::Ki => "KI",
            Self::Km => "KM",
            Self::Kn => "KN",
            Self::Kp => "KP",
            Self::Kr => "KR",
            Self::Kw => "KW",
            Self::Ky => "KY",
            Self::Kz => "KZ",
            Self::La => "LA",
            Self::Lb => "LB",
            Self::Lc => "LC",
            Self::Li => "LI",
            Self::Lk => "LK",
            Self::Lr => "LR",
            Self::Ls => "LS",
            Self::Lt => "LT",
            Self::Lu => "LU",
            Self::Lv => "LV",
            Self::Ly => "LY",
            Self::Ma => "MA",
            Self::Mc => "MC",
            Self::Md => "MD",
            Self::Me => "ME",
            Self::Mf => "MF",
            Self::Mg => "MG",
            Self::Mh => "MH",
            Self::Mk => "MK",
            Self::Ml => "ML",
            Self::Mm => "MM",
            Self::Mn => "MN",
            Self::Mo => "MO",
            Self::Mp => "MP",
            Self::Mq => "MQ",
            Self::Mr => "MR",
            Self::Ms => "MS",
            Self::Mt => "MT",
            Self::Mu => "MU",
            Self::Mv => "MV",
            Self::Mw => "MW",
            Self::Mx => "MX",
            Self::My => "MY",
            Self::Mz => "MZ",
            Self::Na => "NA",
            Self::Nc => "NC",
            Self::Ne => "NE",
            Self::Nf => "NF",
            Self::Ng => "NG",
            Self::Ni => "NI",
            Self::Nl => "NL",
            Self::No => "NO",
            Self::Np => "NP",
            Self::Nr => "NR",
            Self::Nu => "NU",
            Self::Nz => "NZ",
            Self::Om => "OM",
            Self::Pa => "PA",
            Self::Pe => "PE",
            Self::Pf => "PF",
            Self::Pg => "PG",
            Self::Ph => "PH",
            Self::Pk => "PK",
            Self::Pl => "PL",
            Self::Pm => "PM",
            Self::Pn => "PN",
            Self::Pr => "PR",
            Self::Ps => "PS",
            Self::Pt => "PT",
            Self::Pw => "PW",
            Self::Py => "PY",
            Self::Qa => "QA",
            Self::Re => "RE",
            Self::Ro => "RO",
            Self::Rs => "RS",
            Self::Ru => "RU",
            Self::Rw => "RW",
            Self::Sa => "SA",
            Self::Sb => "SB",
            Self::Sc => "SC",
            Self::Sd => "SD",
            Self::Se => "SE",
            Self::Sg => "SG",
            Self::Sh => "SH",
            Self::Si => "SI",
            Self::Sj => "SJ",
            Self::Sk => "SK",
            Self::Sl => "SL",
            Self::Sm => "SM",
            Self::Sn => "SN",
            Self::So => "SO",
            Self::Sr => "SR",
            Self::Ss => "SS",
            Self::St => "ST",
            Self::Sv => "SV",
            Self::Sx => "SX",
            Self::Sy => "SY",
            Self::Sz => "SZ",
            Self::Tc => "TC",
            Self::Td => "TD",
            Self::Tf => "TF",
            Self::Tg => "TG",
            Self::Th => "TH",
            Self::Tj => "TJ",
            Self::Tk => "TK",
            Self::Tl => "TL",
            Self::Tm => "TM",
            Self::Tn => "TN",
            Self::To => "TO",
            Self::Tr => "TR",
            Self::Tt => "TT",
            Self::Tv => "TV",
            Self::Tw => "TW",
            Self::Tz => "TZ",
            Self::Ua => "UA",
            Self::Ug => "UG",
            Self::Um => "UM",
            Self::Us => "US",
            Self::Uy => "UY",
            Self::Uz => "UZ",
            Self::Va => "VA",
            Self::Vc => "VC",
            Self::Ve => "VE",
            Self::Vg => "VG",
            Self::Vi => "VI",
            Self::Vn => "VN",
            Self::Vu => "VU",
            Self::Wf => "WF",
            Self::Ws => "WS",
            Self::Ye => "YE",
            Self::Yt => "YT",
            Self::Za => "ZA",
            Self::Zm => "ZM",
            Self::Zw => "ZW",
        }
    }
}
impl ::std::fmt::Display for CountryAlpha2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CountryAlpha2 {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CountryAlpha2Input {
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AS")]
    As,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CC")]
    Cc,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CX")]
    Cx,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FM")]
    Fm,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HM")]
    Hm,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MH")]
    Mh,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MP")]
    Mp,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NF")]
    Nf,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PW")]
    Pw,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SD")]
    Sd,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "UM")]
    Um,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VI")]
    Vi,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
}
impl CountryAlpha2Input {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ad => "AD",
            Self::Ae => "AE",
            Self::Af => "AF",
            Self::Ag => "AG",
            Self::Ai => "AI",
            Self::Al => "AL",
            Self::Am => "AM",
            Self::Ao => "AO",
            Self::Aq => "AQ",
            Self::Ar => "AR",
            Self::As => "AS",
            Self::At => "AT",
            Self::Au => "AU",
            Self::Aw => "AW",
            Self::Ax => "AX",
            Self::Az => "AZ",
            Self::Ba => "BA",
            Self::Bb => "BB",
            Self::Bd => "BD",
            Self::Be => "BE",
            Self::Bf => "BF",
            Self::Bg => "BG",
            Self::Bh => "BH",
            Self::Bi => "BI",
            Self::Bj => "BJ",
            Self::Bl => "BL",
            Self::Bm => "BM",
            Self::Bn => "BN",
            Self::Bo => "BO",
            Self::Bq => "BQ",
            Self::Br => "BR",
            Self::Bs => "BS",
            Self::Bt => "BT",
            Self::Bv => "BV",
            Self::Bw => "BW",
            Self::By => "BY",
            Self::Bz => "BZ",
            Self::Ca => "CA",
            Self::Cc => "CC",
            Self::Cd => "CD",
            Self::Cf => "CF",
            Self::Cg => "CG",
            Self::Ch => "CH",
            Self::Ci => "CI",
            Self::Ck => "CK",
            Self::Cl => "CL",
            Self::Cm => "CM",
            Self::Cn => "CN",
            Self::Co => "CO",
            Self::Cr => "CR",
            Self::Cv => "CV",
            Self::Cw => "CW",
            Self::Cx => "CX",
            Self::Cy => "CY",
            Self::Cz => "CZ",
            Self::De => "DE",
            Self::Dj => "DJ",
            Self::Dk => "DK",
            Self::Dm => "DM",
            Self::Do => "DO",
            Self::Dz => "DZ",
            Self::Ec => "EC",
            Self::Ee => "EE",
            Self::Eg => "EG",
            Self::Eh => "EH",
            Self::Er => "ER",
            Self::Es => "ES",
            Self::Et => "ET",
            Self::Fi => "FI",
            Self::Fj => "FJ",
            Self::Fk => "FK",
            Self::Fm => "FM",
            Self::Fo => "FO",
            Self::Fr => "FR",
            Self::Ga => "GA",
            Self::Gb => "GB",
            Self::Gd => "GD",
            Self::Ge => "GE",
            Self::Gf => "GF",
            Self::Gg => "GG",
            Self::Gh => "GH",
            Self::Gi => "GI",
            Self::Gl => "GL",
            Self::Gm => "GM",
            Self::Gn => "GN",
            Self::Gp => "GP",
            Self::Gq => "GQ",
            Self::Gr => "GR",
            Self::Gs => "GS",
            Self::Gt => "GT",
            Self::Gu => "GU",
            Self::Gw => "GW",
            Self::Gy => "GY",
            Self::Hk => "HK",
            Self::Hm => "HM",
            Self::Hn => "HN",
            Self::Hr => "HR",
            Self::Ht => "HT",
            Self::Hu => "HU",
            Self::Id => "ID",
            Self::Ie => "IE",
            Self::Il => "IL",
            Self::Im => "IM",
            Self::In => "IN",
            Self::Io => "IO",
            Self::Iq => "IQ",
            Self::Is => "IS",
            Self::It => "IT",
            Self::Je => "JE",
            Self::Jm => "JM",
            Self::Jo => "JO",
            Self::Jp => "JP",
            Self::Ke => "KE",
            Self::Kg => "KG",
            Self::Kh => "KH",
            Self::Ki => "KI",
            Self::Km => "KM",
            Self::Kn => "KN",
            Self::Kr => "KR",
            Self::Kw => "KW",
            Self::Ky => "KY",
            Self::Kz => "KZ",
            Self::La => "LA",
            Self::Lb => "LB",
            Self::Lc => "LC",
            Self::Li => "LI",
            Self::Lk => "LK",
            Self::Lr => "LR",
            Self::Ls => "LS",
            Self::Lt => "LT",
            Self::Lu => "LU",
            Self::Lv => "LV",
            Self::Ly => "LY",
            Self::Ma => "MA",
            Self::Mc => "MC",
            Self::Md => "MD",
            Self::Me => "ME",
            Self::Mf => "MF",
            Self::Mg => "MG",
            Self::Mh => "MH",
            Self::Mk => "MK",
            Self::Ml => "ML",
            Self::Mm => "MM",
            Self::Mn => "MN",
            Self::Mo => "MO",
            Self::Mp => "MP",
            Self::Mq => "MQ",
            Self::Mr => "MR",
            Self::Ms => "MS",
            Self::Mt => "MT",
            Self::Mu => "MU",
            Self::Mv => "MV",
            Self::Mw => "MW",
            Self::Mx => "MX",
            Self::My => "MY",
            Self::Mz => "MZ",
            Self::Na => "NA",
            Self::Nc => "NC",
            Self::Ne => "NE",
            Self::Nf => "NF",
            Self::Ng => "NG",
            Self::Ni => "NI",
            Self::Nl => "NL",
            Self::No => "NO",
            Self::Np => "NP",
            Self::Nr => "NR",
            Self::Nu => "NU",
            Self::Nz => "NZ",
            Self::Om => "OM",
            Self::Pa => "PA",
            Self::Pe => "PE",
            Self::Pf => "PF",
            Self::Pg => "PG",
            Self::Ph => "PH",
            Self::Pk => "PK",
            Self::Pl => "PL",
            Self::Pm => "PM",
            Self::Pn => "PN",
            Self::Pr => "PR",
            Self::Ps => "PS",
            Self::Pt => "PT",
            Self::Pw => "PW",
            Self::Py => "PY",
            Self::Qa => "QA",
            Self::Re => "RE",
            Self::Ro => "RO",
            Self::Rs => "RS",
            Self::Rw => "RW",
            Self::Sa => "SA",
            Self::Sb => "SB",
            Self::Sc => "SC",
            Self::Sd => "SD",
            Self::Se => "SE",
            Self::Sg => "SG",
            Self::Sh => "SH",
            Self::Si => "SI",
            Self::Sj => "SJ",
            Self::Sk => "SK",
            Self::Sl => "SL",
            Self::Sm => "SM",
            Self::Sn => "SN",
            Self::So => "SO",
            Self::Sr => "SR",
            Self::Ss => "SS",
            Self::St => "ST",
            Self::Sv => "SV",
            Self::Sx => "SX",
            Self::Sz => "SZ",
            Self::Tc => "TC",
            Self::Td => "TD",
            Self::Tf => "TF",
            Self::Tg => "TG",
            Self::Th => "TH",
            Self::Tj => "TJ",
            Self::Tk => "TK",
            Self::Tl => "TL",
            Self::Tm => "TM",
            Self::Tn => "TN",
            Self::To => "TO",
            Self::Tr => "TR",
            Self::Tt => "TT",
            Self::Tv => "TV",
            Self::Tw => "TW",
            Self::Tz => "TZ",
            Self::Ua => "UA",
            Self::Ug => "UG",
            Self::Um => "UM",
            Self::Us => "US",
            Self::Uy => "UY",
            Self::Uz => "UZ",
            Self::Va => "VA",
            Self::Vc => "VC",
            Self::Ve => "VE",
            Self::Vg => "VG",
            Self::Vi => "VI",
            Self::Vn => "VN",
            Self::Vu => "VU",
            Self::Wf => "WF",
            Self::Ws => "WS",
            Self::Ye => "YE",
            Self::Yt => "YT",
            Self::Za => "ZA",
            Self::Zm => "ZM",
            Self::Zw => "ZW",
        }
    }
}
impl ::std::fmt::Display for CountryAlpha2Input {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CountryAlpha2Input {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomerCancellationReason {
    #[serde(rename = "customer_service")]
    CustomerService,
    #[serde(rename = "low_quality")]
    LowQuality,
    #[serde(rename = "missing_features")]
    MissingFeatures,
    #[serde(rename = "switched_service")]
    SwitchedService,
    #[serde(rename = "too_complex")]
    TooComplex,
    #[serde(rename = "too_expensive")]
    TooExpensive,
    #[serde(rename = "unused")]
    Unused,
    #[serde(rename = "other")]
    Other,
}
impl CustomerCancellationReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CustomerService => "customer_service",
            Self::LowQuality => "low_quality",
            Self::MissingFeatures => "missing_features",
            Self::SwitchedService => "switched_service",
            Self::TooComplex => "too_complex",
            Self::TooExpensive => "too_expensive",
            Self::Unused => "unused",
            Self::Other => "other",
        }
    }
}
impl ::std::fmt::Display for CustomerCancellationReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomerCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomerType {
    #[serde(rename = "individual")]
    Individual,
    #[serde(rename = "team")]
    Team,
}
impl CustomerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Individual => "individual",
            Self::Team => "team",
        }
    }
}
impl ::std::fmt::Display for CustomerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum DiscountDuration {
    #[serde(rename = "once")]
    Once,
    #[serde(rename = "forever")]
    Forever,
    #[serde(rename = "repeating")]
    Repeating,
}
impl DiscountDuration {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Once => "once",
            Self::Forever => "forever",
            Self::Repeating => "repeating",
        }
    }
}
impl ::std::fmt::Display for DiscountDuration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for DiscountDuration {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum DiscountType {
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "percentage")]
    Percentage,
}
impl DiscountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Percentage => "percentage",
        }
    }
}
impl ::std::fmt::Display for DiscountType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for DiscountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum DisputeStatus {
    #[serde(rename = "prevented")]
    Prevented,
    #[serde(rename = "early_warning")]
    EarlyWarning,
    #[serde(rename = "needs_response")]
    NeedsResponse,
    #[serde(rename = "under_review")]
    UnderReview,
    #[serde(rename = "lost")]
    Lost,
    #[serde(rename = "won")]
    Won,
}
impl DisputeStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Prevented => "prevented",
            Self::EarlyWarning => "early_warning",
            Self::NeedsResponse => "needs_response",
            Self::UnderReview => "under_review",
            Self::Lost => "lost",
            Self::Won => "won",
        }
    }
}
impl ::std::fmt::Display for DisputeStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for DisputeStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum EventSource {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
}
impl EventSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
        }
    }
}
impl ::std::fmt::Display for EventSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for EventSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum FileServiceTypes {
    #[serde(rename = "downloadable")]
    Downloadable,
    #[serde(rename = "product_media")]
    ProductMedia,
    #[serde(rename = "organization_avatar")]
    OrganizationAvatar,
    #[serde(rename = "support_case_attachment")]
    SupportCaseAttachment,
}
impl FileServiceTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Downloadable => "downloadable",
            Self::ProductMedia => "product_media",
            Self::OrganizationAvatar => "organization_avatar",
            Self::SupportCaseAttachment => "support_case_attachment",
        }
    }
}
impl ::std::fmt::Display for FileServiceTypes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for FileServiceTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum FilterConjunction {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
}
impl FilterConjunction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::And => "and",
            Self::Or => "or",
        }
    }
}
impl ::std::fmt::Display for FilterConjunction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for FilterConjunction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum FilterOperator {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "gte")]
    Gte,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lte")]
    Lte,
    #[serde(rename = "like")]
    Like,
    #[serde(rename = "not_like")]
    NotLike,
}
impl FilterOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eq => "eq",
            Self::Ne => "ne",
            Self::Gt => "gt",
            Self::Gte => "gte",
            Self::Lt => "lt",
            Self::Lte => "lte",
            Self::Like => "like",
            Self::NotLike => "not_like",
        }
    }
}
impl ::std::fmt::Display for FilterOperator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for FilterOperator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum MemberRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "billing_manager")]
    BillingManager,
    #[serde(rename = "member")]
    Member,
}
impl MemberRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::BillingManager => "billing_manager",
            Self::Member => "member",
        }
    }
}
impl ::std::fmt::Display for MemberRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for MemberRole {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Default,
)]
pub enum MeterCreditedMetadataReason {
    #[default]
    #[serde(rename = "benefit")]
    Benefit,
    #[serde(rename = "manual")]
    Manual,
}
impl MeterCreditedMetadataReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Benefit => "benefit",
            Self::Manual => "manual",
        }
    }
}
impl ::std::fmt::Display for MeterCreditedMetadataReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for MeterCreditedMetadataReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum MeterUnit {
    #[serde(rename = "scalar")]
    Scalar,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "custom")]
    Custom,
}
impl MeterUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Token => "token",
            Self::Custom => "custom",
        }
    }
}
impl ::std::fmt::Display for MeterUnit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for MeterUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
/// Internal billing reasons with additional granularity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OrderBillingReasonInternal {
    #[serde(rename = "purchase")]
    Purchase,
    #[serde(rename = "subscription_create")]
    SubscriptionCreate,
    #[serde(rename = "subscription_cycle")]
    SubscriptionCycle,
    #[serde(rename = "subscription_cycle_after_trial")]
    SubscriptionCycleAfterTrial,
    #[serde(rename = "subscription_cancel")]
    SubscriptionCancel,
    #[serde(rename = "subscription_update")]
    SubscriptionUpdate,
    #[serde(rename = "subscription_meter_cycle")]
    SubscriptionMeterCycle,
}
impl OrderBillingReasonInternal {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Purchase => "purchase",
            Self::SubscriptionCreate => "subscription_create",
            Self::SubscriptionCycle => "subscription_cycle",
            Self::SubscriptionCycleAfterTrial => "subscription_cycle_after_trial",
            Self::SubscriptionCancel => "subscription_cancel",
            Self::SubscriptionUpdate => "subscription_update",
            Self::SubscriptionMeterCycle => "subscription_meter_cycle",
        }
    }
}
impl ::std::fmt::Display for OrderBillingReasonInternal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OrderBillingReasonInternal {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum OrderStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "refunded")]
    Refunded,
    #[serde(rename = "partially_refunded")]
    PartiallyRefunded,
    #[serde(rename = "void")]
    Void,
}
impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Pending => "pending",
            Self::Paid => "paid",
            Self::Refunded => "refunded",
            Self::PartiallyRefunded => "partially_refunded",
            Self::Void => "void",
        }
    }
}
impl ::std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for OrderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum PaymentProcessor {
    #[serde(rename = "stripe")]
    Stripe,
}
impl PaymentProcessor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stripe => "stripe",
        }
    }
}
impl ::std::fmt::Display for PaymentProcessor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for PaymentProcessor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum PresentmentCurrency {
    #[serde(rename = "aed")]
    Aed,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "amd")]
    Amd,
    #[serde(rename = "aoa")]
    Aoa,
    #[serde(rename = "ars")]
    Ars,
    #[serde(rename = "aud")]
    Aud,
    #[serde(rename = "awg")]
    Awg,
    #[serde(rename = "azn")]
    Azn,
    #[serde(rename = "bam")]
    Bam,
    #[serde(rename = "bbd")]
    Bbd,
    #[serde(rename = "bdt")]
    Bdt,
    #[serde(rename = "bif")]
    Bif,
    #[serde(rename = "bmd")]
    Bmd,
    #[serde(rename = "bnd")]
    Bnd,
    #[serde(rename = "bob")]
    Bob,
    #[serde(rename = "brl")]
    Brl,
    #[serde(rename = "bsd")]
    Bsd,
    #[serde(rename = "bwp")]
    Bwp,
    #[serde(rename = "bzd")]
    Bzd,
    #[serde(rename = "cad")]
    Cad,
    #[serde(rename = "cdf")]
    Cdf,
    #[serde(rename = "chf")]
    Chf,
    #[serde(rename = "clp")]
    Clp,
    #[serde(rename = "cny")]
    Cny,
    #[serde(rename = "cop")]
    Cop,
    #[serde(rename = "crc")]
    Crc,
    #[serde(rename = "cve")]
    Cve,
    #[serde(rename = "czk")]
    Czk,
    #[serde(rename = "djf")]
    Djf,
    #[serde(rename = "dkk")]
    Dkk,
    #[serde(rename = "dop")]
    Dop,
    #[serde(rename = "dzd")]
    Dzd,
    #[serde(rename = "egp")]
    Egp,
    #[serde(rename = "etb")]
    Etb,
    #[serde(rename = "eur")]
    Eur,
    #[serde(rename = "fjd")]
    Fjd,
    #[serde(rename = "fkp")]
    Fkp,
    #[serde(rename = "gbp")]
    Gbp,
    #[serde(rename = "gel")]
    Gel,
    #[serde(rename = "gip")]
    Gip,
    #[serde(rename = "gmd")]
    Gmd,
    #[serde(rename = "gnf")]
    Gnf,
    #[serde(rename = "gtq")]
    Gtq,
    #[serde(rename = "gyd")]
    Gyd,
    #[serde(rename = "hkd")]
    Hkd,
    #[serde(rename = "hnl")]
    Hnl,
    #[serde(rename = "htg")]
    Htg,
    #[serde(rename = "huf")]
    Huf,
    #[serde(rename = "idr")]
    Idr,
    #[serde(rename = "ils")]
    Ils,
    #[serde(rename = "inr")]
    Inr,
    #[serde(rename = "isk")]
    Isk,
    #[serde(rename = "jmd")]
    Jmd,
    #[serde(rename = "jpy")]
    Jpy,
    #[serde(rename = "kes")]
    Kes,
    #[serde(rename = "kgs")]
    Kgs,
    #[serde(rename = "khr")]
    Khr,
    #[serde(rename = "kmf")]
    Kmf,
    #[serde(rename = "krw")]
    Krw,
    #[serde(rename = "kyd")]
    Kyd,
    #[serde(rename = "kzt")]
    Kzt,
    #[serde(rename = "lak")]
    Lak,
    #[serde(rename = "lkr")]
    Lkr,
    #[serde(rename = "lrd")]
    Lrd,
    #[serde(rename = "lsl")]
    Lsl,
    #[serde(rename = "mad")]
    Mad,
    #[serde(rename = "mdl")]
    Mdl,
    #[serde(rename = "mga")]
    Mga,
    #[serde(rename = "mkd")]
    Mkd,
    #[serde(rename = "mnt")]
    Mnt,
    #[serde(rename = "mop")]
    Mop,
    #[serde(rename = "mur")]
    Mur,
    #[serde(rename = "mvr")]
    Mvr,
    #[serde(rename = "mwk")]
    Mwk,
    #[serde(rename = "mxn")]
    Mxn,
    #[serde(rename = "myr")]
    Myr,
    #[serde(rename = "mzn")]
    Mzn,
    #[serde(rename = "nad")]
    Nad,
    #[serde(rename = "ngn")]
    Ngn,
    #[serde(rename = "nio")]
    Nio,
    #[serde(rename = "nok")]
    Nok,
    #[serde(rename = "npr")]
    Npr,
    #[serde(rename = "nzd")]
    Nzd,
    #[serde(rename = "pab")]
    Pab,
    #[serde(rename = "pen")]
    Pen,
    #[serde(rename = "pgk")]
    Pgk,
    #[serde(rename = "php")]
    Php,
    #[serde(rename = "pkr")]
    Pkr,
    #[serde(rename = "pln")]
    Pln,
    #[serde(rename = "pyg")]
    Pyg,
    #[serde(rename = "qar")]
    Qar,
    #[serde(rename = "ron")]
    Ron,
    #[serde(rename = "rsd")]
    Rsd,
    #[serde(rename = "rwf")]
    Rwf,
    #[serde(rename = "sar")]
    Sar,
    #[serde(rename = "sbd")]
    Sbd,
    #[serde(rename = "scr")]
    Scr,
    #[serde(rename = "sek")]
    Sek,
    #[serde(rename = "sgd")]
    Sgd,
    #[serde(rename = "shp")]
    Shp,
    #[serde(rename = "sos")]
    Sos,
    #[serde(rename = "srd")]
    Srd,
    #[serde(rename = "szl")]
    Szl,
    #[serde(rename = "thb")]
    Thb,
    #[serde(rename = "tjs")]
    Tjs,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "try")]
    Try,
    #[serde(rename = "ttd")]
    Ttd,
    #[serde(rename = "twd")]
    Twd,
    #[serde(rename = "tzs")]
    Tzs,
    #[serde(rename = "uah")]
    Uah,
    #[serde(rename = "ugx")]
    Ugx,
    #[serde(rename = "usd")]
    Usd,
    #[serde(rename = "uyu")]
    Uyu,
    #[serde(rename = "uzs")]
    Uzs,
    #[serde(rename = "vnd")]
    Vnd,
    #[serde(rename = "vuv")]
    Vuv,
    #[serde(rename = "wst")]
    Wst,
    #[serde(rename = "xaf")]
    Xaf,
    #[serde(rename = "xcd")]
    Xcd,
    #[serde(rename = "xcg")]
    Xcg,
    #[serde(rename = "xof")]
    Xof,
    #[serde(rename = "xpf")]
    Xpf,
    #[serde(rename = "yer")]
    Yer,
    #[serde(rename = "zar")]
    Zar,
    #[serde(rename = "zmw")]
    Zmw,
}
impl PresentmentCurrency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Aed => "aed",
            Self::All => "all",
            Self::Amd => "amd",
            Self::Aoa => "aoa",
            Self::Ars => "ars",
            Self::Aud => "aud",
            Self::Awg => "awg",
            Self::Azn => "azn",
            Self::Bam => "bam",
            Self::Bbd => "bbd",
            Self::Bdt => "bdt",
            Self::Bif => "bif",
            Self::Bmd => "bmd",
            Self::Bnd => "bnd",
            Self::Bob => "bob",
            Self::Brl => "brl",
            Self::Bsd => "bsd",
            Self::Bwp => "bwp",
            Self::Bzd => "bzd",
            Self::Cad => "cad",
            Self::Cdf => "cdf",
            Self::Chf => "chf",
            Self::Clp => "clp",
            Self::Cny => "cny",
            Self::Cop => "cop",
            Self::Crc => "crc",
            Self::Cve => "cve",
            Self::Czk => "czk",
            Self::Djf => "djf",
            Self::Dkk => "dkk",
            Self::Dop => "dop",
            Self::Dzd => "dzd",
            Self::Egp => "egp",
            Self::Etb => "etb",
            Self::Eur => "eur",
            Self::Fjd => "fjd",
            Self::Fkp => "fkp",
            Self::Gbp => "gbp",
            Self::Gel => "gel",
            Self::Gip => "gip",
            Self::Gmd => "gmd",
            Self::Gnf => "gnf",
            Self::Gtq => "gtq",
            Self::Gyd => "gyd",
            Self::Hkd => "hkd",
            Self::Hnl => "hnl",
            Self::Htg => "htg",
            Self::Huf => "huf",
            Self::Idr => "idr",
            Self::Ils => "ils",
            Self::Inr => "inr",
            Self::Isk => "isk",
            Self::Jmd => "jmd",
            Self::Jpy => "jpy",
            Self::Kes => "kes",
            Self::Kgs => "kgs",
            Self::Khr => "khr",
            Self::Kmf => "kmf",
            Self::Krw => "krw",
            Self::Kyd => "kyd",
            Self::Kzt => "kzt",
            Self::Lak => "lak",
            Self::Lkr => "lkr",
            Self::Lrd => "lrd",
            Self::Lsl => "lsl",
            Self::Mad => "mad",
            Self::Mdl => "mdl",
            Self::Mga => "mga",
            Self::Mkd => "mkd",
            Self::Mnt => "mnt",
            Self::Mop => "mop",
            Self::Mur => "mur",
            Self::Mvr => "mvr",
            Self::Mwk => "mwk",
            Self::Mxn => "mxn",
            Self::Myr => "myr",
            Self::Mzn => "mzn",
            Self::Nad => "nad",
            Self::Ngn => "ngn",
            Self::Nio => "nio",
            Self::Nok => "nok",
            Self::Npr => "npr",
            Self::Nzd => "nzd",
            Self::Pab => "pab",
            Self::Pen => "pen",
            Self::Pgk => "pgk",
            Self::Php => "php",
            Self::Pkr => "pkr",
            Self::Pln => "pln",
            Self::Pyg => "pyg",
            Self::Qar => "qar",
            Self::Ron => "ron",
            Self::Rsd => "rsd",
            Self::Rwf => "rwf",
            Self::Sar => "sar",
            Self::Sbd => "sbd",
            Self::Scr => "scr",
            Self::Sek => "sek",
            Self::Sgd => "sgd",
            Self::Shp => "shp",
            Self::Sos => "sos",
            Self::Srd => "srd",
            Self::Szl => "szl",
            Self::Thb => "thb",
            Self::Tjs => "tjs",
            Self::Top => "top",
            Self::Try => "try",
            Self::Ttd => "ttd",
            Self::Twd => "twd",
            Self::Tzs => "tzs",
            Self::Uah => "uah",
            Self::Ugx => "ugx",
            Self::Usd => "usd",
            Self::Uyu => "uyu",
            Self::Uzs => "uzs",
            Self::Vnd => "vnd",
            Self::Vuv => "vuv",
            Self::Wst => "wst",
            Self::Xaf => "xaf",
            Self::Xcd => "xcd",
            Self::Xcg => "xcg",
            Self::Xof => "xof",
            Self::Xpf => "xpf",
            Self::Yer => "yer",
            Self::Zar => "zar",
            Self::Zmw => "zmw",
        }
    }
}
impl ::std::fmt::Display for PresentmentCurrency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for PresentmentCurrency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum ProductBillingType {
    #[serde(rename = "one_time")]
    OneTime,
    #[serde(rename = "recurring")]
    Recurring,
}
impl ProductBillingType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneTime => "one_time",
            Self::Recurring => "recurring",
        }
    }
}
impl ::std::fmt::Display for ProductBillingType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for ProductBillingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum ProductPriceSource {
    #[serde(rename = "catalog")]
    Catalog,
    #[serde(rename = "ad_hoc")]
    AdHoc,
}
impl ProductPriceSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Catalog => "catalog",
            Self::AdHoc => "ad_hoc",
        }
    }
}
impl ::std::fmt::Display for ProductPriceSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for ProductPriceSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum ProductVisibility {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}
impl ProductVisibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Private => "private",
            Self::Public => "public",
        }
    }
}
impl ::std::fmt::Display for ProductVisibility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for ProductVisibility {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum PropertyAggregationFunc {
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "avg")]
    Avg,
}
impl PropertyAggregationFunc {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sum => "sum",
            Self::Max => "max",
            Self::Min => "min",
            Self::Avg => "avg",
        }
    }
}
impl ::std::fmt::Display for PropertyAggregationFunc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for PropertyAggregationFunc {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum RecurringInterval {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl RecurringInterval {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Year => "year",
        }
    }
}
impl ::std::fmt::Display for RecurringInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for RecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum SeatStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "claimed")]
    Claimed,
    #[serde(rename = "revoked")]
    Revoked,
}
impl SeatStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Claimed => "claimed",
            Self::Revoked => "revoked",
        }
    }
}
impl ::std::fmt::Display for SeatStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for SeatStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum SubscriptionProrationBehavior {
    #[serde(rename = "invoice")]
    Invoice,
    #[serde(rename = "prorate")]
    Prorate,
    #[serde(rename = "next_period")]
    NextPeriod,
    #[serde(rename = "reset")]
    Reset,
}
impl SubscriptionProrationBehavior {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
            Self::Prorate => "prorate",
            Self::NextPeriod => "next_period",
            Self::Reset => "reset",
        }
    }
}
impl ::std::fmt::Display for SubscriptionProrationBehavior {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for SubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum SubscriptionStatus {
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "incomplete_expired")]
    IncompleteExpired,
    #[serde(rename = "trialing")]
    Trialing,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "past_due")]
    PastDue,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "unpaid")]
    Unpaid,
    #[serde(rename = "paused")]
    Paused,
}
impl SubscriptionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Incomplete => "incomplete",
            Self::IncompleteExpired => "incomplete_expired",
            Self::Trialing => "trialing",
            Self::Active => "active",
            Self::PastDue => "past_due",
            Self::Canceled => "canceled",
            Self::Unpaid => "unpaid",
            Self::Paused => "paused",
        }
    }
}
impl ::std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for SubscriptionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum TaxBehaviorOption {
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "inclusive")]
    Inclusive,
    #[serde(rename = "exclusive")]
    Exclusive,
}
impl TaxBehaviorOption {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Location => "location",
            Self::Inclusive => "inclusive",
            Self::Exclusive => "exclusive",
        }
    }
}
impl ::std::fmt::Display for TaxBehaviorOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for TaxBehaviorOption {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
/// List of supported tax ID formats.
///
/// Ref: https://docs.stripe.com/billing/customer/tax-ids#supported-tax-id
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum TaxIDFormat {
    #[serde(rename = "ad_nrt")]
    AdNrt,
    #[serde(rename = "ae_trn")]
    AeTrn,
    #[serde(rename = "ar_cuit")]
    ArCuit,
    #[serde(rename = "au_abn")]
    AuAbn,
    #[serde(rename = "au_arn")]
    AuArn,
    #[serde(rename = "bg_uic")]
    BgUic,
    #[serde(rename = "bh_vat")]
    BhVat,
    #[serde(rename = "bo_tin")]
    BoTin,
    #[serde(rename = "br_cnpj")]
    BrCnpj,
    #[serde(rename = "br_cpf")]
    BrCpf,
    #[serde(rename = "ca_bn")]
    CaBn,
    #[serde(rename = "ca_gst_hst")]
    CaGstHst,
    #[serde(rename = "ca_pst_bc")]
    CaPstBc,
    #[serde(rename = "ca_pst_mb")]
    CaPstMb,
    #[serde(rename = "ca_pst_sk")]
    CaPstSk,
    #[serde(rename = "ca_qst")]
    CaQst,
    #[serde(rename = "ch_uid")]
    ChUid,
    #[serde(rename = "ch_vat")]
    ChVat,
    #[serde(rename = "cl_tin")]
    ClTin,
    #[serde(rename = "cn_tin")]
    CnTin,
    #[serde(rename = "co_nit")]
    CoNit,
    #[serde(rename = "cr_tin")]
    CrTin,
    #[serde(rename = "de_stn")]
    DeStn,
    #[serde(rename = "do_rcn")]
    DoRcn,
    #[serde(rename = "ec_ruc")]
    EcRuc,
    #[serde(rename = "eg_tin")]
    EgTin,
    #[serde(rename = "es_cif")]
    EsCif,
    #[serde(rename = "eu_oss_vat")]
    EuOssVat,
    #[serde(rename = "eu_vat")]
    EuVat,
    #[serde(rename = "gb_vat")]
    GbVat,
    #[serde(rename = "ge_vat")]
    GeVat,
    #[serde(rename = "hk_br")]
    HkBr,
    #[serde(rename = "hr_oib")]
    HrOib,
    #[serde(rename = "hu_tin")]
    HuTin,
    #[serde(rename = "id_npwp")]
    IdNpwp,
    #[serde(rename = "il_vat")]
    IlVat,
    #[serde(rename = "in_gst")]
    InGst,
    #[serde(rename = "is_vat")]
    IsVat,
    #[serde(rename = "jp_cn")]
    JpCn,
    #[serde(rename = "jp_rn")]
    JpRn,
    #[serde(rename = "jp_trn")]
    JpTrn,
    #[serde(rename = "ke_pin")]
    KePin,
    #[serde(rename = "kr_brn")]
    KrBrn,
    #[serde(rename = "kz_bin")]
    KzBin,
    #[serde(rename = "li_uid")]
    LiUid,
    #[serde(rename = "mk_vat")]
    MkVat,
    #[serde(rename = "mx_rfc")]
    MxRfc,
    #[serde(rename = "my_frp")]
    MyFrp,
    #[serde(rename = "my_itn")]
    MyItn,
    #[serde(rename = "my_sst")]
    MySst,
    #[serde(rename = "ng_tin")]
    NgTin,
    #[serde(rename = "no_vat")]
    NoVat,
    #[serde(rename = "no_voec")]
    NoVoec,
    #[serde(rename = "nz_gst")]
    NzGst,
    #[serde(rename = "om_vat")]
    OmVat,
    #[serde(rename = "pe_ruc")]
    PeRuc,
    #[serde(rename = "ph_tin")]
    PhTin,
    #[serde(rename = "ro_tin")]
    RoTin,
    #[serde(rename = "rs_pib")]
    RsPib,
    #[serde(rename = "ru_inn")]
    RuInn,
    #[serde(rename = "ru_kpp")]
    RuKpp,
    #[serde(rename = "sa_vat")]
    SaVat,
    #[serde(rename = "sg_gst")]
    SgGst,
    #[serde(rename = "sg_uen")]
    SgUen,
    #[serde(rename = "si_tin")]
    SiTin,
    #[serde(rename = "sv_nit")]
    SvNit,
    #[serde(rename = "th_vat")]
    ThVat,
    #[serde(rename = "tr_tin")]
    TrTin,
    #[serde(rename = "tw_vat")]
    TwVat,
    #[serde(rename = "ua_vat")]
    UaVat,
    #[serde(rename = "us_ein")]
    UsEin,
    #[serde(rename = "uy_ruc")]
    UyRuc,
    #[serde(rename = "ve_rif")]
    VeRif,
    #[serde(rename = "vn_tin")]
    VnTin,
    #[serde(rename = "za_vat")]
    ZaVat,
    #[serde(rename = "mu_tan")]
    MuTan,
}
impl TaxIDFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AdNrt => "ad_nrt",
            Self::AeTrn => "ae_trn",
            Self::ArCuit => "ar_cuit",
            Self::AuAbn => "au_abn",
            Self::AuArn => "au_arn",
            Self::BgUic => "bg_uic",
            Self::BhVat => "bh_vat",
            Self::BoTin => "bo_tin",
            Self::BrCnpj => "br_cnpj",
            Self::BrCpf => "br_cpf",
            Self::CaBn => "ca_bn",
            Self::CaGstHst => "ca_gst_hst",
            Self::CaPstBc => "ca_pst_bc",
            Self::CaPstMb => "ca_pst_mb",
            Self::CaPstSk => "ca_pst_sk",
            Self::CaQst => "ca_qst",
            Self::ChUid => "ch_uid",
            Self::ChVat => "ch_vat",
            Self::ClTin => "cl_tin",
            Self::CnTin => "cn_tin",
            Self::CoNit => "co_nit",
            Self::CrTin => "cr_tin",
            Self::DeStn => "de_stn",
            Self::DoRcn => "do_rcn",
            Self::EcRuc => "ec_ruc",
            Self::EgTin => "eg_tin",
            Self::EsCif => "es_cif",
            Self::EuOssVat => "eu_oss_vat",
            Self::EuVat => "eu_vat",
            Self::GbVat => "gb_vat",
            Self::GeVat => "ge_vat",
            Self::HkBr => "hk_br",
            Self::HrOib => "hr_oib",
            Self::HuTin => "hu_tin",
            Self::IdNpwp => "id_npwp",
            Self::IlVat => "il_vat",
            Self::InGst => "in_gst",
            Self::IsVat => "is_vat",
            Self::JpCn => "jp_cn",
            Self::JpRn => "jp_rn",
            Self::JpTrn => "jp_trn",
            Self::KePin => "ke_pin",
            Self::KrBrn => "kr_brn",
            Self::KzBin => "kz_bin",
            Self::LiUid => "li_uid",
            Self::MkVat => "mk_vat",
            Self::MxRfc => "mx_rfc",
            Self::MyFrp => "my_frp",
            Self::MyItn => "my_itn",
            Self::MySst => "my_sst",
            Self::NgTin => "ng_tin",
            Self::NoVat => "no_vat",
            Self::NoVoec => "no_voec",
            Self::NzGst => "nz_gst",
            Self::OmVat => "om_vat",
            Self::PeRuc => "pe_ruc",
            Self::PhTin => "ph_tin",
            Self::RoTin => "ro_tin",
            Self::RsPib => "rs_pib",
            Self::RuInn => "ru_inn",
            Self::RuKpp => "ru_kpp",
            Self::SaVat => "sa_vat",
            Self::SgGst => "sg_gst",
            Self::SgUen => "sg_uen",
            Self::SiTin => "si_tin",
            Self::SvNit => "sv_nit",
            Self::ThVat => "th_vat",
            Self::TrTin => "tr_tin",
            Self::TwVat => "tw_vat",
            Self::UaVat => "ua_vat",
            Self::UsEin => "us_ein",
            Self::UyRuc => "uy_ruc",
            Self::VeRif => "ve_rif",
            Self::VnTin => "vn_tin",
            Self::ZaVat => "za_vat",
            Self::MuTan => "mu_tan",
        }
    }
}
impl ::std::fmt::Display for TaxIDFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for TaxIDFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum TierType {
    #[serde(rename = "volume")]
    Volume,
    #[serde(rename = "graduated")]
    Graduated,
}
impl TierType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Volume => "volume",
            Self::Graduated => "graduated",
        }
    }
}
impl ::std::fmt::Display for TierType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for TierType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum TimeInterval {
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
}
impl TimeInterval {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Year => "year",
            Self::Month => "month",
            Self::Week => "week",
            Self::Day => "day",
            Self::Hour => "hour",
        }
    }
}
impl ::std::fmt::Display for TimeInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for TimeInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum TrialInterval {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl TrialInterval {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Year => "year",
        }
    }
}
impl ::std::fmt::Display for TrialInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for TrialInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Visibility {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}
impl Visibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Private => "private",
            Self::Public => "public",
        }
    }
}
impl ::std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for Visibility {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
