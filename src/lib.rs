/// Asserts that `value` matches the snapshot at `snapshot_path`. If there is a
/// diff the function will panic with a colorful diff that shows what changed.
///
/// If the env var `UPDATE_SNAPSHOTS` is set to `1`, `yes`, or `true`, the
/// public API will be written to the snapshot file instead of being
/// asserted to match.
pub fn assert_eq_or_update(
    value: impl AsRef<str>,
    snapshot_path: impl AsRef<std::path::Path>,
    bless: bool,
) {
    let snapshot_path = snapshot_path.as_ref();
    let actual = value.as_ref();

    if bless {
        std::fs::write(snapshot_path, actual)
            .unwrap_or_else(|err| panic!("Writing `{snapshot_path:?}`: {err}"));
    } else {
        let expected = std::fs::read_to_string(snapshot_path)
            .unwrap_or_else(|err| panic!("Reading `{snapshot_path:?}`: {err}"));
        similar_asserts::assert_eq!(actual, expected);
    }
}
