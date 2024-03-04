cargo := $(env) cargo
rustup := $(env) rustup
# Nightly build is currently used for rustfmt and clippy.
nightly := $(shell cat ../rust-nightly-version)

# Build all wasms in release mode
all:
	RUSTFLAGS='-C link-arg=-s' $(cargo) build --release --target wasm32-unknown-unknown --target-dir 'target'
	cp target/wasm32-unknown-unknown/release/*.wasm .

debug:
	RUSTFLAGS='-C link-arg=-s' $(cargo) build --target wasm32-unknown-unknown --target-dir 'target'
	cp target/wasm32-unknown-unknown/debug/*.wasm .

check:
	$(cargo) +$(nightly) check --target wasm32-unknown-unknown

clippy:
	$(cargo) +$(nightly) clippy --all-targets -- -D warnings

clippy-fix:
	$(cargo) +$(nightly) clippy --fix -Z unstable-options --all-targets --allow-dirty --allow-staged

fmt:
	$(cargo) +$(nightly) fmt --all

fmt-check:
	$(cargo) +$(nightly) fmt --all -- --check

test:
	$(cargo) +$(nightly) test -- -Z unstable-options --report-time

clean:
	$(cargo) clean

deps:
	$(rustup) target add wasm32-unknown-unknown

.PHONY: all debug check clippy clippy-fix fmt fmt-check test clean deps