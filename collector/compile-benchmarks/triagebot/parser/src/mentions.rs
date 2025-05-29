/// This provides a list of usernames or teams that were pinged in the text
/// provided.
///
/// It will appropriately skip mentions just like GitHub, i.e., mentions inside
/// code blocks will be ignored.
///
/// Note that the `@` is skipped in the final output.
pub fn get_mentions(input: &str) -> Vec<&str> {
    let ignore_regions = crate::ignore_block::IgnoreBlocks::new(input);

    let mut mentions = Vec::new();
    for (idx, _) in input.match_indices('@') {
        if let Some(previous) = input[..idx].chars().next_back() {
            // A github username must stand apart from other text.
            //
            // Oddly enough, english letters do not work, but letters outside
            // ASCII do work as separators; for now just go with this limited
            // list.
            if let 'a'..='z' | 'A'..='Z' | '0'..='9' = previous {
                continue;
            }
        }
        let mut saw_slash = false;
        let username_end = input
            .get(idx + 1..)
            .unwrap_or_default()
            .char_indices()
            .find(|(_, terminator)| match terminator {
                // These are the valid characters inside of a GitHub
                // username
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => false,
                '/' if !saw_slash => {
                    saw_slash = true;
                    false
                }
                _ => true,
            })
            .map(|(end, _)| idx + 1 + end)
            .unwrap_or(input.len());
        let username = input.get(idx + 1..username_end).unwrap_or_default();
        if username.is_empty() {
            continue;
        }
        if ignore_regions
            .overlaps_ignore(idx..idx + username.len())
            .is_some()
        {
            continue;
        }
        mentions.push(username);
    }
    mentions
}

#[test]
fn mentions_in_code_ignored() {
    assert_eq!(
        get_mentions("@rust-lang/libs `@user`"),
        vec!["rust-lang/libs"]
    );
    assert_eq!(get_mentions("@user `@user`"), vec!["user"]);
    assert_eq!(get_mentions("`@user`"), Vec::<&str>::new());
}

#[test]
fn italics() {
    assert_eq!(get_mentions("*@rust-lang/libs*"), vec!["rust-lang/libs"]);
}

#[test]
fn slash() {
    assert_eq!(
        get_mentions("@rust-lang/libs/@rust-lang/release"),
        vec!["rust-lang/libs", "rust-lang/release"]
    );
}

#[test]
fn no_panic_lone() {
    assert_eq!(get_mentions("@ `@`"), Vec::<&str>::new());
}

#[test]
fn no_email() {
    assert_eq!(get_mentions("user@example.com"), Vec::<&str>::new());
    assert_eq!(get_mentions("user123@example.com"), Vec::<&str>::new());
}
