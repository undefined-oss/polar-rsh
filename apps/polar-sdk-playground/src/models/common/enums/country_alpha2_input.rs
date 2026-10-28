use serde::{Deserialize, Serialize};

// Values from the vendored 2026-10 OpenAPI schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CountryAlpha2Input {
    #[serde(rename = "AD")]
    AD,
    #[serde(rename = "AE")]
    AE,
    #[serde(rename = "AF")]
    AF,
    #[serde(rename = "AG")]
    AG,
    #[serde(rename = "AI")]
    AI,
    #[serde(rename = "AL")]
    AL,
    #[serde(rename = "AM")]
    AM,
    #[serde(rename = "AO")]
    AO,
    #[serde(rename = "AQ")]
    AQ,
    #[serde(rename = "AR")]
    AR,
    #[serde(rename = "AS")]
    AS,
    #[serde(rename = "AT")]
    AT,
    #[serde(rename = "AU")]
    AU,
    #[serde(rename = "AW")]
    AW,
    #[serde(rename = "AX")]
    AX,
    #[serde(rename = "AZ")]
    AZ,
    #[serde(rename = "BA")]
    BA,
    #[serde(rename = "BB")]
    BB,
    #[serde(rename = "BD")]
    BD,
    #[serde(rename = "BE")]
    BE,
    #[serde(rename = "BF")]
    BF,
    #[serde(rename = "BG")]
    BG,
    #[serde(rename = "BH")]
    BH,
    #[serde(rename = "BI")]
    BI,
    #[serde(rename = "BJ")]
    BJ,
    #[serde(rename = "BL")]
    BL,
    #[serde(rename = "BM")]
    BM,
    #[serde(rename = "BN")]
    BN,
    #[serde(rename = "BO")]
    BO,
    #[serde(rename = "BQ")]
    BQ,
    #[serde(rename = "BR")]
    BR,
    #[serde(rename = "BS")]
    BS,
    #[serde(rename = "BT")]
    BT,
    #[serde(rename = "BV")]
    BV,
    #[serde(rename = "BW")]
    BW,
    #[serde(rename = "BY")]
    BY,
    #[serde(rename = "BZ")]
    BZ,
    #[serde(rename = "CA")]
    CA,
    #[serde(rename = "CC")]
    CC,
    #[serde(rename = "CD")]
    CD,
    #[serde(rename = "CF")]
    CF,
    #[serde(rename = "CG")]
    CG,
    #[serde(rename = "CH")]
    CH,
    #[serde(rename = "CI")]
    CI,
    #[serde(rename = "CK")]
    CK,
    #[serde(rename = "CL")]
    CL,
    #[serde(rename = "CM")]
    CM,
    #[serde(rename = "CN")]
    CN,
    #[serde(rename = "CO")]
    CO,
    #[serde(rename = "CR")]
    CR,
    #[serde(rename = "CV")]
    CV,
    #[serde(rename = "CW")]
    CW,
    #[serde(rename = "CX")]
    CX,
    #[serde(rename = "CY")]
    CY,
    #[serde(rename = "CZ")]
    CZ,
    #[serde(rename = "DE")]
    DE,
    #[serde(rename = "DJ")]
    DJ,
    #[serde(rename = "DK")]
    DK,
    #[serde(rename = "DM")]
    DM,
    #[serde(rename = "DO")]
    DO,
    #[serde(rename = "DZ")]
    DZ,
    #[serde(rename = "EC")]
    EC,
    #[serde(rename = "EE")]
    EE,
    #[serde(rename = "EG")]
    EG,
    #[serde(rename = "EH")]
    EH,
    #[serde(rename = "ER")]
    ER,
    #[serde(rename = "ES")]
    ES,
    #[serde(rename = "ET")]
    ET,
    #[serde(rename = "FI")]
    FI,
    #[serde(rename = "FJ")]
    FJ,
    #[serde(rename = "FK")]
    FK,
    #[serde(rename = "FM")]
    FM,
    #[serde(rename = "FO")]
    FO,
    #[serde(rename = "FR")]
    FR,
    #[serde(rename = "GA")]
    GA,
    #[serde(rename = "GB")]
    GB,
    #[serde(rename = "GD")]
    GD,
    #[serde(rename = "GE")]
    GE,
    #[serde(rename = "GF")]
    GF,
    #[serde(rename = "GG")]
    GG,
    #[serde(rename = "GH")]
    GH,
    #[serde(rename = "GI")]
    GI,
    #[serde(rename = "GL")]
    GL,
    #[serde(rename = "GM")]
    GM,
    #[serde(rename = "GN")]
    GN,
    #[serde(rename = "GP")]
    GP,
    #[serde(rename = "GQ")]
    GQ,
    #[serde(rename = "GR")]
    GR,
    #[serde(rename = "GS")]
    GS,
    #[serde(rename = "GT")]
    GT,
    #[serde(rename = "GU")]
    GU,
    #[serde(rename = "GW")]
    GW,
    #[serde(rename = "GY")]
    GY,
    #[serde(rename = "HK")]
    HK,
    #[serde(rename = "HM")]
    HM,
    #[serde(rename = "HN")]
    HN,
    #[serde(rename = "HR")]
    HR,
    #[serde(rename = "HT")]
    HT,
    #[serde(rename = "HU")]
    HU,
    #[serde(rename = "ID")]
    ID,
    #[serde(rename = "IE")]
    IE,
    #[serde(rename = "IL")]
    IL,
    #[serde(rename = "IM")]
    IM,
    #[serde(rename = "IN")]
    IN,
    #[serde(rename = "IO")]
    IO,
    #[serde(rename = "IQ")]
    IQ,
    #[serde(rename = "IS")]
    IS,
    #[serde(rename = "IT")]
    IT,
    #[serde(rename = "JE")]
    JE,
    #[serde(rename = "JM")]
    JM,
    #[serde(rename = "JO")]
    JO,
    #[serde(rename = "JP")]
    JP,
    #[serde(rename = "KE")]
    KE,
    #[serde(rename = "KG")]
    KG,
    #[serde(rename = "KH")]
    KH,
    #[serde(rename = "KI")]
    KI,
    #[serde(rename = "KM")]
    KM,
    #[serde(rename = "KN")]
    KN,
    #[serde(rename = "KR")]
    KR,
    #[serde(rename = "KW")]
    KW,
    #[serde(rename = "KY")]
    KY,
    #[serde(rename = "KZ")]
    KZ,
    #[serde(rename = "LA")]
    LA,
    #[serde(rename = "LB")]
    LB,
    #[serde(rename = "LC")]
    LC,
    #[serde(rename = "LI")]
    LI,
    #[serde(rename = "LK")]
    LK,
    #[serde(rename = "LR")]
    LR,
    #[serde(rename = "LS")]
    LS,
    #[serde(rename = "LT")]
    LT,
    #[serde(rename = "LU")]
    LU,
    #[serde(rename = "LV")]
    LV,
    #[serde(rename = "LY")]
    LY,
    #[serde(rename = "MA")]
    MA,
    #[serde(rename = "MC")]
    MC,
    #[serde(rename = "MD")]
    MD,
    #[serde(rename = "ME")]
    ME,
    #[serde(rename = "MF")]
    MF,
    #[serde(rename = "MG")]
    MG,
    #[serde(rename = "MH")]
    MH,
    #[serde(rename = "MK")]
    MK,
    #[serde(rename = "ML")]
    ML,
    #[serde(rename = "MM")]
    MM,
    #[serde(rename = "MN")]
    MN,
    #[serde(rename = "MO")]
    MO,
    #[serde(rename = "MP")]
    MP,
    #[serde(rename = "MQ")]
    MQ,
    #[serde(rename = "MR")]
    MR,
    #[serde(rename = "MS")]
    MS,
    #[serde(rename = "MT")]
    MT,
    #[serde(rename = "MU")]
    MU,
    #[serde(rename = "MV")]
    MV,
    #[serde(rename = "MW")]
    MW,
    #[serde(rename = "MX")]
    MX,
    #[serde(rename = "MY")]
    MY,
    #[serde(rename = "MZ")]
    MZ,
    #[serde(rename = "NA")]
    NA,
    #[serde(rename = "NC")]
    NC,
    #[serde(rename = "NE")]
    NE,
    #[serde(rename = "NF")]
    NF,
    #[serde(rename = "NG")]
    NG,
    #[serde(rename = "NI")]
    NI,
    #[serde(rename = "NL")]
    NL,
    #[serde(rename = "NO")]
    NO,
    #[serde(rename = "NP")]
    NP,
    #[serde(rename = "NR")]
    NR,
    #[serde(rename = "NU")]
    NU,
    #[serde(rename = "NZ")]
    NZ,
    #[serde(rename = "OM")]
    OM,
    #[serde(rename = "PA")]
    PA,
    #[serde(rename = "PE")]
    PE,
    #[serde(rename = "PF")]
    PF,
    #[serde(rename = "PG")]
    PG,
    #[serde(rename = "PH")]
    PH,
    #[serde(rename = "PK")]
    PK,
    #[serde(rename = "PL")]
    PL,
    #[serde(rename = "PM")]
    PM,
    #[serde(rename = "PN")]
    PN,
    #[serde(rename = "PR")]
    PR,
    #[serde(rename = "PS")]
    PS,
    #[serde(rename = "PT")]
    PT,
    #[serde(rename = "PW")]
    PW,
    #[serde(rename = "PY")]
    PY,
    #[serde(rename = "QA")]
    QA,
    #[serde(rename = "RE")]
    RE,
    #[serde(rename = "RO")]
    RO,
    #[serde(rename = "RS")]
    RS,
    #[serde(rename = "RW")]
    RW,
    #[serde(rename = "SA")]
    SA,
    #[serde(rename = "SB")]
    SB,
    #[serde(rename = "SC")]
    SC,
    #[serde(rename = "SD")]
    SD,
    #[serde(rename = "SE")]
    SE,
    #[serde(rename = "SG")]
    SG,
    #[serde(rename = "SH")]
    SH,
    #[serde(rename = "SI")]
    SI,
    #[serde(rename = "SJ")]
    SJ,
    #[serde(rename = "SK")]
    SK,
    #[serde(rename = "SL")]
    SL,
    #[serde(rename = "SM")]
    SM,
    #[serde(rename = "SN")]
    SN,
    #[serde(rename = "SO")]
    SO,
    #[serde(rename = "SR")]
    SR,
    #[serde(rename = "SS")]
    SS,
    #[serde(rename = "ST")]
    ST,
    #[serde(rename = "SV")]
    SV,
    #[serde(rename = "SX")]
    SX,
    #[serde(rename = "SZ")]
    SZ,
    #[serde(rename = "TC")]
    TC,
    #[serde(rename = "TD")]
    TD,
    #[serde(rename = "TF")]
    TF,
    #[serde(rename = "TG")]
    TG,
    #[serde(rename = "TH")]
    TH,
    #[serde(rename = "TJ")]
    TJ,
    #[serde(rename = "TK")]
    TK,
    #[serde(rename = "TL")]
    TL,
    #[serde(rename = "TM")]
    TM,
    #[serde(rename = "TN")]
    TN,
    #[serde(rename = "TO")]
    TO,
    #[serde(rename = "TR")]
    TR,
    #[serde(rename = "TT")]
    TT,
    #[serde(rename = "TV")]
    TV,
    #[serde(rename = "TW")]
    TW,
    #[serde(rename = "TZ")]
    TZ,
    #[serde(rename = "UA")]
    UA,
    #[serde(rename = "UG")]
    UG,
    #[serde(rename = "UM")]
    UM,
    #[serde(rename = "US")]
    US,
    #[serde(rename = "UY")]
    UY,
    #[serde(rename = "UZ")]
    UZ,
    #[serde(rename = "VA")]
    VA,
    #[serde(rename = "VC")]
    VC,
    #[serde(rename = "VE")]
    VE,
    #[serde(rename = "VG")]
    VG,
    #[serde(rename = "VI")]
    VI,
    #[serde(rename = "VN")]
    VN,
    #[serde(rename = "VU")]
    VU,
    #[serde(rename = "WF")]
    WF,
    #[serde(rename = "WS")]
    WS,
    #[serde(rename = "YE")]
    YE,
    #[serde(rename = "YT")]
    YT,
    #[serde(rename = "ZA")]
    ZA,
    #[serde(rename = "ZM")]
    ZM,
    #[serde(rename = "ZW")]
    ZW,
}

impl CountryAlpha2Input {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AD => "AD",
            Self::AE => "AE",
            Self::AF => "AF",
            Self::AG => "AG",
            Self::AI => "AI",
            Self::AL => "AL",
            Self::AM => "AM",
            Self::AO => "AO",
            Self::AQ => "AQ",
            Self::AR => "AR",
            Self::AS => "AS",
            Self::AT => "AT",
            Self::AU => "AU",
            Self::AW => "AW",
            Self::AX => "AX",
            Self::AZ => "AZ",
            Self::BA => "BA",
            Self::BB => "BB",
            Self::BD => "BD",
            Self::BE => "BE",
            Self::BF => "BF",
            Self::BG => "BG",
            Self::BH => "BH",
            Self::BI => "BI",
            Self::BJ => "BJ",
            Self::BL => "BL",
            Self::BM => "BM",
            Self::BN => "BN",
            Self::BO => "BO",
            Self::BQ => "BQ",
            Self::BR => "BR",
            Self::BS => "BS",
            Self::BT => "BT",
            Self::BV => "BV",
            Self::BW => "BW",
            Self::BY => "BY",
            Self::BZ => "BZ",
            Self::CA => "CA",
            Self::CC => "CC",
            Self::CD => "CD",
            Self::CF => "CF",
            Self::CG => "CG",
            Self::CH => "CH",
            Self::CI => "CI",
            Self::CK => "CK",
            Self::CL => "CL",
            Self::CM => "CM",
            Self::CN => "CN",
            Self::CO => "CO",
            Self::CR => "CR",
            Self::CV => "CV",
            Self::CW => "CW",
            Self::CX => "CX",
            Self::CY => "CY",
            Self::CZ => "CZ",
            Self::DE => "DE",
            Self::DJ => "DJ",
            Self::DK => "DK",
            Self::DM => "DM",
            Self::DO => "DO",
            Self::DZ => "DZ",
            Self::EC => "EC",
            Self::EE => "EE",
            Self::EG => "EG",
            Self::EH => "EH",
            Self::ER => "ER",
            Self::ES => "ES",
            Self::ET => "ET",
            Self::FI => "FI",
            Self::FJ => "FJ",
            Self::FK => "FK",
            Self::FM => "FM",
            Self::FO => "FO",
            Self::FR => "FR",
            Self::GA => "GA",
            Self::GB => "GB",
            Self::GD => "GD",
            Self::GE => "GE",
            Self::GF => "GF",
            Self::GG => "GG",
            Self::GH => "GH",
            Self::GI => "GI",
            Self::GL => "GL",
            Self::GM => "GM",
            Self::GN => "GN",
            Self::GP => "GP",
            Self::GQ => "GQ",
            Self::GR => "GR",
            Self::GS => "GS",
            Self::GT => "GT",
            Self::GU => "GU",
            Self::GW => "GW",
            Self::GY => "GY",
            Self::HK => "HK",
            Self::HM => "HM",
            Self::HN => "HN",
            Self::HR => "HR",
            Self::HT => "HT",
            Self::HU => "HU",
            Self::ID => "ID",
            Self::IE => "IE",
            Self::IL => "IL",
            Self::IM => "IM",
            Self::IN => "IN",
            Self::IO => "IO",
            Self::IQ => "IQ",
            Self::IS => "IS",
            Self::IT => "IT",
            Self::JE => "JE",
            Self::JM => "JM",
            Self::JO => "JO",
            Self::JP => "JP",
            Self::KE => "KE",
            Self::KG => "KG",
            Self::KH => "KH",
            Self::KI => "KI",
            Self::KM => "KM",
            Self::KN => "KN",
            Self::KR => "KR",
            Self::KW => "KW",
            Self::KY => "KY",
            Self::KZ => "KZ",
            Self::LA => "LA",
            Self::LB => "LB",
            Self::LC => "LC",
            Self::LI => "LI",
            Self::LK => "LK",
            Self::LR => "LR",
            Self::LS => "LS",
            Self::LT => "LT",
            Self::LU => "LU",
            Self::LV => "LV",
            Self::LY => "LY",
            Self::MA => "MA",
            Self::MC => "MC",
            Self::MD => "MD",
            Self::ME => "ME",
            Self::MF => "MF",
            Self::MG => "MG",
            Self::MH => "MH",
            Self::MK => "MK",
            Self::ML => "ML",
            Self::MM => "MM",
            Self::MN => "MN",
            Self::MO => "MO",
            Self::MP => "MP",
            Self::MQ => "MQ",
            Self::MR => "MR",
            Self::MS => "MS",
            Self::MT => "MT",
            Self::MU => "MU",
            Self::MV => "MV",
            Self::MW => "MW",
            Self::MX => "MX",
            Self::MY => "MY",
            Self::MZ => "MZ",
            Self::NA => "NA",
            Self::NC => "NC",
            Self::NE => "NE",
            Self::NF => "NF",
            Self::NG => "NG",
            Self::NI => "NI",
            Self::NL => "NL",
            Self::NO => "NO",
            Self::NP => "NP",
            Self::NR => "NR",
            Self::NU => "NU",
            Self::NZ => "NZ",
            Self::OM => "OM",
            Self::PA => "PA",
            Self::PE => "PE",
            Self::PF => "PF",
            Self::PG => "PG",
            Self::PH => "PH",
            Self::PK => "PK",
            Self::PL => "PL",
            Self::PM => "PM",
            Self::PN => "PN",
            Self::PR => "PR",
            Self::PS => "PS",
            Self::PT => "PT",
            Self::PW => "PW",
            Self::PY => "PY",
            Self::QA => "QA",
            Self::RE => "RE",
            Self::RO => "RO",
            Self::RS => "RS",
            Self::RW => "RW",
            Self::SA => "SA",
            Self::SB => "SB",
            Self::SC => "SC",
            Self::SD => "SD",
            Self::SE => "SE",
            Self::SG => "SG",
            Self::SH => "SH",
            Self::SI => "SI",
            Self::SJ => "SJ",
            Self::SK => "SK",
            Self::SL => "SL",
            Self::SM => "SM",
            Self::SN => "SN",
            Self::SO => "SO",
            Self::SR => "SR",
            Self::SS => "SS",
            Self::ST => "ST",
            Self::SV => "SV",
            Self::SX => "SX",
            Self::SZ => "SZ",
            Self::TC => "TC",
            Self::TD => "TD",
            Self::TF => "TF",
            Self::TG => "TG",
            Self::TH => "TH",
            Self::TJ => "TJ",
            Self::TK => "TK",
            Self::TL => "TL",
            Self::TM => "TM",
            Self::TN => "TN",
            Self::TO => "TO",
            Self::TR => "TR",
            Self::TT => "TT",
            Self::TV => "TV",
            Self::TW => "TW",
            Self::TZ => "TZ",
            Self::UA => "UA",
            Self::UG => "UG",
            Self::UM => "UM",
            Self::US => "US",
            Self::UY => "UY",
            Self::UZ => "UZ",
            Self::VA => "VA",
            Self::VC => "VC",
            Self::VE => "VE",
            Self::VG => "VG",
            Self::VI => "VI",
            Self::VN => "VN",
            Self::VU => "VU",
            Self::WF => "WF",
            Self::WS => "WS",
            Self::YE => "YE",
            Self::YT => "YT",
            Self::ZA => "ZA",
            Self::ZM => "ZM",
            Self::ZW => "ZW",
        }
    }
}

impl std::fmt::Display for CountryAlpha2Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for CountryAlpha2Input {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
