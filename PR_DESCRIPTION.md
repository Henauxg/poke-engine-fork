# Serve gens 1-9 from one library, generation chosen at runtime

Converts the `genx` engine (generations 4-9) from six mutually-exclusive Cargo features
into a single library generic over `const GEN: u8`, with zero hot-path cost, and then folds
generations 1-3 into the same build so **one binary serves all nine generations, selected at
runtime**. Generation differences that were `#[cfg(feature = "genN")]` are now `GEN == N` /
`GEN >= N` branches that LLVM constant-folds per instantiation: no runtime generation field
on `State`, no trait objects, no enum dispatch inside the engine. **All per-generation Cargo
features are gone.**

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
- **Generations 1-3 in the same build.** Two changes made the four engines coexist:
  *unified enums* (`Abilities`, `Items`, `Weather`, `Terrain`, `PokemonVolatileStatus`,
  `MoveChoice` now have one definition, genx's, which every engine re-exports) and
  *prefixed engine methods* (all four engines added same-named inherent methods to the
  shared `State`/`Side`/`Pokemon`; gen1/2/3's now carry a `gen1_`/`gen2_`/`gen3_` prefix).
  `gen_dispatch::dispatch::*` routes by `GEN` and folds away completely.
- The `terastallization` feature is gone; its mechanics are `GEN >= 9`. `champions`/`bss`
  remain ordinary features, orthogonal to gen, mapped to gen-9 behaviour.

See the README "Generations (one build, runtime selection)" section and MIGRATION.md.

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

## End-to-end proof of runtime generation selection

A single binary, same state and move pair, generation chosen at runtime:

```
$ poke-engine --gen 4 generate-instructions --state <state> -o psyshock -t bravebird
        Percentage: 93.75
$ poke-engine --gen 5 ...
        Percentage: 93.75
$ poke-engine --gen 9 ...
        Percentage: 95.83333
$ poke-engine --gen 0 ...
unsupported generation 0: this build serves 1..=9
```

That is `base_crit_chance::<GEN>()` (1/16 through gen 6, 1/24 from gen 7) resolving
per-instantiation inside one build.

## Tests: one `cargo test` covers every generation

Each genx test file re-includes its shared test bodies (in `tests/impls/`, or a sibling
`*_test_bodies.rs` for the inline unit-test modules) once per generation via a `gen_tests!`
macro (`genN::test_x`), so gens 4-9 are all exercised. Per-gen expected values that were
`#[cfg]`-gated became `if GEN == N` branches; tests that only apply to some gens
early-return for the others. The gen1/2/3 suites run against their own engines in the same
command, which previously required three separate feature builds.

| suite | result |
| --- | --- |
| lib unit tests (gens 4-9) | 1375 passed |
| `test_battle_mechanics` (gens 4-9) | 4194 passed |
| `test_last_used_move` (gens 4-9) | 102 passed |
| `test_damage_dealt` (gens 4-9) | 90 passed |
| `test_gen1` / `test_gen2` / `test_gen3` | 42 / 35 / 15 passed |
| `test_gen_dispatch` (facade, gens 1-9) | 4 passed |
| doctest | 1 passed |
| **total, one `cargo test`** | **5858 passed, 0 failed** |
| `--features champions` (runs as gen 9) | all passed |

`tests/test_gen_dispatch.rs` is a new regression test that round-trips a state, produces
root options, evaluates, and generates instructions for **every** generation 1-9 through the
facade, and asserts an out-of-range generation is rejected.

## clippy

The conversion is clippy-neutral: the base commit already emits ~332 `cargo clippy` warnings
(pre-existing style lints across the engine); this branch emits a comparable count and the
new code (`gen_dispatch.rs`, the `const fn` conversions, the dispatch shim) is clippy-clean.
The pre-existing warnings are intentionally left untouched to keep the diff mechanical and
avoid reformatting code this change does not otherwise touch.

## What is left

Generations 1-3 are now in the same build but still separate implementations behind the
facade. Making them literal `GEN <= 3` branches inside the generic functions (deleting the
`gen1/`, `gen2/`, `gen3/` modules) is the remaining cleanup. The groundwork is done: shared
types, a shared move table via `add_all_moves::<GEN>`, and a single dispatch point. It is a
four-way merge of independently written bodies though, and the gen1/2/3 suites (42/35/15
tests) are a far thinner safety net than the genx mechanics suite (699 tests x 6
generations), so expanding that coverage should come first. Doubles is a separate,
orthogonal extension.

## Python bindings

Migrated too. The `PyState -> State` conversion chain became generic inherent methods
(`into_state::<GEN>` / `into_side` / `into_pokemon` / `into_move`) because a move's `Choice`
data is per-generation and a plain `Into` impl cannot take `GEN`. Each entry point gained a
trailing `gen` argument (defaulting to `DEFAULT_GEN`) and dispatches the runtime value to the
right monomorphization, so existing positional Python calls keep working. The module exports
`MIN_GEN`/`MAX_GEN`/`DEFAULT_GEN`, and an out-of-range generation raises `ValueError`.

Behaviour note: the wheels used to be built with `poke-engine/gen4`, so the implicit
generation was 4; it is now 9 unless `gen=4` is passed. Documented in MIGRATION.md.

## Folding in generations 1-3

The enum union needed only **five** extra variants, so it is additive: `CACOPHONY` (gen3),
`MINTBERRY` / `MIRACLEBERRY` (gen2), `GEN1BURNNULLIFY` / `GEN1PARALYSISNULLIFY` (gen1). They
are appended rather than inserted so existing discriminants, and therefore the
`VolatileStatusBitset` bit indices, do not shift (106 variants still fit in `u128`).

The real obstacle was not the enums but that all four engines add **same-named inherent
methods** to the shared `State`/`Side`/`Pokemon` types (`get_all_options`,
`calculate_boosted_stat`, `has_type`, ...), which only works while one engine is compiled.
gen1/2/3's methods are now prefixed. They were renamed rather than deleted even where the
body matched genx's, because an identical-looking method can call a *differing* one and
would otherwise silently bind to genx's version.

One subtlety worth noting: `dispatch` uses literal consts (`::<4>` .. `::<9>`) for the genx
arms rather than `::<GEN>`. Monomorphization happens before the `match GEN` folds, so
`::<GEN>` would instantiate genx with `GEN = 1..3` on the unreachable fallthrough and trip
genx's `AssertGenInRange` guard.

`cargo test` now runs **every** generation in one command (previously gens 1, 2 and 3 each
needed their own feature build).
