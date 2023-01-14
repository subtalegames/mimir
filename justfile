set shell := ["pwsh.exe", "-c"]

fmt:
  cargo +nightly fmt

clippy:
  cargo clippy

lint: fmt clippy

test:
  cargo test --features float

bench:
  cargo bench --features float

docs:
  cd .docs
  pnpm dev
