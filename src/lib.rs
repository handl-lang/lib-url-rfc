//! `url_rfc` — URL parser plus its `url_rfc.*` ops, registered into `lib-parse`.

pub const URL_RFC_HNL: &str = include_str!("index.hnl");



/// `.hnl` package source set — index + sibling files (wired into kits via
/// `handl_module_from_sources`).
pub const URL_RFC_FUNCS: &[&str] = &["parse","url_rfc"];
pub const URL_RFC_SOURCES: &[(&str, &str, &[&str])] = &[
    ("slist", include_str!("internal/slist.hnl"), &["slen","take","drop","s_slice","s_len"]),
    ("parse", include_str!("parse.hnl"), &["parse"]),
];

/// Flattened single-source `.hnl` (for `stdlib::MODULES` table rows).
pub const URL_RFC_FLAT_HNL: &str = include_str!("index_flat.hnl");
