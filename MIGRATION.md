# Migration: per-gen Cargo features → const-generic `genx`

Generations 4-9 are no longer selected with a Cargo feature. They are served by a single
build of the `genx` engine, generic over `const GEN: u8`, and the generation is supplied at
each call site (or once, at a runtime facade). Generations 1, 2 and 3 are unchanged: they
remain separate, compile-time-selected engines.

## What changed

| Before | After |
| --- | --- |
| `poke-engine = { version = "...", features = ["gen5"] }` | `poke-engine = { version = "..." }` (no gen feature) |
| features `gen4`, `gen5`, `gen6`, `gen7`, `gen8`, `gen9` | removed |
| feature `terastallization` | removed; the behaviour is `GEN >= 9` |
| features `gen1`, `gen2`, `gen3` | unchanged (still compile-time, still mutually exclusive) |
| features `champions`, `bss` | unchanged (genx-path features; always behave as gen 9) |

Because the engine is compiled once and picks the generation at runtime, the entry points
gained a `const GEN: u8` parameter. Rust cannot infer a const generic from value arguments,
so pass the generation as a turbofish.

## Cargo.toml

```diff
- poke-engine = { version = "0.0.48", features = ["gen5"] }
+ poke-engine = { version = "0.0.48" }
```

If you build with `--no-default-features` and previously added `gen5`, just drop it.

## Call sites

Every engine entry point takes `::<GEN>`:

```diff
- let state = State::deserialize(state_str);
- let instrs = generate_instructions_from_move_pair(&mut state, &s1, &s2, branch);
- let (o1, o2) = state.root_get_all_options();
+ let state = State::deserialize::<5>(state_str);
+ let instrs = generate_instructions_from_move_pair::<5>(&mut state, &s1, &s2, branch);
+ let (o1, o2) = state.root_get_all_options::<5>();
```

Other now-generic methods you may be calling: `State::deserialize`, `Pokemon::replace_move`,
`Side::get_all_options`, `Side::root_get_all_options`, `Side::calculate_boosted_stat`,
`generate_instructions_from_move`, `calculate_damage_rolls`, `calculate_both_damage_rolls`,
`generate_instructions_for_bss_team_preview`. `evaluate`, `Pokemon::has_type`, and all
serialize/`from_str` helpers are NOT generic.

### If your crate pinned one generation (the common case)

Pick the const once and use it everywhere:

```rust
const GEN: u8 = 5;
let state = poke_engine::state::State::deserialize::<GEN>(state_str);
let instrs = generate_instructions_from_move_pair::<GEN>(&mut state, &s1, &s2, branch);
```

This is exactly the pre-conversion behaviour and machine code for that generation, so a crate
that benchmarked `features = ["gen5"]` will get the same performance from `::<5>`
(measured within ~1.5% on the same machine).

### If your crate chooses the generation at runtime

Use the facade, which matches the `u8` to the right instantiation once:

```rust
use poke_engine::gen_dispatch::for_gen;

let mut state = for_gen::deserialize(gen, state_str);                     // gen: u8 in 4..=9
let instrs = for_gen::generate_instructions_from_move_pair(gen, &mut state, &s1, &s2, branch);
let (o1, o2) = for_gen::root_get_all_options(gen, &state);
```

`gen` outside `4..=9` panics with a clear message; a `const GEN` outside `4..=9` is a compile
error.

## Constants that became functions

Some public constants were per-gen and are now `const fn`s of `GEN`:

| Before | After |
| --- | --- |
| `damage_calc::CRIT_MULTIPLIER` | `damage_calc::crit_multiplier::<GEN>()` |
| `generate_instructions::BASE_CRIT_CHANCE` | `generate_instructions::base_crit_chance::<GEN>()` |
| `generate_instructions::MAX_SLEEP_TURNS` | `generate_instructions::max_sleep_turns::<GEN>()` |
| `generate_instructions::BURN_RESIDUAL_DAMAGE_PCT` | `...::burn_residual_damage_pct::<GEN>()` |
| `generate_instructions::CONSECUTIVE_PROTECT_CHANCE` | `...::consecutive_protect_chance::<GEN>()` |
| `abilities::WEATHER_ABILITY_TURNS` | `abilities::weather_ability_turns::<GEN>()` |
| `choices::MOVES` (genx) | `choices::moves::<GEN>()` |

`choices::MOVES` still exists as a static for the gen1/2/3 builds. Pure-`champions`
constants (`THAW_CHANCE`, `FULLY_PARALYZED_CHANCE`, `SALT_CURE_DAMAGE_DIVISOR`) are unchanged.

## The CLI

The `poke-engine` binary now takes a global `--gen <N>` flag (default: newest supported
generation). All subcommands run under it:

```shell
poke-engine --gen 5 generate-instructions --state <state> -o tackle -t tackle
```

## Python bindings (`poke-engine-py`)

Not yet migrated. They still call the old monomorphic API and must be updated to the
const-generic / facade API (thread a chosen `GEN`, or expose the generation to Python) before
they will build again. Their `default = ["poke-engine/gen4"]` feature was removed so the
workspace still resolves.
