# Poke Engine

An engine for searching through Pokémon battles (singles only).

**This is not a perfect engine**

This battle engine is meant to capture important aspects of Pokémon for the purposes of competitive single battles.
It is nowhere near as complete or robust as the [PokemonShowdown](https://github.com/smogon/pokemon-showdown) battle engine.

## Links

#### [Python Bindings](poke-engine-py)

#### [CHANGELOG](CHANGELOG.md)

## Running Directly

### Building

Make sure you have Rust / Cargo installed.

Generations **4 through 9** are served by a single build (the `genx` engine, generic over
`const GEN: u8`) and the generation is chosen at runtime. Generations **1, 2 and 3** are
separate engines still selected at compile time with a Cargo feature.

```shell
# gens 4-9: one build, pick the generation with --gen at runtime
cargo build --release --no-default-features
./target/release/poke-engine --gen 5 <subcommand> ...

# gens 1/2/3: compile-time selected (unchanged)
make gen1   # or: cargo build --release --no-default-features --features gen1
```

See [the const-generic generations section](#generations-const-generic-genx-engine) below
for the design, and [MIGRATION.md](MIGRATION.md) if you are upgrading a crate that pinned a
`gen4`..`gen9` feature.

### Usage

There are several ways to interact with the engine through subcommands:

1. **Generate Instructions**
```shell
poke-engine generate-instructions --state <state-string> -o <s1_move> -t <s2_move>
```
Generate and display the different Instructions that could be applied to the state if side 1 and side 2 used the given moves.

e.g.
```shell
poke-engine generate-instructions --state <state-string> -o shadowball -t breloom
```
```
Index: 0
StateInstruction: 
	Percentage: 80.00
	Instructions:
		Switch SideTwo: P0 -> P2
		Damage SideTwo: 184

Index: 1
StateInstruction: 
	Percentage: 20.00
	Instructions:
		Switch SideTwo: P0 -> P2
		Damage SideTwo: 184
		Boost SideTwo SpecialDefense: -1
```

2. **Expectiminimax**
```shell
poke-engine expectiminimax --state <state-string> --depth <depth> [--ab-prune]
```
Search through the state using [expectiminimax](https://en.wikipedia.org/wiki/Expectiminimax) to the given depth.
Displays the results along with the best move found.

e.g.
```shell
poke-engine expectiminimax --state <state-string> -d 3
```
```
side one options: psychic,grassknot,shadowball,hiddenpowerfire70,switch skarmory,switch tyranitar,switch mamoswine,switch jellicent,switch excadrill
side two options: closecombat,stoneedge,stealthrock,taunt,xscissor,quickattack,switch lucario,switch breloom,switch keldeo,switch conkeldurr,switch toxicroak
matrix: 32.39,11.99,39.72,99.72,-9.94,69.44,55.46,75.91,75.91,75.91,101.19,32.39,-2.94,39.72,99.72,-28.60,69.44,53.51,79.84,108.92,78.63,-23.62,32.39,-20.35,34.37,94.37,-49.04,49.60,53.51,81.39,88.49,89.01,0.00,17.65,-43.57,11.15,71.15,-72.26,26.38,75.91,75.91,65.27,83.70,0.00,-76.18,-85.66,-72.00,-36.99,-34.19,-34.19,-50.07,-11.07,-25.16,-31.11,15.53,-119.69,-85.88,-101.20,-29.40,-100.00,-82.60,-90.04,-107.86,-77.15,-73.11,-25.90,-100.00,-95.17,-118.42,-75.85,-86.53,-86.53,-97.97,-102.52,-83.18,-74.85,-44.47,-45.01,-74.53,-117.55,-45.01,-56.64,-45.01,-84.08,-120.08,-45.01,-74.85,-44.47,-100.00,-47.20,-96.28,-32.62,-52.23,-42.56,-41.19,-120.08,-74.58,-74.85,-41.19
choice: psychic
evaluation: -9.944763
````

3. **Iterative Deepening**
```shell
poke-engine iterative-deepening --state <state-string> --time-to-search-ms <time>
```
Similar to expectiminimax, search through the state but use iterative deepening.
Searches for the given amount of time, then returns the best move found.

e.g.
```shell
poke-engine iterative-deepening --state <state-string> -t 100
```
```
side one options: psychic,switch jellicent,grassknot,shadowball,hiddenpowerfire70,switch skarmory,switch mamoswine,switch excadrill,switch tyranitar
side two options: closecombat,stoneedge,stealthrock,taunt,xscissor,quickattack,switch lucario,switch breloom,switch keldeo,switch conkeldurr,switch toxicroak
matrix: 32.39,11.99,39.72,99.72,-9.94,69.44,55.46,75.91,75.91,75.91,101.19,-45.01,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,32.39,-2.94,39.72,99.72,-28.60,NaN,NaN,NaN,NaN,NaN,NaN,32.39,-20.35,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,17.65,-43.57,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,-76.18,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,-100.00,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,-100.00,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,-119.69,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN
choice: psychic
evaluation: -9.944763
```

4. **Monte Carlo Tree Search**
```shell
poke-engine monte-carlo-tree-search --state <state-string> --time-to-search-ms <time>
```
Search through the state using [Monte Carlo Tree Search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search) for the given amount of time.

e.g.
```shell
poke-engine monte-carlo-tree-search --state <state-string> -t 100
```
```
Total Iterations: 25000
side one: switch mamoswine,115.31,300|switch tyranitar,41.00,123|hiddenpowerfire70,58.14,165|switch jellicent,1067.52,2402|switch excadrill,3754.58,8173|shadowball,115.37,300|grassknot,298.20,715|psychic,4038.05,8780|switch skarmory,1826.44,4042
side two: stoneedge,915.55,1723|switch lucario,70.53,159|closecombat,827.19,1562|switch breloom,181.84,373|switch keldeo,141.66,297|stealthrock,413.54,805|quickattack,84.78,187|taunt,123.90,263|xscissor,10745.95,19240|switch conkeldurr,153.71,320|switch toxicroak,26.94,71
```

5. **Calculate Damage**
```shell
poke-engine calculate-damage --state <state-string> -o <s1_move> -t <s2_move>
```
Calculate the damage rolls for the given moves.

e.g.
```shell
poke-engine calculate-damage --state <state-string> -o shadowball -t closecombat
```
```
Damage Rolls: 122,123,125,126,128,129,131,132,133,135,136,138,139,141,142,144
Damage Rolls: 155,157,159,161,162,164,166,168,170,172,173,175,177,179,181,183
```

6. **Interactive Mode**: Run the engine and input commands directly

e.g.
```shell
poke-engine --state <state-string>
```

Available commands:

| Command                                               | Shorthand | Function                                                                                                      |
|-------------------------------------------------------|:---------:|---------------------------------------------------------------------------------------------------------------|
| **state** *state-string*                              |     s     | Reset the state to *state-string*                                                                             |
| **matchup**                                           |     m     | Display some information about the current state                                                              |
| **generate-instructions** *side-1-move* *side-2-move* |     g     | Generate all of the instructions that would be applied to the state if side 1 and side 2 used the given moves |
| **instructions**                                      |     i     | Display the last instructions generated by **generate-instructions**                                          |
| **apply** *instruction-index*                         |     a     | Apply the last instructions instructions to the state, modifying it                                           |
| **pop**                                               |     p     | Pops the last instructions from the state, undoing their changes                                              |
| **pop-all**                                           |    pa     | Pops all applied instructions from the state                                                                  |
| **evaluate**                                          |    ev     | Calculate the current state's evaluation                                                                      |
| **calculate-damage** *side-1-move* *side-2-move*      |     d     | Calculate the damage rolls for the given moves                                                                |
| **expectiminimax** *depth* *[ab-prune=false]*         |     e     | Perform expectiminimax (see above), and display the results                                                   |
| **iterative-deepening** *time-ms*                     |    id     | Perform iterative-deepening (see above), and display the results                                              |
| **monte-carlo-tree-search** *time-ms*                 |   mcts    | Perform monte-carlo-tree-search (see above), and display the results                                          |
| **serialize**                                         |    ser    | Display the current state's serialized string                                                                 |
| **exit/quit**                                         |     q     | Quit interactive mode                                                                                         |


### State Representation

When running directly, the engine parses the state of the game from a string.

Properly representing the state of a Pokémon battle gets really complicated.
See the doctest for `State::deserialize` in [state.rs](src/state.rs)
for the source of truth on how to parse a state string.

## Generations (const-generic `genx` engine)

Generations 4-9 share one engine, `src/genx/`, that is generic over a single const
parameter `const GEN: u8`. There is no runtime generation field on `State`, no trait
objects and no enum dispatch inside the engine: each generation is a separate
monomorphization, and every generational difference is a `GEN == N` / `GEN >= N` branch
that LLVM constant-folds per instantiation. A gen-5 build of the hot path is therefore the
same machine code it was when gen 5 was a Cargo feature (measured within ~1.5% of the
pre-conversion engine on the same machine, see the PR description).

### The const parameter

Every hot-path entry point is generic:

```rust
use poke_engine::engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::state::State;

let mut state = State::deserialize::<5>(state_string);      // parse with gen-5 move data
let instructions = generate_instructions_from_move_pair::<5>(&mut state, &s1, &s2, false);
```

Because Rust const generics are not inferred from value arguments, the generation is always
given as a turbofish (`::<5>`). Instantiating with `GEN` outside `4..=9` is a
post-monomorphization compile error (`AssertGenInRange::<GEN>::CHECK`, referenced from the
entry point), not silently-wrong behaviour.

Where a generational difference lives:
- `damage_calc.rs`: two type charts (`type_matchup_table::<GEN>()`), `crit_multiplier::<GEN>()`
  (2.0 for gens 4-5, 1.5 after), terrain boost (`GEN >= 8`), snow/ice defence.
- `generate_instructions.rs`: crit chance, sleep turns, confusion/paralysis/burn residual
  constants become `const fn name::<GEN>()`; sleep, encore, taunt, wish, etc. are `GEN`
  branches; terastallization is `GEN >= 9`.
- `state.rs` (`genx`): `calculate_boosted_stat::<GEN>` (gen-4 Simple stat doubling),
  `can_use_tera::<GEN>` (`GEN >= 9`).
- `abilities.rs` / `items.rs` / `choice_effects.rs`: per-ability / per-item / per-move
  generational tweaks as `GEN` branches.
- `choices.rs`: the ~19k-line move table is built by `add_all_moves::<GEN>`; the genx path
  memoizes one table per generation, lazily, via `moves::<GEN>()` (unused generations are
  never built and cost nothing).

### The runtime facade

Callers that receive the generation as data (a `u8`) use the facade in
[`gen_dispatch`](src/gen_dispatch.rs), which maps the value to the right instantiation once
at the crate edge:

```rust
use poke_engine::gen_dispatch::for_gen;

let mut state = for_gen::deserialize(gen, state_string);           // gen: u8 in 4..=9
let instrs = for_gen::generate_instructions_from_move_pair(gen, &mut state, &s1, &s2, false);
```

The CLI (`poke-engine --gen N ...`) is one such caller: it reads `--gen` and dispatches the
whole command over it. `champions` / `bss` builds always behave as generation 9.

### How gen1/2/3 still work

`gen1/`, `gen2/`, `gen3/` are separate engines (different `MoveChoice`, stub enums) and stay
compile-time selected and mutually exclusive, exactly as before. The crate-root code they
share with genx (`state.rs`, `io.rs`, `mcts.rs`, `search.rs`) is generic over `GEN`; a thin
`gen_dispatch::dispatch::*` shim forwards `GEN` on the genx path and ignores it on the
gen1/2/3 path (where the engine is compile-time fixed), so those modules did not have to
change.

### Phase 2: folding gen1/2/3 behind the same facade

gen1/2/3 are out of scope for the const-generic engine today because they use a smaller
`MoveChoice` and their own, smaller enums (`Abilities`, `Items`, `Weather`, ...). Unifying
them means giving genx's enums a superset that also covers the gen1/2/3 variants (the enums
are already `#[repr(u8)]` with `FromStr`/`From<u8>`, so a superset is additive) and widening
`MoveChoice` to the genx shape. Once the type shapes match, the gen1/2/3 bodies become more
`GEN == N` branches inside the same generic functions (gens become `1..=9`), the separate
`gen1/`, `gen2/`, `gen3/` modules and their features disappear, and `MIN_GEN` drops to 1.
The move table already supports this: `add_all_moves::<GEN>` still contains the gen1/2/3
`GEN == 1|2|3` arms. Doubles support is a separate, orthogonal extension.
