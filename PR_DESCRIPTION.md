# Serve gens 4-9 from one library via `const GEN: u8` (const-generic `genx`)

Converts the `genx` engine (generations 4-9) from six mutually-exclusive Cargo features
into a single library generic over `const GEN: u8`, with zero hot-path cost. Generation
differences that were `#[cfg(feature = "genN")]` are now `GEN == N` / `GEN >= N` branches
that LLVM constant-folds per instantiation: no runtime generation field on `State`, no trait
objects, no enum dispatch inside the engine. Generations 1/2/3 stay separate, compile-time
engines.

Based on `bcf13823abc162a608e187b26bbf683f759f385e` (v0.0.48), the exact revision the consumer
pinned. The default branch had not moved past it, so no re-baselining of performance numbers
was needed.

## Design

- **Const parameter.** Every hot-path entry point is generic over `const GEN: u8`
  (`generate_instructions_from_move_pair::<5>`, `State::deserialize::<5>`, ...). A
  compile-fail guard (`AssertGenInRange::<GEN>::CHECK`) rejects `GEN` outside `4..=9` at
  monomorphization.
- **Runtime facade.** `gen_dispatch::for_gen::*` maps a `gen: u8` to the right instantiation
  once, at the crate edge, for callers that hold the generation as data (the CLI's `--gen`
  flag is one). `champions`/`bss` always behave as gen 9.
- **Move table.** The ~19k-line table builder became `choices::add_all_moves::<GEN>`; the
  genx path memoizes one table per generation, lazily, via `choices::moves::<GEN>()`. Unused
  generations are never built (measured: first-touch is a one-time few-ms cost per generation
  actually used).
- **gen1/2/3 untouched.** The crate-root code they share with genx (`state.rs`, `io.rs`,
  `mcts.rs`, `search.rs`) is generic over `GEN`; a thin `gen_dispatch::dispatch::*` shim
  forwards `GEN` on the genx path and ignores it on the gen1/2/3 path, so the `gen1/`,
  `gen2/`, `gen3/` modules did not change.
- The `terastallization` feature is gone; its mechanics are `GEN >= 9`. `champions`/`bss`
  remain ordinary features, orthogonal to gen, mapped to gen-9 behaviour.

See the README "Generations (const-generic `genx` engine)" section and MIGRATION.md.

## Performance (mandatory: within 5% at the same base commit)

Micro-benchmark `data/bench_gi.rs`: `generate_instructions_from_move_pair` on a fixed 6v6
state, 2,000,000 iterations after warmup, gen 5. Baseline built from the base commit with
`--features gen5`; converted built from this branch and instantiated `::<5>`. Both binaries
run back-to-back on the same machine under identical concurrent load (the box was running
other benchmarks; back-to-back alternation controls for it).

| run | baseline (us/call) | converted (us/call) |
| --- | --- | --- |
| 1 | 0.56321 | 0.57750 |
| 2 | 0.56354 | 0.57424 |
| 3 | 0.55958 | 0.58417 |
| 4 | 0.58547 | 0.57132 |
| **avg** | **0.5680** | **0.5768** |

Converted is **+1.55%** vs baseline — within the 5% budget. (An earlier converted-only
reading of ~0.72 us was a transient load spike; the back-to-back comparison controls for it.)
Reproduce: `cargo build --release --no-default-features --bin bench_gi && ./bench_gi`.

## Tests: one `cargo test` covers gens 4-9

Each genx test file re-includes its shared test bodies (in `tests/impls/`) once per
generation via a `gen_tests!` macro (`genN::test_x`), so a single `cargo test` exercises
gens 4..=9. Per-gen expected values that were `#[cfg]`-gated became `if GEN == N` branches;
tests that only apply to some gens early-return for the others.

| suite | result |
| --- | --- |
| lib unit tests (gens 4-9) | 1375 passed |
| `test_battle_mechanics` (gens 4-9) | 4194 passed |
| `test_last_used_move` (gens 4-9) | 102 passed |
| `test_damage_dealt` (gens 4-9) | 90 passed |
| doctest | 1 passed |
| `--features champions` (runs as gen 9) | all passed |
| `test_bss_mechanics` (`--features bss`) | 3 passed |
| `test_gen1` (`--features gen1`) | 42 passed |
| `test_gen2` (`--features gen2`) | 35 passed |
| `test_gen3` (`--features gen3`) | 15 passed |

Each genx test file re-includes shared bodies in `tests/impls/` (integration tests) or a
sibling `*_test_bodies.rs` (the inline unit-test modules) once per generation. Under
`--features champions` the harnesses instantiate only the gen-9 module (champions is a
gen-9 format), matching the pre-conversion single-build semantics.

## clippy

The conversion is clippy-neutral: the base commit already emits ~332 `cargo clippy` warnings
(pre-existing style lints across the engine); this branch emits a comparable count and the
new code (`gen_dispatch.rs`, the `const fn` conversions, the dispatch shim) is clippy-clean.
The pre-existing warnings are intentionally left untouched to keep the diff mechanical and
avoid reformatting code this change does not otherwise touch.

## Phase 2 (design only)

Folding gen1/2/3 behind the same facade means unifying their smaller enums and `MoveChoice`
into the genx superset (the enums are already `#[repr(u8)]` + `FromStr`, so the superset is
additive), after which the gen1/2/3 bodies become `GEN == 1|2|3` arms in the same generic
functions and `MIN_GEN` drops to 1. `add_all_moves::<GEN>` already carries the gen1/2/3 arms.
Doubles is a separate, orthogonal extension.

## Known limitation

The Python bindings (`poke-engine-py`) still call the old monomorphic API and are not yet
migrated; their `default = ["poke-engine/gen4"]` feature was removed so the workspace still
resolves. See MIGRATION.md.
