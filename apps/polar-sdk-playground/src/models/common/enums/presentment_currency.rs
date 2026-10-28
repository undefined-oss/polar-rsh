use serde::{Deserialize, Serialize};

// Values from the vendored 2026-10 OpenAPI schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
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

impl std::fmt::Display for PresentmentCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for PresentmentCurrency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
