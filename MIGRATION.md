# Migration: per-gen Cargo features to one build with runtime generation

Generations are no longer selected with a Cargo feature. **All nine generations are served
by a single build** and the generation is supplied at each call site (or once, at a runtime
facade). Gens 4-9 come from the `genx` engine, generic over `const GEN: u8`; gens 1-3 keep
their own engine implementations behind the same facade.

## What changed

| Before | After |
| --- | --- |
| `poke-engine = { version = "...", features = ["gen5"] }` | `poke-engine = { version = "..." }` (no gen feature) |
| features `gen4`, `gen5`, `gen6`, `gen7`, `gen8`, `gen9` | removed |
| feature `terastallization` | removed; the behaviour is `GEN >= 9` |
| features `gen1`, `gen2`, `gen3` | removed; gens 1-3 are selected at runtime like the rest |
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

let mut state = for_gen::deserialize(gen, state_str);                     // gen: u8 in 1..=9
let instrs = for_gen::generate_instructions_from_move_pair(gen, &mut state, &s1, &s2, branch);
let (o1, o2) = for_gen::root_get_all_options(gen, &state);
```

`gen` outside `1..=9` panics with a clear message. A `const GEN` outside `4..=9` passed to a
**genx** entry point is a compile error (gens 1-3 have their own engines; go through the
facade or `gen_dispatch::dispatch::*` for them).

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

`choices::MOVES` is gone entirely: every generation's table now comes from
`choices::moves::<GEN>()`, built lazily per generation. Pure-`champions` constants
(`THAW_CHANCE`, `FULLY_PARALYZED_CHANCE`, `SALT_CURE_DAMAGE_DIVISOR`) are unchanged.

## The CLI

The `poke-engine` binary now takes a global `--gen <N>` flag (default: newest supported
generation). All subcommands run under it:

```shell
poke-engine --gen 5 generate-instructions --state <state> -o tackle -t tackle
```

## Python bindings (`poke-engine-py`)

Migrated. The bindings no longer pin a gen feature; they take the generation as an argument.
Every entry point gained a trailing `gen` parameter defaulting to the newest supported
generation, so existing positional calls keep working:

```diff
- state = State.from_string(state_str)
- result = monte_carlo_tree_search(state, duration_ms=1000)
+ state = State.from_string(state_str, gen=5)
+ result = monte_carlo_tree_search(state, duration_ms=1000, gen=5)
```

Affected: `State.from_string`, `mcts` / `monte_carlo_tree_search`, `id` /
`iterative_deepening_expectiminimax`, `generate_instructions`, `calculate_damage`. The module
also exports `MIN_GEN`, `MAX_GEN` and `DEFAULT_GEN`. Passing a generation outside `1..=9`
raises `ValueError`.

Note the default changed in effect: the wheels used to be built with `poke-engine/gen4`, so
the implicit generation was 4. It is now `DEFAULT_GEN` (9). If you relied on gen-4 behaviour,
pass `gen=4` explicitly. Build with plain `maturin develop` (no feature flag).

## Calling generations 1-3

Gens 1-3 no longer need their own build. Use the same facade:

```rust
use poke_engine::gen_dispatch::for_gen;
let mut state = for_gen::deserialize(1, state_str);          // gen 1
let instrs = for_gen::generate_instructions_from_move_pair(1, &mut state, &s1, &s2, false);
```

If you previously built with `--features gen1` and called the engine directly, the gen1/2/3
engine modules are now public as `poke_engine::gen1`, `gen2`, `gen3`. Their methods on the
shared `State`/`Side`/`Pokemon` types are prefixed to avoid clashing with genx's, e.g.
`state.get_all_options()` under `--features gen1` becomes `state.gen1_get_all_options()`
(or, preferably, `gen_dispatch::dispatch::get_all_options::<1>(&state)`).
