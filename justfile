check:
    ast-grep scan
    cargo clippy --all-features --workspace -- -D warnings

fix:
    cargo clippy --fix --allow-dirty --allow-staged --workspace -- -D warnings
    ast-grep scan --interactive 