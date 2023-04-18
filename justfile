fmt:
  cargo +nightly fmt

clippy:
  cargo clippy

lint: fmt clippy

test:
  cargo test --all-features

bench:
  cargo bench --all-features

docs:
  cd .docs
  pnpm dev
