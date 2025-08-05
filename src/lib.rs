pub fn assert_or_bless(actual: impl AsRef<str>, snapshot_path: impl AsRef<std::path::Path>) {
    assert_or_bless_if(
        actual,
        snapshot_path,
        std::env::var("ASSERT_OR_BLESS") == Ok("bless".to_string()),
    );
}

pub fn assert_or_bless_if(
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
