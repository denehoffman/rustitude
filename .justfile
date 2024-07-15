alias b := build
alias bf := build-float
alias d := develop
alias df := develop-float
alias t := test
alias tf := test-float

default:
  just --list

build:
  cargo update -q
  cargo build -r -q

build-float:
  cargo update -q
  cargo build -r -q -F float

develop:
  source .venv/bin/activate
  cargo update -q
  maturin develop -r -m py-rustitude/Cargo.toml

develop-float:
  source .venv/bin/activate
  cargo update -q
  maturin develop -r -m py-rustitude/Cargo.toml -F float

release:
  release-plz release-pr -u --verbose

publish:
  git pull
  cargo publish -p rustitude-core
  cargo publish -p rustitude-gluex
  cargo publish -p rustitude
  cargo publish -p py-rustitude

pydoc:
  sphinx-build -M html docs/source/ docs/build/

test:
  cargo watch -x 'nextest r'

test-float:
  cargo watch -x 'nextest r -F float'
