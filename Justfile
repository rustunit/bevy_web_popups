alias c := check
alias d := doc
alias f := format
alias fmt := format

default:
    just --list

ci: check doc format

check:
    cargo clippy --target wasm32-unknown-unknown -- -Dwarnings

doc:
    cargo doc --all-features --no-deps --document-private-items --keep-going

format:
    cargo fmt --check

clean:
    cargo clean
