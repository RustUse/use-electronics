.PHONY: help fmt check lint test test-minimal build doc examples audit deny sbom publish-dry-run-first-wave publish-dry-run-second-wave publish-dry-run-third-wave publish-dry-run-facade release-readiness second-wave-readiness third-wave-readiness facade-post-publish-validation verify

FIRST_WAVE_CRATES := use-component use-package use-net-label use-rating use-transistor use-board
SECOND_WAVE_CRATES := use-pin use-resistor use-capacitor use-diode
THIRD_WAVE_CRATE := use-circuit
FACADE_CRATE := use-electronics

help:
	@printf "%s\n" \
		"help                           Show available repository tasks" \
		"fmt                            Check formatting with rustfmt" \
		"check                          Run cargo check for the workspace" \
		"lint                           Run clippy with warnings denied" \
		"test                           Run workspace tests with all features" \
		"test-minimal                   Run workspace tests with no default features" \
		"build                          Build the workspace with all features" \
		"doc                            Build workspace docs without dependencies" \
		"examples                       Check all examples" \
		"audit                          Run cargo-audit" \
		"deny                           Run cargo-deny" \
		"sbom                           Generate a CycloneDX SBOM for $(FACADE_CRATE)" \
		"publish-dry-run-first-wave     List package contents and dry-run the independent first-wave crates" \
		"publish-dry-run-second-wave    Dry-run the second-wave crates after first-wave propagation" \
		"publish-dry-run-third-wave     Dry-run $(THIRD_WAVE_CRATE) after second-wave propagation" \
		"publish-dry-run-facade         Dry-run publish $(FACADE_CRATE) after crates.io propagation" \
		"release-readiness              Run the pre-release first-wave validation path" \
		"second-wave-readiness          Dry-run the second-wave crates after the first wave is live" \
		"third-wave-readiness           Dry-run $(THIRD_WAVE_CRATE) after the second wave is live" \
		"facade-post-publish-validation Dry-run the facade crate after focused crates are live" \
		"verify                         Run the main workspace validation path"

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

test-minimal:
	cargo test --workspace --no-default-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

examples:
	cargo check --workspace --all-features --examples

audit:
	cargo audit

deny:
	cargo deny check

sbom:
	cargo cyclonedx --manifest-path crates/$(FACADE_CRATE)/Cargo.toml --all-features --format json --spec-version 1.5 --override-filename sbom.cyclonedx

publish-dry-run-first-wave:
	@if [ -z "$(strip $(FIRST_WAVE_CRATES))" ]; then \
		printf "%s\n" "No first-wave crates configured"; \
	else \
		for crate in $(FIRST_WAVE_CRATES); do \
			cargo package --list -p $$crate; \
			cargo publish --dry-run --allow-dirty -p $$crate; \
		done; \
	fi

publish-dry-run-second-wave:
	@if [ -z "$(strip $(SECOND_WAVE_CRATES))" ]; then \
		printf "%s\n" "No second-wave crates configured"; \
	else \
		for crate in $(SECOND_WAVE_CRATES); do \
			cargo package --list -p $$crate; \
			cargo publish --dry-run --allow-dirty -p $$crate; \
		done; \
	fi

publish-dry-run-third-wave:
	@if [ -z "$(strip $(THIRD_WAVE_CRATE))" ]; then \
		printf "%s\n" "No focused crates configured"; \
	else \
		cargo package --list -p $(THIRD_WAVE_CRATE); \
		cargo publish --dry-run --allow-dirty -p $(THIRD_WAVE_CRATE); \
	fi

publish-dry-run-facade:
	cargo publish --dry-run --allow-dirty -p $(FACADE_CRATE)

release-readiness: verify examples test-minimal publish-dry-run-first-wave

second-wave-readiness: publish-dry-run-second-wave

third-wave-readiness: publish-dry-run-third-wave

facade-post-publish-validation: publish-dry-run-facade

verify: fmt lint test build
