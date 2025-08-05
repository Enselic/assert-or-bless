### Audit the code

This crate is ~30 lines of code. Audit with the following one-liner, but make sure you follow to the [crates.io Data Access Policy](https://crates.io/data-access):

```sh
curl -H "User-Agent: $USER at $HOST" \
     -L https://crates.io/api/v1/crates/assert-or-bless/0.1.2/download |
         tar --extract --gzip --to-stdout | less
```

### Usage

Write a test:

```rs
#[test]
fn check_snapshot() {
    let actual == "...";
    assert_or_bless::assert_eq_or_bless(actual, "snapshot.txt");
}
```

Bless current output (write the snapshot file):

```sh
ASSERT_OR_BLESS=bless cargo test
```

Assert current output (compare with existing snapshot file):

```sh
cargo test
```

