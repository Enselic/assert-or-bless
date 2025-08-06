/// Assert that `value` matches the snapshot at `snapshot_path`. If there is a
/// diff the function will panic with a colorful diff that shows what changed.
///
/// If the env var `UPDATE_SNAPSHOTS` is set to `1`, `yes` or `true` then
/// `value` will be written to `snapshot_file` instead of being asserted to
/// match.
pub fn assert_eq_or_update(value: impl AsRef<str>, snapshot_path: impl AsRef<std::path::Path>) {
    let update =
        std::env::var("UPDATE_SNAPSHOTS").map_or(false, |s| s == "1" || s == "yes" || s == "true");

    if update {
        std::fs::write(&snapshot_path, value.as_ref())
            .unwrap_or_else(|e| panic!("Error writing `{:?}`: {e}", snapshot_path.as_ref()));
    } else {
        let snapshot = std::fs::read_to_string(&snapshot_path)
            .unwrap_or_else(|e| panic!("Error reading `{:?}`: {e}", snapshot_path.as_ref()));
        similar_asserts::assert_eq!(value.as_ref(), snapshot);
    }
}
