# Produces a changelog using git-cliff and commits it to the active branch.
cliff version:
  git cliff --tag {{version}} -o CHANGELOG.md
  git add CHANGELOG.md
  git commit -m "CHANGELOG.md updated for version {{version}}"

# Installs the latests versions of package-specific tooling from cargo.
tools:
  cargo install just git-cliff cargo-audit cargo-auditable omnibor-cli
  cargo install cargo-dist cargo-release --locked

# Plan and build source code and artifacts.
dist:
  dist build
  dist plan

# Runs cargo release, not for use on workspaces.
prepare version: tools dist
  cargo release {{version}} --workspace

# Load a particular dataset.
load dataset:
  RUST_LOG=info cargo run --release -- -c load -d {{dataset}}

# Dataset loading test suite.
load_all:
  RUST_LOG=info cargo run --release -- -c load -d FixedAssets
  RUST_LOG=info cargo run --release -- -c load -d GdpByIndustry
  # RUST_LOG=info cargo run --release -- -c load -d Mne
  RUST_LOG=info cargo run --release -- -c load -d Nipa
  RUST_LOG=info cargo run --release -- -c load -d NIUnderlyingDetail
  RUST_LOG=info cargo run --release -- -c load -d Iip
  RUST_LOG=info cargo run --release -- -c load -d InputOutput
  RUST_LOG=info cargo run --release -- -c load -d Ita
  RUST_LOG=info cargo run --release -- -c load -d UnderlyingGdpByIndustry

# Dataset downloading test suite.
download_all:
  RUST_LOG=info cargo run --release -- -c download -d FixedAssets -x
  RUST_LOG=info cargo run --release -- -c download -d GdpByIndustry -x
  # RUST_LOG=info cargo run --release -- -c download -d Mne -x
  RUST_LOG=info cargo run --release -- -c download -d Nipa -x
  RUST_LOG=info cargo run --release -- -c download -d NIUnderlyingDetail -x
  RUST_LOG=info cargo run --release -- -c download -d Iip -x
  RUST_LOG=info cargo run --release -- -c download -d InputOutput -x
  RUST_LOG=info cargo run --release -- -c download -d Ita -x
  RUST_LOG=info cargo run --release -- -c download -d UnderlyingGdpByIndustry -x
  bash format_json.sh
