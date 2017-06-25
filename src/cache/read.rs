// Bloom
//
// HTTP REST API caching middleware
// Copyright: 2017, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use hyper::{HttpVersion, Method};
use farmhash;

pub struct CacheRead;

impl CacheRead {
    pub fn gen_ns(shard: u8, version: HttpVersion, method: &Method, path: &str,
                    query: Option<&str>, authorization: &str) -> String {
        let namespace_raw = format!("[{}][{}][{}][{}][{}]", version, method,
            path, query.unwrap_or(""), authorization);
        let namespace_hash = farmhash::hash64(namespace_raw.as_bytes());

        debug!("Generated namespace: {} with hash: {}", namespace_raw,
            namespace_hash);

        format!("{}.{:x}", shard, namespace_hash)
    }

    pub fn acquire(ns: &str) {
        // TODO: Not implemented
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_valid_ns() {
        assert_eq!(CacheRead::gen_ns(
            0, HttpVersion::Http11, &Method::Get, "/", Some(""), ""),
            "0.f8bb423988eb2814", "[shard=0][auth=no] HTTP/1.1 GET /");
        assert_eq!(CacheRead::gen_ns(
            0, HttpVersion::Http11, &Method::Post, "/login", Some(""), ""),
            "0.9927a78b4d94dbf5", "[shard=0][auth=no] HTTP/1.1 POST /login");
        assert_eq!(CacheRead::gen_ns(
            7, HttpVersion::Http11, &Method::Options, "/feed", Some(""), "8ab"),
            "7.2b5dc16d448eecb9", "[shard=7][auth=yes] HTTP/1.1 OPTIONS /feed");
        assert_eq!(CacheRead::gen_ns(
            80, HttpVersion::H2, &Method::Head, "/user", Some("u=1"), "2d"),
            "80.7011223c059f2bfb", "[shard=80][auth=yes] h2 HEAD /feed");
    }
}