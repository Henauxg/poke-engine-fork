.PHONY: release

dev:
	virtualenv -p python3 venv
	. venv/bin/activate && pip install -r poke-engine-py/requirements.txt && pip install -r poke-engine-py/requirements-dev.txt && cd poke-engine-py && maturin develop

upload_python_bindings:
	cd poke-engine-py && ./build_and_publish

upload_rust_lib:
	cargo publish

release:
	./release

fmt:
	cargo fmt
	ruff format poke-engine-py

# All generations (1-9) are one build; select the generation at runtime with `--gen N`
# (see README). There are no longer per-generation build targets.
build:
	cargo build --release --no-default-features

champions:
	cargo build --release --features champions --no-default-features

bss:
	cargo build --release --features bss --no-default-features

pytest:
	. venv/bin/activate && pytest --rootdir=poke-engine-py/python poke-engine-py/python/tests

# One `cargo test` covers every generation (1-9): the genx suites run once per
# generation 4-9 and the gen1/2/3 suites run against their own engines. `champions` is
# the only remaining separate feature build.
test: pytest
	cargo test --no-default-features
	cargo test --no-default-features --features "champions"

install_ci:
	pip install -r poke-engine-py/requirements.txt
	pip install -r poke-engine-py/requirements-dev.txt
	cd poke-engine-py && maturin develop

fmt_ci:
	cargo fmt -- --check
	ruff format --check poke-engine-py

test_ci:
	pytest --rootdir=poke-engine-py/python poke-engine-py/python/tests
	cargo test --no-default-features
	cargo test --no-default-features --features "champions"

ci: install_ci fmt_ci test_ci
