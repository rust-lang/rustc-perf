use chrono::{DateTime, NaiveDate, Utc};
use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;

pub trait ExtractIf<T> {
    fn extract_if_stable<F>(&mut self, predicate: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool;
}

/// Vec method `extract_if` is unstable, this very simple implementation
/// can be deleted once it is stable
impl<T> ExtractIf<T> for Vec<T> {
    fn extract_if_stable<F>(&mut self, mut predicate: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        let mut extracted = Vec::new();
        let mut i = 0;

        while i < self.len() {
            if predicate(&self[i]) {
                extracted.push(self.remove(i));
            } else {
                i += 1;
            }
        }
        extracted
    }
}

/// Parses strings in the following formats extracting the Date & release tag
/// `static.rust-lang.org/dist/2016-05-24/channel-rust-1.9.0.toml`
/// `static.rust-lang.org/dist/2016-05-31/channel-rust-nightly.toml`
/// `static.rust-lang.org/dist/2016-06-01/channel-rust-beta.toml`
/// `static.rust-lang.org/dist/2025-06-26/channel-rust-1.89-beta.toml`
/// `static.rust-lang.org/dist/2025-06-26/channel-rust-1.89.0-beta.toml`
/// `static.rust-lang.org/dist/2025-06-26/channel-rust-1.89.0-beta.2.toml`
pub fn parse_release_string(url: &str) -> Option<(String, DateTime<Utc>)> {
    static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+\.\d+\.\d+)").unwrap());

    // Grab ".../YYYY-MM-DD/FILE.toml" components with Path helpers.
    let file = Path::new(url).file_name().and_then(|n| n.to_str())?;

    let date_str = Path::new(url)
        .parent()
        .and_then(Path::file_name)
        .and_then(|n| n.to_str())?;

    // No other beta releases are recognized as toolchains.
    //
    // We also have names like this:
    //
    // * channel-rust-1.75-beta.toml
    // * channel-rust-1.75.0-beta.toml
    // * channel-rust-1.75.0-beta.1.toml
    //
    // Which should get ignored for now, they're not consumable via rustup yet.
    if file.contains("beta") && file != "channel-rust-beta.toml" {
        return None;
    }

    // Parse the YYYY-MM-DD segment and stamp it with *current* UTC time.
    if let Ok(naive) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let published = naive
            .and_hms_opt(0, 0, 0)
            .expect("valid HMS")
            .and_local_timezone(Utc)
            .single()
            .unwrap();

        // Special-case the rolling beta channel.
        if file == "channel-rust-beta.toml" {
            return Some((format!("beta-{date_str}"), published));
        }

        // Otherwise pull out a semver like "1.70.0" and return it.
        if let Some(cap) = VERSION_RE.captures(file).and_then(|m| m.get(1)) {
            return Some((cap.as_str().to_owned(), published));
        }
    }

    None
}

// Copy of Iterator::partition_in_place, which is currently unstable.
pub fn partition_in_place<'a, I, T: 'a, P>(mut iter: I, mut predicate: P) -> usize
where
    I: Sized + DoubleEndedIterator<Item = &'a mut T>,
    P: FnMut(&T) -> bool,
{
    // FIXME: should we worry about the count overflowing? The only way to have more than
    // `usize::MAX` mutable references is with ZSTs, which aren't useful to partition...

    // These closure "factory" functions exist to avoid genericity in `Self`.

    #[inline]
    fn is_false<'a, T>(
        predicate: &'a mut impl FnMut(&T) -> bool,
        true_count: &'a mut usize,
    ) -> impl FnMut(&&mut T) -> bool + 'a {
        move |x| {
            let p = predicate(&**x);
            *true_count += p as usize;
            !p
        }
    }

    #[inline]
    fn is_true<T>(predicate: &mut impl FnMut(&T) -> bool) -> impl FnMut(&&mut T) -> bool + '_ {
        move |x| predicate(&**x)
    }

    // Repeatedly find the first `false` and swap it with the last `true`.
    let mut true_count = 0;
    while let Some(head) = iter.find(is_false(&mut predicate, &mut true_count)) {
        if let Some(tail) = iter.rfind(is_true(&mut predicate)) {
            std::mem::swap(head, tail);
            true_count += 1;
        } else {
            break;
        }
    }
    true_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    /// Helper: unwrap the Option, panic otherwise.
    fn tag(url: &str) -> String {
        parse_release_string(url)
            .expect("Some") // Option<_>
            .0 // take the tag
    }

    /// Helper: unwrap the DateTime and keep only the YYYY-MM-DD part
    fn day(url: &str) -> NaiveDate {
        parse_release_string(url).expect("Some").1.date_naive()
    }

    #[test]
    fn parses_stable_versions() {
        assert_eq!(
            tag("static.rust-lang.org/dist/2016-05-24/channel-rust-1.9.0.toml"),
            "1.9.0"
        );
        assert_eq!(
            day("static.rust-lang.org/dist/2016-05-24/channel-rust-1.9.0.toml"),
            NaiveDate::from_ymd_opt(2016, 5, 24).unwrap()
        );

        assert_eq!(
            tag("static.rust-lang.org/dist/2025-06-26/channel-rust-1.88.0.toml"),
            "1.88.0"
        );
        assert_eq!(
            day("static.rust-lang.org/dist/2025-06-26/channel-rust-1.88.0.toml"),
            NaiveDate::from_ymd_opt(2025, 6, 26).unwrap()
        );
    }

    #[test]
    fn parses_plain_beta_channel() {
        let want = "beta-2016-06-01";
        let url = "static.rust-lang.org/dist/2016-06-01/channel-rust-beta.toml";

        assert_eq!(tag(url), want);
        assert_eq!(day(url), NaiveDate::from_ymd_opt(2016, 6, 1).unwrap());
    }

    #[test]
    fn skips_unconsumable_channels() {
        // nightly never returns Anything
        assert!(parse_release_string(
            "static.rust-lang.org/dist/2016-05-31/channel-rust-nightly.toml"
        )
        .is_none());

        // versioned-beta artefacts are skipped too
        for should_ignore in [
            "static.rust-lang.org/dist/2025-06-26/channel-rust-1.89-beta.toml",
            "static.rust-lang.org/dist/2025-06-26/channel-rust-1.89.0-beta.toml",
            "static.rust-lang.org/dist/2025-06-26/channel-rust-1.89.0-beta.2.toml",
        ] {
            assert!(
                parse_release_string(should_ignore).is_none(),
                "{should_ignore} should be ignored"
            );
        }
    }
}
