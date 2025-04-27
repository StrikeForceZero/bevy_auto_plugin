use rustc_version::{Channel, version_meta};

fn main() {
    // panic!("{:#?}", std::env::vars().filter(|v| v.0.starts_with("CARGO_FEATURE_")).collect::<Vec<_>>());
    let version = version_meta().expect("Failed to get rustc version");
    let Some(commit_date) = version.commit_date else {
        return;
    };
    let _all = std::env::var("CARGO_FEATURE__ALL").is_ok();
    let nightly_requested = std::env::var("CARGO_FEATURE_NIGHTLY").is_ok();
    let old_nightly_requested = std::env::var("CARGO_FEATURE_NIGHTLY_PRE_2025_04_16").is_ok();
    if version.channel == Channel::Nightly {
        if _all {
            // valid (internal)
            return;
        }
        if nightly_requested && old_nightly_requested {
            panic!("nightly and nightly_pre_2025_04_16 features are mutually exclusive");
        }
        let old_nightly = commit_date.as_str() < "2025-04-16";
        if old_nightly_requested && old_nightly || !old_nightly_requested && !old_nightly {
            // valid
            return;
        }
        if nightly_requested && !old_nightly {
            panic!("nightly feature requires toolchain nightly or nightly-2025-04-16 or later");
        }
        if !old_nightly && old_nightly_requested {
            panic!(
                "nightly_pre_2025_04_16 feature requires toolchain nightly-2025-04-15 or earlier"
            );
        }
        if old_nightly && !old_nightly_requested {
            panic!("nightly_pre_2025_04_16 feature is required");
        }
    } else if nightly_requested || old_nightly_requested {
        panic!("nightly features not supported on {:?}", version.channel);
    }
}
