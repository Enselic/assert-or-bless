/// Asserts that `actual` matches the text-file snapshot at `snapshot_path`. If
/// there is a diff the function will panic with a helpful diff that shows what
/// changed.
///
/// If the env var `ASSERT_OR_BLESS` is set to `bless` then `actual` will be
/// written to the snapshot file at `snapshot_path` instead of asserting that it
/// matches.
pub fn assert_eq_or_bless(actual: impl AsRef<str>, snapshot_path: impl AsRef<std::path::Path>) {
    assert_eq_or_bless_if(
        actual,
        snapshot_path,
        std::env::var("ASSERT_OR_BLESS") == Ok("bless".to_string()),
    );
}

/// Same as [`assert_eq_or_bless`] but allows you to use custom logic to
/// determine when to bless. Maybe you want to use a different environment
/// variable, for example.
pub fn assert_eq_or_bless_if(
    actual: impl AsRef<str>,
    snapshot_path: impl AsRef<std::path::Path>,
    bless: bool,
) {
    let snapshot_path = snapshot_path.as_ref();
    let actual = actual.as_ref();

    if bless {
        // Write the current public API to the snapshot path
        std::fs::write(snapshot_path, actual)
            .unwrap_or_else(|err| panic!("Writing `{snapshot_path:?}`: {err}"));
    } else {
        // Assert that the current public API matches the snapshot
        let expected = std::fs::read_to_string(snapshot_path)
            .unwrap_or_else(|err| panic!("Reading `{snapshot_path:?}`: {err}"));
        similar_asserts::assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DirGuard(std::path::PathBuf);
    impl DirGuard {
        fn new(path: std::path::PathBuf) -> Self {
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for DirGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn test_assert_eq(first: &str, second: &str) {
        let dir = DirGuard::new(
            std::env::temp_dir().join(format!("assert-or-bless-{}", fastrand::u32(0..1_000_000))),
        );
        let snapshot_path = dir.0.join("test-snapshot.txt");
        assert_eq_or_bless_if(first, &snapshot_path, true);
        assert_eq_or_bless(second, &snapshot_path);
    }

    #[test]
    fn assert_eq_succeeds() {
        test_assert_eq(
            "this is the\ncorrect snapshot contents",
            "this is the\ncorrect snapshot contents",
        );
    }

    #[test]
    #[should_panic]
    fn assert_eq_panics() {
        test_assert_eq(
            "this is the\ncorrect snapshot contents",
            "this is the\nWRONG snapshot contents",
        );
    }
}
