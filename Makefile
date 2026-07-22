.PHONY: release

dev:
	virtualenv -p python3 venv
	. venv/bin/activate && pip install -r poke-engine-py/requirements.txt && pip install -r poke-engine-py/requirements-dev.txt && cd poke-engine-py && maturin develop --features="poke-engine/gen4"

upload_python_bindings:
	cd poke-engine-py && ./build_and_publish

upload_rust_lib:
	cargo publish --features "gen4"

release:
	./release

fmt:
	cargo fmt
	ruff format poke-engine-py

gen1:
	cargo build --release --features gen1 --no-default-features

gen2:
	cargo build --release --features gen2 --no-default-features

gen3:
	cargo build --release --features gen3 --no-default-features

# Gens 4-9 are one build (the const-generic genx engine); select the generation at
# runtime with `--gen N` (see README). There are no longer per-gen build targets.
genx:
	cargo build --release --no-default-features

champions:
	cargo build --release --features champions --no-default-features

bss:
	cargo build --release --features bss --no-default-features

pytest:
	. venv/bin/activate && pytest --rootdir=poke-engine-py/python poke-engine-py/python/tests

# One `cargo test` covers gens 4-9 (each genx test runs once per generation). champions
# and gen1/2/3 remain separate feature builds.
test: pytest
	cargo test --no-default-features
	cargo test --no-default-features --features "champions"
	cargo test --no-default-features --features "gen3"
	cargo test --no-default-features --features "gen2"
	cargo test --no-default-features --features "gen1"

install_ci:
	pip install -r poke-engine-py/requirements.txt
	pip install -r poke-engine-py/requirements-dev.txt
	cd poke-engine-py && maturin develop --features="poke-engine/gen4"

fmt_ci:
	cargo fmt -- --check
	ruff format --check poke-engine-py

test_ci:
	pytest --rootdir=poke-engine-py/python poke-engine-py/python/tests
	cargo test --no-default-features
	cargo test --no-default-features --features "champions"
	cargo test --no-default-features --features "gen3"
	cargo test --no-default-features --features "gen2"
	cargo test --no-default-features --features "gen1"

ci: install_ci fmt_ci test_ci
