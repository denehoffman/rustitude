alias b := build
alias d := develop

build:
  cargo update -q
  cargo build -r -q

develop:
  source .venv/bin/activate
  cargo update -q
  maturin develop -r -m py-rustitude/Cargo.toml

release:
  release-plz release-pr -u --verbose

publish:
  git pull
  cargo publish -p rustitude-core
  cargo publish -p rustitude-gluex
  cargo publish -p rustitude
