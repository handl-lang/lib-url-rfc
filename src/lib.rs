//! `url_rfc` — URL parser plus its `url_rfc.*` ops, registered into `lib-parse`.

pub const URL_RFC_HNL: &str = include_str!("index.hnl");

pub mod ops;

pub use ops::*;
