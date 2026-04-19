use serde::{Deserialize, Serialize};

/// Issue #14: JSON で数値が文字列 "2000" のときも受け付ける
fn deserialize_i32_flexible<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum I32OrStr {
        I32(i32),
        Str(String),
    }
    match I32OrStr::deserialize(deserializer)? {
        I32OrStr::I32(n) => Ok(n),
        I32OrStr::Str(s) => s.trim().parse().map_err(serde::de::Error::custom),
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MusicData {
    pub title: String,
    pub janre: Janre,
    pub label: String,
    pub id: String,
    #[serde(deserialize_with = "deserialize_i32_flexible")]
    pub release_year: i32,
    pub record_year: Vec<i32>,
    pub personnel: Personnel,
    pub tracks: Vec<Track>,
    #[serde(deserialize_with = "deserialize_i32_flexible")]
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
    pub soloists: Vec<SoloistEntry>,
    #[serde(default)]
    pub leader: Vec<LeaderEntry>,
    #[serde(default)]
    pub sidemen: Vec<SidemenEntry>,
    #[serde(default)]
    pub group: Vec<GroupEntry>,
}

/// グループ（例: Art Blakey & The Jazz Messengers）。オプショナル。追加ボタンで1件ずつ追加。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GroupEntry {
    pub name: String,
    pub abbr: String,
    pub members: Vec<GroupMemberEntry>,
}

/// グループ内メンバー。leader は true のときのみ JSON に保存する。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GroupMemberEntry {
    pub name: String,
    pub instruments: String,
    pub tracks: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    pub leader: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SoloistEntry {
    pub name: String,
    #[serde(default)]
    pub instrument: String,
    pub tracks: String,
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
    #[serde(deserialize_with = "deserialize_i32_flexible")]
    pub disc_no: i32,
    #[serde(deserialize_with = "deserialize_i32_flexible")]
    pub no: i32,
    pub title: String,
    #[serde(deserialize_with = "deserialize_composer", serialize_with = "serialize_composer")]
    pub composer: String,
    pub length: String,
}

/// フォームの「トラック追加」で並べる次の `(disc_no, no)`。直前トラックと同じディスクで、番号は直前+1（issue #23）。
#[must_use]
pub fn disc_and_track_no_for_append(tracks: &[Track]) -> (i32, i32) {
    match tracks.last() {
        Some(t) => (t.disc_no, t.no.saturating_add(1)),
        None => (1, 1),
    }
}

fn deserialize_composer<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum ComposerSer {
        Str(String),
        Arr(Vec<String>),
    }
    match ComposerSer::deserialize(deserializer)? {
        ComposerSer::Str(s) => Ok(s),
        ComposerSer::Arr(a) => Ok(a.join(" | ")),
    }
}

fn serialize_composer<S>(s: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let arr: Vec<String> = s
        .split('|')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();
    if arr.len() <= 1 {
        serializer.serialize_str(s.trim())
    } else {
        serde::Serialize::serialize(&arr, serializer)
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

#[cfg(test)]
mod disc_track_append_tests {
    use super::{disc_and_track_no_for_append, Track};

    fn t(disc: i32, no: i32) -> Track {
        Track {
            disc_no: disc,
            no,
            ..Default::default()
        }
    }

    #[test]
    fn empty_defaults_to_disc1_track1() {
        assert_eq!(disc_and_track_no_for_append(&[]), (1, 1));
    }

    #[test]
    fn continues_same_disc_and_increments_track() {
        let tracks = vec![t(1, 1), t(1, 2)];
        assert_eq!(disc_and_track_no_for_append(&tracks), (1, 3));
    }

    #[test]
    fn follows_last_row_disc_not_always_one() {
        let tracks = vec![t(1, 1), t(1, 2), t(2, 1), t(2, 2), t(2, 3)];
        assert_eq!(disc_and_track_no_for_append(&tracks), (2, 4));
    }

    #[test]
    fn after_first_track_of_second_disc_next_is_track2_same_disc() {
        let tracks = vec![t(1, 8), t(2, 1)];
        assert_eq!(disc_and_track_no_for_append(&tracks), (2, 2));
    }
}
