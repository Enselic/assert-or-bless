pub fn assert_eq_or_bless(actual: impl AsRef<str>, snapshot_path: impl AsRef<std::path::Path>) {
    assert_eq_or_bless_if(
        actual,
        snapshot_path,
        std::env::var("ASSERT_OR_BLESS") == Ok("bless".to_string()),
    );
}

pub fn assert_eq_or_bless_if(
    actual: impl AsRef<str>,
    snapshot_path: impl AsRef<std::path::Path>,
    bless: bool,
) {
    let snapshot_path = snapshot_path.as_ref();
    let actual = actual.as_ref();

    if bless {
        // Write the current public API to the snapshot path
        std::fs::write(snapshot_path, actual).unwrap_or_else(|err| {
            panic!("Failed to write snapshot to `{:?}`: {err}", snapshot_path,)
        });
    } else {
        // Assert that the current public API matches the snapshot
        let expected = std::fs::read_to_string(snapshot_path).unwrap_or_else(|err| {
            panic!("Failed to read snapshot from `{:?}`: {err}", snapshot_path,)
        });
        similar_asserts::assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_assert_eq(first: &str, second: &str) {
        let test_dir = tempfile::tempdir().unwrap();
        let snapshot_path = test_dir.path().join("test-snapshot.txt");

        assert_eq_or_bless_if(first, &snapshot_path, true);
        assert_eq_or_bless(second, &snapshot_path);
    }

    #[test]
    fn assert_eq_succeeds() {
        test_assert_eq(
            "this is the\ncorrect snapshot contents",
            "this is the\nWRONG snapshot contents",
        );
    }

    #[test]
    #[should_panic]
    fn assert_eq_fails() {
        test_assert_eq(
            "this is the\ncorrect snapshot contents",
            "this is the\ncorrect snapshot contents",
        );
    }
}
