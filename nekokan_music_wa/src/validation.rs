use crate::types::*;
use std::collections::HashMap;

pub type FieldErrors = HashMap<String, String>;

fn valid_len(s: &str, max: usize) -> bool {
    s.chars().count() <= max
}

fn valid_year(y: i32) -> bool {
    (1900..=2099).contains(&y)
}

fn valid_length_format(s: &str) -> bool {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return false;
    }
    parts[0].trim().parse::<i32>().is_ok() && parts[1].trim().parse::<i32>().is_ok()
}

fn valid_url(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let s = s.trim();
    (s.starts_with("http://") || s.starts_with("https://"))
        && !s.contains(char::is_whitespace)
        && s.len() > 10
}

fn valid_filename(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let forbidden = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', '\0'];
    !s.chars().any(|c| forbidden.contains(&c)) && s.len() <= 255
}

pub fn validate_form(data: &MusicData, filename: &str) -> FieldErrors {
    let mut err = FieldErrors::new();

    if data.title.is_empty() {
        err.insert("title".into(), "必須です".into());
    } else if !valid_len(&data.title, 128) {
        err.insert("title".into(), "128文字以内".into());
    }

    if data.janre.main.is_empty() {
        err.insert("janre.main".into(), "Main Janreを選択してください".into());
    }

    if data.janre.sub.is_empty() {
        err.insert("janre.sub".into(), "Sub Janreを1つ以上選択してください".into());
    }

    if data.label.is_empty() {
        err.insert("label".into(), "必須です".into());
    } else if !valid_len(&data.label, 64) {
        err.insert("label".into(), "64文字以内".into());
    }

    if data.id.is_empty() {
        err.insert("id".into(), "必須です".into());
    } else if !valid_len(&data.id, 64) {
        err.insert("id".into(), "64文字以内".into());
    }

    if !valid_year(data.release_year) {
        err.insert("release_year".into(), "1900〜2099の整数".into());
    }

    if data.record_year.is_empty() {
        err.insert("record_year".into(), "1つ以上の年をカンマ区切りで入力".into());
    } else if data.record_year.iter().any(|&y| !valid_year(y)) {
        err.insert("record_year".into(), "各年は1900〜2099".into());
    }

    for (i, c) in data.personnel.conductor.iter().enumerate() {
        if !valid_len(&c.name, 128) {
            err.insert(format!("personnel.conductor[{}].name", i), "128文字以内".into());
        }
        if !valid_len(&c.tracks, 64) {
            err.insert(format!("personnel.conductor[{}].tracks", i), "64文字以内".into());
        }
    }
    for (i, o) in data.personnel.orchestra.iter().enumerate() {
        if !valid_len(&o.name, 128) {
            err.insert(format!("personnel.orchestra[{}].name", i), "128文字以内".into());
        }
        if !valid_len(&o.tracks, 64) {
            err.insert(format!("personnel.orchestra[{}].tracks", i), "64文字以内".into());
        }
    }
    for (i, c) in data.personnel.company.iter().enumerate() {
        if !valid_len(&c.name, 128) {
            err.insert(format!("personnel.company[{}].name", i), "128文字以内".into());
        }
        if !valid_len(&c.tracks, 64) {
            err.insert(format!("personnel.company[{}].tracks", i), "64文字以内".into());
        }
    }
    for (i, l) in data.personnel.leader.iter().enumerate() {
        if !valid_len(&l.name, 128) {
            err.insert(format!("personnel.leader[{}].name", i), "128文字以内".into());
        }
        if !valid_len(&l.instruments, 128) {
            err.insert(format!("personnel.leader[{}].instruments", i), "128文字以内".into());
        }
        if !valid_len(&l.tracks, 64) {
            err.insert(format!("personnel.leader[{}].tracks", i), "64文字以内".into());
        }
    }
    for (i, s) in data.personnel.sidemen.iter().enumerate() {
        if !valid_len(&s.name, 128) {
            err.insert(format!("personnel.sidemen[{}].name", i), "128文字以内".into());
        }
        if !valid_len(&s.instruments, 128) {
            err.insert(format!("personnel.sidemen[{}].instruments", i), "128文字以内".into());
        }
        if !valid_len(&s.tracks, 64) {
            err.insert(format!("personnel.sidemen[{}].tracks", i), "64文字以内".into());
        }
    }
    for (gi, g) in data.personnel.group.iter().enumerate() {
        if g.name.is_empty() {
            err.insert(format!("personnel.group[{}].name", gi), "必須です".into());
        } else if !valid_len(&g.name, 128) {
            err.insert(format!("personnel.group[{}].name", gi), "128文字以内".into());
        }
        if g.abbr.is_empty() {
            err.insert(format!("personnel.group[{}].abbr", gi), "必須です".into());
        } else if !valid_len(&g.abbr, 64) {
            err.insert(format!("personnel.group[{}].abbr", gi), "64文字以内".into());
        }
        for (mi, m) in g.members.iter().enumerate() {
            if m.name.is_empty() {
                err.insert(
                    format!("personnel.group[{}].members[{}].name", gi, mi),
                    "必須です".into(),
                );
            } else if !valid_len(&m.name, 128) {
                err.insert(
                    format!("personnel.group[{}].members[{}].name", gi, mi),
                    "128文字以内".into(),
                );
            }
            if m.instruments.is_empty() {
                err.insert(
                    format!("personnel.group[{}].members[{}].instruments", gi, mi),
                    "必須です".into(),
                );
            } else if !valid_len(&m.instruments, 128) {
                err.insert(
                    format!("personnel.group[{}].members[{}].instruments", gi, mi),
                    "128文字以内".into(),
                );
            }
            if m.tracks.is_empty() {
                err.insert(
                    format!("personnel.group[{}].members[{}].tracks", gi, mi),
                    "必須です".into(),
                );
            } else if !valid_len(&m.tracks, 64) {
                err.insert(
                    format!("personnel.group[{}].members[{}].tracks", gi, mi),
                    "64文字以内".into(),
                );
            }
        }
    }

    if data.tracks.is_empty() {
        err.insert("tracks".into(), "1件以上のトラックが必要です".into());
    }
    for (i, t) in data.tracks.iter().enumerate() {
        if !valid_len(&t.title, 128) {
            err.insert(format!("tracks[{}].title", i), "128文字以内".into());
        }
        if !valid_len(&t.composer, 128) {
            err.insert(format!("tracks[{}].composer", i), "128文字以内".into());
        }
        if !valid_length_format(&t.length) {
            err.insert(format!("tracks[{}].length", i), "分:秒の形式（例 4:46）".into());
        }
    }

    if !(1..=6).contains(&data.score) {
        err.insert("score".into(), "1〜6を選択".into());
    }

    if data.date.is_empty() {
        err.insert("date".into(), "YYYY/MM/DDで入力".into());
    } else {
        let parts: Vec<&str> = data.date.split('/').collect();
        if parts.len() != 3
            || parts[0].len() != 4
            || parts[1].len() != 2
            || parts[2].len() != 2
            || parts[0].parse::<i32>().is_err()
            || parts[1].parse::<u32>().is_err()
            || parts[2].parse::<u32>().is_err()
        {
            err.insert("date".into(), "YYYY/MM/DDの形式で".into());
        }
    }

    for (i, r) in data.references.iter().enumerate() {
        if !valid_len(&r.name, 128) {
            err.insert(format!("references[{}].name", i), "128文字以内".into());
        }
        if !valid_url(&r.url) {
            err.insert(format!("references[{}].url", i), "有効なURLを入力".into());
        }
    }

    if filename.is_empty() {
        err.insert("filename".into(), "ファイル名を入力してください".into());
    } else {
        let f = filename.trim();
        let f = if f.ends_with(".json") {
            f.strip_suffix(".json").unwrap_or(f)
        } else {
            f
        };
        if !valid_filename(f) {
            err.insert("filename".into(), "ファイル名に使用できない文字が含まれています".into());
        }
    }

    err
}
