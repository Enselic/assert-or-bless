/// Asserts that `actual` matches the text-file snapshot at `snapshot_path`. If
/// there is a diff the function will panic with a helpful diff that shows what
/// changed.
///
/// If `bless` is `true` then `actual` will be written to the snapshot file at
/// `snapshot_path` instead of asserting that it matches.
pub fn assert_eq_or_bless_if(
    actual: impl AsRef<str>,
    snapshot_path: impl AsRef<std::path::Path>,
    bless: bool,
) {
    let snapshot_path = snapshot_path.as_ref();
    let actual = actual.as_ref();

    if bless {
        std::fs::write(snapshot_path, actual)
            .unwrap_or_else(|err| panic!("Writing `{snapshot_path:?}`: {err}"));
    } else {
        let expected = std::fs::read_to_string(snapshot_path)
            .unwrap_or_else(|err| panic!("Reading `{snapshot_path:?}`: {err}"));
        similar_asserts::assert_eq!(actual, expected);
    }
}

/// Same as [`assert_eq_or_bless_if`] but allows you to use an environment
/// variable to determine when to bless.
///
/// If the environment variable named `env_var_name` is set to `1`, `yes` or
/// `true` then `actual` will be written to the snapshot file at `snapshot_path`
/// instead of asserting that it matches with `actual`.
pub fn assert_eq_or_bless_with_env_var(
    actual: impl AsRef<str>,
    snapshot_path: impl AsRef<std::path::Path>,
    env_var_name: impl AsRef<str>,
) {
    assert_eq_or_bless_if(
        actual,
        snapshot_path,
        std::env::var(env_var_name.as_ref())
            .map_or(false, |s| s == "1" || s == "yes" || s == "true"),
    );
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

    pub fn bless(actual: impl AsRef<str>, snapshot_path: impl AsRef<std::path::Path>) {
        assert_eq_or_bless_if(actual, snapshot_path, true);
    }

    pub fn assert_eq(actual: impl AsRef<str>, snapshot_path: impl AsRef<std::path::Path>) {
        assert_eq_or_bless_if(actual, snapshot_path, false);
    }

    fn test_assert_eq(first: &str, second: &str) {
        let dir = DirGuard::new(
            std::env::temp_dir().join(format!("assert-or-bless-{}", fastrand::u32(0..1_000_000))),
        );
        let snapshot_path = dir.0.join("test-snapshot.txt");
        bless(first, &snapshot_path);
        assert_eq(second, &snapshot_path);
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
