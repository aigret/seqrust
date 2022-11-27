
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 65,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "aarch64-unknown-linux-gnu".to_owned(),
                    short_version_string: "rustc 1.65.0 (897e37553 2022-11-02)".to_owned(),
                    commit_hash: Some("897e37553bba8b42751c67658967889d11ecd120".to_owned()),
                    commit_date: Some("2022-11-02".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            