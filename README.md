## Dev

```
cargo clippy --all-features --tests --examples -- -D clippy::all
cargo +nightly clippy --all-features --tests --examples -- -D clippy::all

cargo fmt -- --check

cargo test-all-features -- --nocapture
```

## Publish

```shell
cargo publish --workspace
```

## Publish order

continent-code

country-code

data_set/countrycode
