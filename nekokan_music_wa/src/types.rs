use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MusicData {
    pub title: String,
    pub janre: Janre,
    pub label: String,
    pub id: String,
    pub release_year: i32,
    pub record_year: Vec<i32>,
    pub personnel: Personnel,
    pub tracks: Vec<Track>,
    pub score: i32,
    pub comment: String,
    pub date: String,
    #[serde(default)]
    pub references: Vec<Reference>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Janre {
    pub main: String,
    pub sub: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Personnel {
    #[serde(default)]
    pub conductor: Vec<ConductorEntry>,
    #[serde(default)]
    pub orchestra: Vec<OrchestraEntry>,
    #[serde(default)]
    pub company: Vec<CompanyEntry>,
    #[serde(default)]
    pub leader: Vec<LeaderEntry>,
    #[serde(default)]
    pub sidemen: Vec<SidemenEntry>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConductorEntry {
    pub name: String,
    pub tracks: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OrchestraEntry {
    pub name: String,
    pub tracks: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CompanyEntry {
    pub name: String,
    pub tracks: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LeaderEntry {
    pub name: String,
    pub instruments: String,
    pub tracks: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SidemenEntry {
    pub name: String,
    pub instruments: String,
    pub tracks: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub disc_no: i32,
    pub no: i32,
    pub title: String,
    #[serde(deserialize_with = "deserialize_composer", serialize_with = "serialize_composer")]
    pub composer: String,
    pub length: String,
}

fn deserialize_composer<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum ComposerSer {
        Str(String),
        Arr(Vec<String>),
    }
    match ComposerSer::deserialize(deserializer)? {
        ComposerSer::Str(s) => Ok(s),
        ComposerSer::Arr(a) => Ok(a.join(", ")),
    }
}

fn serialize_composer<S>(s: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let arr: Vec<&str> = s.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()).collect();
    if arr.len() <= 1 {
        serializer.serialize_str(s)
    } else {
        let vec: Vec<String> = arr.iter().map(|x| x.to_string()).collect();
        serde::Serialize::serialize(&vec, serializer)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub name: String,
    pub url: String,
}

pub const MAIN_JANRES: &[&str] = &[
    "Classical",
    "Jazz",
    "Fusion",
    "Pops",
    "Progressive Rock",
    "Rock",
    "Nature",
    "Healing",
    "Art",
    "English",
    "Game",
];

pub fn sub_janres_for_main(main: &str) -> &'static [&'static str] {
    match main {
        "Classical" => &[
            "Early", "Baroque", "Classicists", "Romanticism", "Late Romanticism",
            "Nationalist", "Impressionist", "Modern", "Contemporary",
        ],
        "Jazz" => &[
            "Dixieland", "Symphonic", "Swing", "Bebop", "Hard Bop", "Cool",
            "New Mainstream", "Avrant-Garde", "Free", "Neo Hard Bop", "Post Hard Bop",
            "West Coast", "Modern", "Acid", "Chamber Music", "Soul", "Game", "Bigband", "Jazzrock", "Mode",
        ],
        "Fusion" => &["Funk", "Disco", "Soft", "Straight Ahead", "Fusion", "Rock", "Contemporary", "Urban Soul"],
        "Game" => &["Game", "Jazz", "Fusion", "Classical"],
        "Rock" => &["Progressive Rock", "Punk", "Rock"],
        _ => MAIN_JANRES,
    }
}
