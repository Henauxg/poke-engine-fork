# Move implementation audit (gens 1-9)

## Summary

Audited **885** `Choices` variants in poke-engine against declarative move data (`src/choices.rs`), special-case handlers (`choice_effects` / `generate_instructions` / `damage_calc` per generation), and [Pokemon Showdown `moves.json`](https://play.pokemonshowdown.com/data/moves.json) as a machine-readable effect oracle. Documentation links point at [Bulbapedia](https://bulbapedia.bulbagarden.net/) move pages.

Entries are sorted by **criticality for MCTS / statistical simulation** (the engine's real use case: many rollouts to pick a move mid-fight). Losing variance is cheap when the mean is right; systematic bias or a blank competitive option is expensive.

| Metric | Count |
|---|---:|
| Total `Choices` | 885 |
| Looks OK (declarative and/or special-cased) | ~771* |
| Flagged incomplete / approximate / shell | **113** |
| Doubles/out-of-scope stubs | 8 |

\*OK count is approximate: a move with correct BP/flags can still miss a niche interaction not covered by this static audit.

### Counts by MCTS criticality

| Priority | Meaning | Count |
|---|---|---:|
| **P0** | Critical for MCTS | 2 |
| **P1** | High (systematic bias / field state) | 16 |
| **P2** | Medium (niche / situational) | 60 |
| **P3** | Low (near-unbiased approximations) | 21 |
| **P4** | Negligible / out of scope | 22 |

### Criticality rubric

| Priority | When to care |
|---|---|
| **P0** | Real singles options that are non-functional or drastically wrong (search will never value them correctly). |
| **P1** | Systematic optimistic/pessimistic bias on used moves, or missing field/type/trap state that changes later EV. |
| **P2** | Wrong, but niche / situational for the formats this engine targets. |
| **P3** | Near-unbiased statistical shortcuts (e.g. 2-5 hits -> always 3 ~ mean ~3.2). Fine for MCTS; branching is polish. |
| **P4** | Gimmicks, OHKOs, doubles-only, or essentially unused in singles search. |

**Example:** fixing "2-5 hit moves always roll 3" is low value for MCTS because the mean is already right. Fixing Rage Fist (always BP 50) or terrain *moves* (no-op) changes which lines the search believes are good.

### Counts by mechanical class (secondary)

| Class | Count |
|---|---:|
| Known approximations (implemented, but simplified) | 24 |
| Partial: variable base power missing or wrong | 4 |
| Partial: consecutive-use / ramp moves | 3 |
| Partial: fail conditions missing | 1 |
| Partial: other incomplete mechanics | 7 |
| OHKO moves (no KO effect) | 4 |
| Fixed / variable damage shells (no damage logic) | 13 |
| Field-control moves (terrain / rooms / gravity / sports) | 11 |
| Type / ability changing moves | 14 |
| Copy / transform / random-call moves | 9 |
| Trapping / Lock-On style moves | 5 |
| Stat-control / swap / split moves | 10 |
| Other status shells | 7 |
| Other shells | 1 |
| Doubles / team-support (out of scope for singles engine) | 8 |

## Methodology

1. Enumerate every `Choices::{MOVE}` and its `moves.insert` `Choice` blocks.
2. Treat a move as a **shell** if it has no effectful declarative fields (`base_power>0`, boosts, status, volatile, side condition, heal, drain, secondaries) and no `Choices::MOVE` match arm in gen1/2/3/x `choice_effects`, `generate_instructions`, or `damage_calc`.
3. Collect **documented approximations** from code comments (multi-hit average, locked-move duration, Population Bomb, etc.).
4. Manually add **partial** implementations that have table data / BP but are missing the characteristic mechanic (Flail vs Reversal, Rage Fist, Beat Up, Rollout, ...).
5. Cross-check names/gens via Showdown `moves.json`; link each entry to Bulbapedia.
6. Rank for **MCTS criticality**: prefer fixing blank/biased competitive options over recovering variance around a correct mean.

### False positives removed

- **Roar / Whirlwind**: implemented via `flags.drag` in `src/genx/generate_instructions.rs`.
- **Teleport**: `flags.pivot` in gen 8+; earlier generations correctly do nothing in battle.
- Intentional no-ops: Splash, Celebrate, Happy Hour, Hold Hands, `NONE`/`NOTHING`.

### Generation coverage notes

- Move **data** for all gens lives in shared `src/choices.rs` (`add_all_moves::<GEN>`).
- **Mechanics** for gens 4-9 share `src/genx/`; gens 1-3 use `src/gen1|gen2|gen3/`.
- Terrain **abilities** (Electric/Grassy/Psychic/Misty Surge) can set terrain in genx; terrain **moves** were shells in the original audit and have since been implemented on some branches (still worth checking duration below).

### Weather / terrain duration and extender items

In real Pokemon, duration can be extended by held items:

- **Weather moves / weather abilities:** Heat Rock, Damp Rock, Smooth Rock, Icy Rock make sun/rain/sand/hail last **8** turns instead of **5**.
- **Terrain moves / surge abilities:** Terrain Extender makes terrain last **8** turns instead of **5**.

poke-engine hardcodes **5** turns for weather and terrain, same as Rain Dance / Sunny Day and Electric Surge / Grassy Surge in this repo. Those extender items are **not implemented here at all** (no item enums, no duration branching).

For MCTS this is usually a mild optimistic/pessimistic bias on residual turns rather than a blank move, but it matters for long weather/terrain offense (e.g. sand teams, Expanding Force turns remaining).

---

## P0 - Critical for MCTS

Common (or format-relevant) singles options that are non-functional or drastically wrong. Search will systematically mis-rank these lines.

_2 moves_

### Flail (`FLAIL`)

- **MCTS criticality:** P0 - Deals 0 damage while Reversal works; any Flail user is treated as having a blank move.
- **Generations:** 2-9
- **Mechanical class:** Partial: variable base power missing or wrong
- **What is wrong:** HP-ratio base power is not implemented (unlike Reversal, which is special-cased in `modify_choice`). Table BP is 0, so the move deals no damage.
- **Easy to fix?** easy
- **Fix hint:** Mirror the existing `Choices::REVERSAL` branch in `src/genx/choice_effects.rs`.
- **Bulbapedia:** [Flail](https://bulbapedia.bulbagarden.net/wiki/Flail_(move))
- **Move table:** [`src/choices.rs:5493`](src/choices.rs#L5493) (`Choices::FLAIL`)
- **Special-case handlers:** _none_

### Rage Fist (`RAGEFIST`)

- **MCTS criticality:** P0 - Annihilape staple; fixed BP 50 instead of hit-scaling makes the whole mon mis-evaluated.
- **Generations:** 9
- **Mechanical class:** Partial: variable base power missing or wrong
- **What is wrong:** Always BP 50. Does not add +50 per time the user was hit (cap 350).
- **Easy to fix?** medium
- **Fix hint:** Needs a hit-counter on the Pokemon plus `modify_choice` scaling.
- **Bulbapedia:** [Rage Fist](https://bulbapedia.bulbagarden.net/wiki/Rage_Fist_(move))
- **Move table:** [`src/choices.rs:12871`](src/choices.rs#L12871) (`Choices::RAGEFIST`)
- **Special-case handlers:** _none_

## P1 - High (systematic bias / field state)

Optimistic or pessimistic bias on used moves, or missing field effects that change later damage. Means of rollouts are skewed, not just noisy.

_16 moves_

### Block (`BLOCK`)

- **MCTS criticality:** P1 - Trapping not applied; phantom switches stay legal in search.
- **Generations:** 3-9
- **Mechanical class:** Trapping / Lock-On style moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Block](https://bulbapedia.bulbagarden.net/wiki/Block_(move))
- **Move table:** [`src/choices.rs:1484`](src/choices.rs#L1484) (`Choices::BLOCK`)
- **Special-case handlers:** _none_

### Electric Terrain (`ELECTRICTERRAIN`)

- **MCTS criticality:** P1 - Move does not set terrain (abilities like Electric Surge still do). Move-based setters never enable Expanding Force / Rising Voltage / terrain mods in search.
- **Generations:** 6-9
- **Mechanical class:** Field-control moves (terrain / rooms / gravity / sports)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Electric Terrain](https://bulbapedia.bulbagarden.net/wiki/Electric_Terrain_(move))
- **Move table:** [`src/choices.rs:4531`](src/choices.rs#L4531) (`Choices::ELECTRICTERRAIN`)
- **Special-case handlers:** _none_

### Forest's Curse (`FORESTSCURSE`)

- **MCTS criticality:** P1 - Type-changing support can enable KO lines; missing effect hides those futures.
- **Generations:** 6-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Forest's Curse](https://bulbapedia.bulbagarden.net/wiki/Forest%27s_Curse_(move))
- **Move table:** [`src/choices.rs:5990`](src/choices.rs#L5990) (`Choices::FORESTSCURSE`)
- **Special-case handlers:** _none_

### Grassy Terrain (`GRASSYTERRAIN`)

- **MCTS criticality:** P1 - Move is a no-op; Grassy Surge still works. Same search gap for move-based setters.
- **Generations:** 6-9
- **Mechanical class:** Field-control moves (terrain / rooms / gravity / sports)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Grassy Terrain](https://bulbapedia.bulbagarden.net/wiki/Grassy_Terrain_(move))
- **Move table:** [`src/choices.rs:6691`](src/choices.rs#L6691) (`Choices::GRASSYTERRAIN`)
- **Special-case handlers:** _none_

### Lock-On (`LOCKON`)

- **MCTS criticality:** P1 - Missing trap/lock-on lets the search over-value illegal switches or miss sure hits.
- **Generations:** 2-9
- **Mechanical class:** Trapping / Lock-On style moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Lock-On](https://bulbapedia.bulbagarden.net/wiki/Lock-On_(move))
- **Move table:** [`src/choices.rs:9457`](src/choices.rs#L9457) (`Choices::LOCKON`)
- **Special-case handlers:** _none_

### Magic Powder (`MAGICPOWDER`)

- **MCTS criticality:** P1 - Type-changing support can enable KO lines; missing effect hides those futures.
- **Generations:** 8-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Magic Powder](https://bulbapedia.bulbagarden.net/wiki/Magic_Powder_(move))
- **Move table:** [`src/choices.rs:9803`](src/choices.rs#L9803) (`Choices::MAGICPOWDER`)
- **Special-case handlers:** _none_

### Mean Look (`MEANLOOK`)

- **MCTS criticality:** P1 - Trapping not applied; phantom switches stay legal in search.
- **Generations:** 2-9
- **Mechanical class:** Trapping / Lock-On style moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Mean Look](https://bulbapedia.bulbagarden.net/wiki/Mean_Look_(move))
- **Move table:** [`src/choices.rs:10027`](src/choices.rs#L10027) (`Choices::MEANLOOK`)
- **Special-case handlers:** _none_

### Mind Reader (`MINDREADER`)

- **MCTS criticality:** P1 - Missing trap/lock-on lets the search over-value illegal switches or miss sure hits.
- **Generations:** 2-9
- **Mechanical class:** Trapping / Lock-On style moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Mind Reader](https://bulbapedia.bulbagarden.net/wiki/Mind_Reader_(move))
- **Move table:** [`src/choices.rs:10395`](src/choices.rs#L10395) (`Choices::MINDREADER`)
- **Special-case handlers:** _none_

### Misty Terrain (`MISTYTERRAIN`)

- **MCTS criticality:** P1 - Move is a no-op; Misty Surge still works.
- **Generations:** 6-9
- **Mechanical class:** Field-control moves (terrain / rooms / gravity / sports)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Misty Terrain](https://bulbapedia.bulbagarden.net/wiki/Misty_Terrain_(move))
- **Move table:** [`src/choices.rs:10590`](src/choices.rs#L10590) (`Choices::MISTYTERRAIN`)
- **Special-case handlers:** _none_

### Population Bomb (`POPULATIONBOMB`)

- **MCTS criticality:** P1 - Always 6 (or 9) hits; real multi-accuracy EV is lower. Optimistic damage bias on a used move.
- **Generations:** 9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** Multi-accuracy not implemented; approximated as fixed 6 hits (9 with Wide Lens)
- **Easy to fix?** medium
- **Bulbapedia:** [Population Bomb](https://bulbapedia.bulbagarden.net/wiki/Population_Bomb_(move))
- **Move table:** [`src/choices.rs:12118`](src/choices.rs#L12118) (`Choices::POPULATIONBOMB`)
- **Special-case handlers:** _none_

### Psychic Terrain (`PSYCHICTERRAIN`)

- **MCTS criticality:** P1 - Move is a no-op; Psychic Surge still works.
- **Generations:** 7-9
- **Mechanical class:** Field-control moves (terrain / rooms / gravity / sports)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Psychic Terrain](https://bulbapedia.bulbagarden.net/wiki/Psychic_Terrain_(move))
- **Move table:** [`src/choices.rs:12547`](src/choices.rs#L12547) (`Choices::PSYCHICTERRAIN`)
- **Special-case handlers:** _none_

### Soak (`SOAK`)

- **MCTS criticality:** P1 - Type change missing; breaks Water-type tricks / Immuno / coverage interactions in search.
- **Generations:** 5-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Soak](https://bulbapedia.bulbagarden.net/wiki/Soak_(move))
- **Move table:** [`src/choices.rs:15244`](src/choices.rs#L15244) (`Choices::SOAK`)
- **Special-case handlers:** _none_

### Spider Web (`SPIDERWEB`)

- **MCTS criticality:** P1 - Trapping not applied; phantom switches stay legal in search.
- **Generations:** 2-9
- **Mechanical class:** Trapping / Lock-On style moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Spider Web](https://bulbapedia.bulbagarden.net/wiki/Spider_Web_(move))
- **Move table:** [`src/choices.rs:15441`](src/choices.rs#L15441) (`Choices::SPIDERWEB`)
- **Special-case handlers:** _none_

### Trick-or-Treat (`TRICKORTREAT`)

- **MCTS criticality:** P1 - Type-changing support can enable KO lines; missing effect hides those futures.
- **Generations:** 6-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Trick-or-Treat](https://bulbapedia.bulbagarden.net/wiki/Trick-or-Treat_(move))
- **Move table:** [`src/choices.rs:17668`](src/choices.rs#L17668) (`Choices::TRICKORTREAT`)
- **Special-case handlers:** _none_

### Triple Axel (`TRIPLEAXEL`)

- **MCTS criticality:** P1 - Always lands all three escalating hits; real multi-accuracy often stops early. Optimistic bias.
- **Generations:** 8-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** Multi-accuracy not implemented; always lands all 3 hits at escalating BP
- **Easy to fix?** medium
- **Bulbapedia:** [Triple Axel](https://bulbapedia.bulbagarden.net/wiki/Triple_Axel_(move))
- **Move table:** [`src/choices.rs:17726`](src/choices.rs#L17726) (`Choices::TRIPLEAXEL`)
- **Special-case handlers:** _none_

### Triple Kick (`TRIPLEKICK`)

- **MCTS criticality:** P1 - Same multi-accuracy optimism as Triple Axel (less common modernly).
- **Generations:** 2-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** Multi-accuracy not implemented; treated similarly to multi-hit
- **Easy to fix?** medium
- **Bulbapedia:** [Triple Kick](https://bulbapedia.bulbagarden.net/wiki/Triple_Kick_(move))
- **Move table:** [`src/choices.rs:17758`](src/choices.rs#L17758) (`Choices::TRIPLEKICK`)
- **Special-case handlers:** _none_

## P2 - Medium (niche / situational)

Wrong or incomplete, but rarely chosen in the formats this engine targets, or only matters in narrow states.

_60 moves_

### Acupressure (`ACUPRESSURE`)

- **MCTS criticality:** P2 - Swap/split/acupressure-style tools are rare in standard singles search.
- **Generations:** 4-9
- **Mechanical class:** Stat-control / swap / split moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Acupressure](https://bulbapedia.bulbagarden.net/wiki/Acupressure_(move))
- **Move table:** [`src/choices.rs:204`](src/choices.rs#L204) (`Choices::ACUPRESSURE`)
- **Special-case handlers:** _none_

### Beat Up (`BEATUP`)

- **MCTS criticality:** P2 - Variable BP wrong/zero; impact depends on whether the move is actually used.
- **Generations:** 2-9
- **Mechanical class:** Partial: variable base power missing or wrong
- **What is wrong:** Gen 2-4 uses a single fixed BP 10 hit (not one hit per healthy ally). Gen 5+ has `base_power: 0` and no ally-Attack scaling, so it deals no damage.
- **Easy to fix?** medium
- **Bulbapedia:** [Beat Up](https://bulbapedia.bulbagarden.net/wiki/Beat_Up_(move))
- **Move table:** [`src/choices.rs:1084`](src/choices.rs#L1084) (`Choices::BEATUP`)
- **Special-case handlers:** _none_

### Bestow (`BESTOW`)

- **MCTS criticality:** P2 - Status shell; only matters if that exact utility is on the moveset.
- **Generations:** 5-9
- **Mechanical class:** Other status shells
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** unknown
- **Bulbapedia:** [Bestow](https://bulbapedia.bulbagarden.net/wiki/Bestow_(move))
- **Move table:** [`src/choices.rs:1171`](src/choices.rs#L1171) (`Choices::BESTOW`)
- **Special-case handlers:** _none_

### Bide (`BIDE`)

- **MCTS criticality:** P2 - Incomplete multi-turn / stockpile-style mechanic; situational.
- **Generations:** 1-4
- **Mechanical class:** Partial: other incomplete mechanics
- **What is wrong:** Sets `BIDE` volatile, but there is no genx damage-store / double-return implementation.
- **Easy to fix?** hard
- **Bulbapedia:** [Bide](https://bulbapedia.bulbagarden.net/wiki/Bide_(move))
- **Move table:** [`src/choices.rs:1182`](src/choices.rs#L1182) (`Choices::BIDE`)
- **Special-case handlers:** _none_

### Camouflage (`CAMOUFLAGE`)

- **MCTS criticality:** P2 - Ability/type gizmos are situational; wrong if chosen, rarely central to MCTS policy.
- **Generations:** 3-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Camouflage](https://bulbapedia.bulbagarden.net/wiki/Camouflage_(move))
- **Move table:** [`src/choices.rs:2189`](src/choices.rs#L2189) (`Choices::CAMOUFLAGE`)
- **Special-case handlers:** _none_

### Conversion (`CONVERSION`)

- **MCTS criticality:** P2 - Ability/type gizmos are situational; wrong if chosen, rarely central to MCTS policy.
- **Generations:** 1-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Conversion](https://bulbapedia.bulbagarden.net/wiki/Conversion_(move))
- **Move table:** [`src/choices.rs:2819`](src/choices.rs#L2819) (`Choices::CONVERSION`)
- **Special-case handlers:** _none_

### Conversion 2 (`CONVERSION2`)

- **MCTS criticality:** P2 - Ability/type gizmos are situational; wrong if chosen, rarely central to MCTS policy.
- **Generations:** 2-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Conversion 2](https://bulbapedia.bulbagarden.net/wiki/Conversion_2_(move))
- **Move table:** [`src/choices.rs:2831`](src/choices.rs#L2831) (`Choices::CONVERSION2`)
- **Special-case handlers:** _none_

### Corrosive Gas (`CORROSIVEGAS`)

- **MCTS criticality:** P2 - Status shell; only matters if that exact utility is on the moveset.
- **Generations:** 8-9
- **Mechanical class:** Other status shells
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** unknown
- **Bulbapedia:** [Corrosive Gas](https://bulbapedia.bulbagarden.net/wiki/Corrosive_Gas_(move))
- **Move table:** [`src/choices.rs:2868`](src/choices.rs#L2868) (`Choices::CORROSIVEGAS`)
- **Special-case handlers:** _none_

### Crush Grip (`CRUSHGRIP`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 4-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Crush Grip](https://bulbapedia.bulbagarden.net/wiki/Crush_Grip_(move))
- **Move table:** [`src/choices.rs:3216`](src/choices.rs#L3216) (`Choices::CRUSHGRIP`)
- **Special-case handlers:** _none_

### Doodle (`DOODLE`)

- **MCTS criticality:** P2 - Ability/type gizmos are situational; wrong if chosen, rarely central to MCTS policy.
- **Generations:** 9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Doodle](https://bulbapedia.bulbagarden.net/wiki/Doodle_(move))
- **Move table:** [`src/choices.rs:3762`](src/choices.rs#L3762) (`Choices::DOODLE`)
- **Special-case handlers:** _none_

### Dragon Rage (`DRAGONRAGE`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 1-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Dragon Rage](https://bulbapedia.bulbagarden.net/wiki/Dragon_Rage_(move))
- **Move table:** [`src/choices.rs:4172`](src/choices.rs#L4172) (`Choices::DRAGONRAGE`)
- **Special-case handlers:** _none_

### Entrainment (`ENTRAINMENT`)

- **MCTS criticality:** P2 - Ability/type gizmos are situational; wrong if chosen, rarely central to MCTS policy.
- **Generations:** 5-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Entrainment](https://bulbapedia.bulbagarden.net/wiki/Entrainment_(move))
- **Move table:** [`src/choices.rs:4770`](src/choices.rs#L4770) (`Choices::ENTRAINMENT`)
- **Special-case handlers:** _none_

### Fairy Lock (`FAIRYLOCK`)

- **MCTS criticality:** P2 - Field sport / niche control; rarely the best action, but currently a no-op if chosen.
- **Generations:** 6-9
- **Mechanical class:** Field-control moves (terrain / rooms / gravity / sports)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Fairy Lock](https://bulbapedia.bulbagarden.net/wiki/Fairy_Lock_(move))
- **Move table:** [`src/choices.rs:4934`](src/choices.rs#L4934) (`Choices::FAIRYLOCK`)
- **Special-case handlers:** _none_

### Fling (`FLING`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 4-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Fling](https://bulbapedia.bulbagarden.net/wiki/Fling_(move))
- **Move table:** [`src/choices.rs:5734`](src/choices.rs#L5734) (`Choices::FLING`)
- **Special-case handlers:** _none_

### Fury Cutter (`FURYCUTTER`)

- **MCTS criticality:** P2 - Ramp moves are niche; first-hit BP may still be non-zero so not fully blank.
- **Generations:** 2-9
- **Mechanical class:** Partial: consecutive-use / ramp moves
- **What is wrong:** Static BP only; successive-use doubling is not modeled.
- **Easy to fix?** medium
- **Bulbapedia:** [Fury Cutter](https://bulbapedia.bulbagarden.net/wiki/Fury_Cutter_(move))
- **Move table:** [`src/choices.rs:6172`](src/choices.rs#L6172) (`Choices::FURYCUTTER`)
- **Special-case handlers:** _none_

### Gravity (`GRAVITY`)

- **MCTS criticality:** P2 - Uncommon in singles but can change immunities / item / speed interactions when used.
- **Generations:** 4-9
- **Mechanical class:** Field-control moves (terrain / rooms / gravity / sports)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Gravity](https://bulbapedia.bulbagarden.net/wiki/Gravity_(move))
- **Move table:** [`src/choices.rs:6728`](src/choices.rs#L6728) (`Choices::GRAVITY`)
- **Special-case handlers:** _none_

### Guard Split (`GUARDSPLIT`)

- **MCTS criticality:** P2 - Swap/split/acupressure-style tools are rare in standard singles search.
- **Generations:** 5-9
- **Mechanical class:** Stat-control / swap / split moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Guard Split](https://bulbapedia.bulbagarden.net/wiki/Guard_Split_(move))
- **Move table:** [`src/choices.rs:6829`](src/choices.rs#L6829) (`Choices::GUARDSPLIT`)
- **Special-case handlers:** _none_

### Guard Swap (`GUARDSWAP`)

- **MCTS criticality:** P2 - Swap/split/acupressure-style tools are rare in standard singles search.
- **Generations:** 4-9
- **Mechanical class:** Stat-control / swap / split moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Guard Swap](https://bulbapedia.bulbagarden.net/wiki/Guard_Swap_(move))
- **Move table:** [`src/choices.rs:6841`](src/choices.rs#L6841) (`Choices::GUARDSWAP`)
- **Special-case handlers:** _none_

### Heart Swap (`HEARTSWAP`)

- **MCTS criticality:** P2 - Swap/split/acupressure-style tools are rare in standard singles search.
- **Generations:** 4-9
- **Mechanical class:** Stat-control / swap / split moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Heart Swap](https://bulbapedia.bulbagarden.net/wiki/Heart_Swap_(move))
- **Move table:** [`src/choices.rs:7241`](src/choices.rs#L7241) (`Choices::HEARTSWAP`)
- **Special-case handlers:** _none_

### Ice Ball (`ICEBALL`)

- **MCTS criticality:** P2 - Ramp moves are niche; first-hit BP may still be non-zero so not fully blank.
- **Generations:** 3-9
- **Mechanical class:** Partial: consecutive-use / ramp moves
- **What is wrong:** Same incomplete ramp/lock behavior as Rollout.
- **Easy to fix?** medium
- **Bulbapedia:** [Ice Ball](https://bulbapedia.bulbagarden.net/wiki/Ice_Ball_(move))
- **Move table:** [`src/choices.rs:8258`](src/choices.rs#L8258) (`Choices::ICEBALL`)
- **Special-case handlers:** _none_

### Ion Deluge (`IONDELUGE`)

- **MCTS criticality:** P2 - Field sport / niche control; rarely the best action, but currently a no-op if chosen.
- **Generations:** 6-9
- **Mechanical class:** Field-control moves (terrain / rooms / gravity / sports)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Ion Deluge](https://bulbapedia.bulbagarden.net/wiki/Ion_Deluge_(move))
- **Move table:** [`src/choices.rs:8657`](src/choices.rs#L8657) (`Choices::IONDELUGE`)
- **Special-case handlers:** _none_

### Last Resort (`LASTRESORT`)

- **MCTS criticality:** P2 - Last Resort always available overvalues a niche move when other moves exist.
- **Generations:** 4-9
- **Mechanical class:** Partial: fail conditions missing
- **What is wrong:** Always usable at full BP. Does not require all other moves to have been used.
- **Easy to fix?** hard
- **Fix hint:** Needs per-Pokemon used-move tracking.
- **Bulbapedia:** [Last Resort](https://bulbapedia.bulbagarden.net/wiki/Last_Resort_(move))
- **Move table:** [`src/choices.rs:9043`](src/choices.rs#L9043) (`Choices::LASTRESORT`)
- **Special-case handlers:** _none_

### Magic Room (`MAGICROOM`)

- **MCTS criticality:** P2 - Uncommon in singles but can change immunities / item / speed interactions when used.
- **Generations:** 5-9
- **Mechanical class:** Field-control moves (terrain / rooms / gravity / sports)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Magic Room](https://bulbapedia.bulbagarden.net/wiki/Magic_Room_(move))
- **Move table:** [`src/choices.rs:9817`](src/choices.rs#L9817) (`Choices::MAGICROOM`)
- **Special-case handlers:** _none_

### Magnitude (`MAGNITUDE`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 2-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Magnitude](https://bulbapedia.bulbagarden.net/wiki/Magnitude_(move))
- **Move table:** [`src/choices.rs:9932`](src/choices.rs#L9932) (`Choices::MAGNITUDE`)
- **Special-case handlers:** _none_

### Mud Sport (`MUDSPORT`)

- **MCTS criticality:** P2 - Field sport / niche control; rarely the best action, but currently a no-op if chosen.
- **Generations:** 3-9
- **Mechanical class:** Field-control moves (terrain / rooms / gravity / sports)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Mud Sport](https://bulbapedia.bulbagarden.net/wiki/Mud_Sport_(move))
- **Move table:** [`src/choices.rs:10853`](src/choices.rs#L10853) (`Choices::MUDSPORT`)
- **Special-case handlers:** _none_

### Natural Gift (`NATURALGIFT`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 4-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Natural Gift](https://bulbapedia.bulbagarden.net/wiki/Natural_Gift_(move))
- **Move table:** [`src/choices.rs:11002`](src/choices.rs#L11002) (`Choices::NATURALGIFT`)
- **Special-case handlers:** _none_

### Nature Power (`NATUREPOWER`)

- **MCTS criticality:** P2 - Incomplete multi-turn / stockpile-style mechanic; situational.
- **Generations:** 3-9
- **Mechanical class:** Partial: other incomplete mechanics
- **What is wrong:** Does not become the terrain/environment move (e.g. Tri Attack). Pure no-op.
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Nature Power](https://bulbapedia.bulbagarden.net/wiki/Nature_Power_(move))
- **Move table:** [`src/choices.rs:11015`](src/choices.rs#L11015) (`Choices::NATUREPOWER`)
- **Special-case handlers:** _none_

### Pika Papow (`PIKAPAPOW`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 7-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Pika Papow](https://bulbapedia.bulbagarden.net/wiki/Pika_Papow_(move))
- **Move table:** [`src/choices.rs:11757`](src/choices.rs#L11757) (`Choices::PIKAPAPOW`)
- **Special-case handlers:** _none_

### Power Split (`POWERSPLIT`)

- **MCTS criticality:** P2 - Swap/split/acupressure-style tools are rare in standard singles search.
- **Generations:** 5-9
- **Mechanical class:** Stat-control / swap / split moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Power Split](https://bulbapedia.bulbagarden.net/wiki/Power_Split_(move))
- **Move table:** [`src/choices.rs:12262`](src/choices.rs#L12262) (`Choices::POWERSPLIT`)
- **Special-case handlers:** _none_

### Power Swap (`POWERSWAP`)

- **MCTS criticality:** P2 - Swap/split/acupressure-style tools are rare in standard singles search.
- **Generations:** 4-9
- **Mechanical class:** Stat-control / swap / split moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Power Swap](https://bulbapedia.bulbagarden.net/wiki/Power_Swap_(move))
- **Move table:** [`src/choices.rs:12274`](src/choices.rs#L12274) (`Choices::POWERSWAP`)
- **Special-case handlers:** _none_

### Present (`PRESENT`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 2-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Present](https://bulbapedia.bulbagarden.net/wiki/Present_(move))
- **Move table:** [`src/choices.rs:12376`](src/choices.rs#L12376) (`Choices::PRESENT`)
- **Special-case handlers:** _none_

### Psych Up (`PSYCHUP`)

- **MCTS criticality:** P2 - Swap/split/acupressure-style tools are rare in standard singles search.
- **Generations:** 2-9
- **Mechanical class:** Stat-control / swap / split moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Psych Up](https://bulbapedia.bulbagarden.net/wiki/Psych_Up_(move))
- **Move table:** [`src/choices.rs:12627`](src/choices.rs#L12627) (`Choices::PSYCHUP`)
- **Special-case handlers:** _none_

### Psycho Shift (`PSYCHOSHIFT`)

- **MCTS criticality:** P2 - Incomplete relative to Bulbapedia/Showdown; situational for MCTS.
- **Generations:** 4-9
- **Mechanical class:** Other shells
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** unknown
- **Bulbapedia:** [Psycho Shift](https://bulbapedia.bulbagarden.net/wiki/Psycho_Shift_(move))
- **Move table:** [`src/choices.rs:12600`](src/choices.rs#L12600) (`Choices::PSYCHOSHIFT`)
- **Special-case handlers:** _none_

### Psywave (`PSYWAVE`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 1-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Psywave](https://bulbapedia.bulbagarden.net/wiki/Psywave_(move))
- **Move table:** [`src/choices.rs:12695`](src/choices.rs#L12695) (`Choices::PSYWAVE`)
- **Special-case handlers:** _none_

### Punishment (`PUNISHMENT`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 4-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Punishment](https://bulbapedia.bulbagarden.net/wiki/Punishment_(move))
- **Move table:** [`src/choices.rs:12724`](src/choices.rs#L12724) (`Choices::PUNISHMENT`)
- **Special-case handlers:** _none_

### Rage (`RAGE`)

- **MCTS criticality:** P2 - Incomplete multi-turn / stockpile-style mechanic; situational.
- **Generations:** 1-7
- **Mechanical class:** Partial: other incomplete mechanics
- **What is wrong:** Deals fixed 20 BP damage only. Rage volatile / Attack boost-when-hit is not wired from the move path in genx special effects.
- **Easy to fix?** medium
- **Bulbapedia:** [Rage](https://bulbapedia.bulbagarden.net/wiki/Rage_(move))
- **Move table:** [`src/choices.rs:12856`](src/choices.rs#L12856) (`Choices::RAGE`)
- **Special-case handlers:** _none_

### Recycle (`RECYCLE`)

- **MCTS criticality:** P2 - Status shell; only matters if that exact utility is on the moveset.
- **Generations:** 3-9
- **Mechanical class:** Other status shells
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** unknown
- **Bulbapedia:** [Recycle](https://bulbapedia.bulbagarden.net/wiki/Recycle_(move))
- **Move table:** [`src/choices.rs:13085`](src/choices.rs#L13085) (`Choices::RECYCLE`)
- **Special-case handlers:** _none_

### Reflect Type (`REFLECTTYPE`)

- **MCTS criticality:** P2 - Ability/type gizmos are situational; wrong if chosen, rarely central to MCTS policy.
- **Generations:** 5-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Reflect Type](https://bulbapedia.bulbagarden.net/wiki/Reflect_Type_(move))
- **Move table:** [`src/choices.rs:13132`](src/choices.rs#L13132) (`Choices::REFLECTTYPE`)
- **Special-case handlers:** _none_

### Role Play (`ROLEPLAY`)

- **MCTS criticality:** P2 - Ability/type gizmos are situational; wrong if chosen, rarely central to MCTS policy.
- **Generations:** 3-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Role Play](https://bulbapedia.bulbagarden.net/wiki/Role_Play_(move))
- **Move table:** [`src/choices.rs:13619`](src/choices.rs#L13619) (`Choices::ROLEPLAY`)
- **Special-case handlers:** _none_

### Rollout (`ROLLOUT`)

- **MCTS criticality:** P2 - Ramp moves are niche; first-hit BP may still be non-zero so not fully blank.
- **Generations:** 2-9
- **Mechanical class:** Partial: consecutive-use / ramp moves
- **What is wrong:** No consecutive-use power doubling, no Defense Curl boost, no 5-turn lock-in.
- **Easy to fix?** medium
- **Bulbapedia:** [Rollout](https://bulbapedia.bulbagarden.net/wiki/Rollout_(move))
- **Move table:** [`src/choices.rs:13651`](src/choices.rs#L13651) (`Choices::ROLLOUT`)
- **Special-case handlers:** _none_

### Simple Beam (`SIMPLEBEAM`)

- **MCTS criticality:** P2 - Ability/type gizmos are situational; wrong if chosen, rarely central to MCTS policy.
- **Generations:** 5-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Simple Beam](https://bulbapedia.bulbagarden.net/wiki/Simple_Beam_(move))
- **Move table:** [`src/choices.rs:14627`](src/choices.rs#L14627) (`Choices::SIMPLEBEAM`)
- **Special-case handlers:** _none_

### Skill Swap (`SKILLSWAP`)

- **MCTS criticality:** P2 - Ability/type gizmos are situational; wrong if chosen, rarely central to MCTS policy.
- **Generations:** 3-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Skill Swap](https://bulbapedia.bulbagarden.net/wiki/Skill_Swap_(move))
- **Move table:** [`src/choices.rs:14690`](src/choices.rs#L14690) (`Choices::SKILLSWAP`)
- **Special-case handlers:** _none_

### Sonic Boom (`SONICBOOM`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 1-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Sonic Boom](https://bulbapedia.bulbagarden.net/wiki/Sonic_Boom_(move))
- **Move table:** [`src/choices.rs:15306`](src/choices.rs#L15306) (`Choices::SONICBOOM`)
- **Special-case handlers:** _none_

### Speed Swap (`SPEEDSWAP`)

- **MCTS criticality:** P2 - Swap/split/acupressure-style tools are rare in standard singles search.
- **Generations:** 7-9
- **Mechanical class:** Stat-control / swap / split moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Speed Swap](https://bulbapedia.bulbagarden.net/wiki/Speed_Swap_(move))
- **Move table:** [`src/choices.rs:15405`](src/choices.rs#L15405) (`Choices::SPEEDSWAP`)
- **Special-case handlers:** _none_

### Spit Up (`SPITUP`)

- **MCTS criticality:** P2 - Incomplete multi-turn / stockpile-style mechanic; situational.
- **Generations:** 3-9
- **Mechanical class:** Partial: other incomplete mechanics
- **What is wrong:** BP 0 shell; does not consume Stockpile counts for damage.
- **Easy to fix?** medium
- **Bulbapedia:** [Spit Up](https://bulbapedia.bulbagarden.net/wiki/Spit_Up_(move))
- **Move table:** [`src/choices.rs:15581`](src/choices.rs#L15581) (`Choices::SPITUP`)
- **Special-case handlers:** _none_

### Spite (`SPITE`)

- **MCTS criticality:** P2 - Status shell; only matters if that exact utility is on the moveset.
- **Generations:** 2-9
- **Mechanical class:** Other status shells
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** unknown
- **Bulbapedia:** [Spite](https://bulbapedia.bulbagarden.net/wiki/Spite_(move))
- **Move table:** [`src/choices.rs:15568`](src/choices.rs#L15568) (`Choices::SPITE`)
- **Special-case handlers:** _none_

### Stockpile (`STOCKPILE`)

- **MCTS criticality:** P2 - Incomplete multi-turn / stockpile-style mechanic; situational.
- **Generations:** 3-9
- **Mechanical class:** Partial: other incomplete mechanics
- **What is wrong:** Sets `STOCKPILE` volatile only; no Def/SpD boosts and no stockpile count (1-3).
- **Easy to fix?** medium
- **Bulbapedia:** [Stockpile](https://bulbapedia.bulbagarden.net/wiki/Stockpile_(move))
- **Move table:** [`src/choices.rs:15823`](src/choices.rs#L15823) (`Choices::STOCKPILE`)
- **Special-case handlers:** _none_

### Stuff Cheeks (`STUFFCHEEKS`)

- **MCTS criticality:** P2 - Status shell; only matters if that exact utility is on the moveset.
- **Generations:** 8-9
- **Mechanical class:** Other status shells
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** unknown
- **Bulbapedia:** [Stuff Cheeks](https://bulbapedia.bulbagarden.net/wiki/Stuff_Cheeks_(move))
- **Move table:** [`src/choices.rs:16101`](src/choices.rs#L16101) (`Choices::STUFFCHEEKS`)
- **Special-case handlers:** _none_

### Swallow (`SWALLOW`)

- **MCTS criticality:** P2 - Incomplete multi-turn / stockpile-style mechanic; situational.
- **Generations:** 3-9
- **Mechanical class:** Partial: other incomplete mechanics
- **What is wrong:** Incomplete without stockpile counts; cannot correctly scale healing.
- **Easy to fix?** medium
- **Bulbapedia:** [Swallow](https://bulbapedia.bulbagarden.net/wiki/Swallow_(move))
- **Move table:** [`src/choices.rs:16412`](src/choices.rs#L16412) (`Choices::SWALLOW`)
- **Special-case handlers:** _none_

### Take Heart (`TAKEHEART`)

- **MCTS criticality:** P2 - Status shell; only matters if that exact utility is on the moveset.
- **Generations:** 8-9
- **Mechanical class:** Other status shells
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** unknown
- **Bulbapedia:** [Take Heart](https://bulbapedia.bulbagarden.net/wiki/Take_Heart_(move))
- **Move table:** [`src/choices.rs:16738`](src/choices.rs#L16738) (`Choices::TAKEHEART`)
- **Special-case handlers:** _none_

### Teatime (`TEATIME`)

- **MCTS criticality:** P2 - Status shell; only matters if that exact utility is on the moveset.
- **Generations:** 8-9
- **Mechanical class:** Other status shells
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** unknown
- **Bulbapedia:** [Teatime](https://bulbapedia.bulbagarden.net/wiki/Teatime_(move))
- **Move table:** [`src/choices.rs:16818`](src/choices.rs#L16818) (`Choices::TEATIME`)
- **Special-case handlers:** _none_

### Topsy-Turvy (`TOPSYTURVY`)

- **MCTS criticality:** P2 - Swap/split/acupressure-style tools are rare in standard singles search.
- **Generations:** 6-9
- **Mechanical class:** Stat-control / swap / split moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Topsy-Turvy](https://bulbapedia.bulbagarden.net/wiki/Topsy-Turvy_(move))
- **Move table:** [`src/choices.rs:17428`](src/choices.rs#L17428) (`Choices::TOPSYTURVY`)
- **Special-case handlers:** _none_

### Trump Card (`TRUMPCARD`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 4-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Trump Card](https://bulbapedia.bulbagarden.net/wiki/Trump_Card_(move))
- **Move table:** [`src/choices.rs:17801`](src/choices.rs#L17801) (`Choices::TRUMPCARD`)
- **Special-case handlers:** _none_

### Uproar (`UPROAR`)

- **MCTS criticality:** P2 - Incomplete multi-turn / stockpile-style mechanic; situational.
- **Generations:** 3-9
- **Mechanical class:** Partial: other incomplete mechanics
- **What is wrong:** Has BP and an `UPROAR` volatile exists, but multi-turn lock / wake-from-sleep behavior is not fully special-cased.
- **Easy to fix?** medium
- **Bulbapedia:** [Uproar](https://bulbapedia.bulbagarden.net/wiki/Uproar_(move))
- **Move table:** [`src/choices.rs:17890`](src/choices.rs#L17890) (`Choices::UPROAR`)
- **Special-case handlers:** _none_

### Veevee Volley (`VEEVEEVOLLEY`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 7-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Veevee Volley](https://bulbapedia.bulbagarden.net/wiki/Veevee_Volley_(move))
- **Move table:** [`src/choices.rs:17980`](src/choices.rs#L17980) (`Choices::VEEVEEVOLLEY`)
- **Special-case handlers:** _none_

### Venom Drench (`VENOMDRENCH`)

- **MCTS criticality:** P2 - Swap/split/acupressure-style tools are rare in standard singles search.
- **Generations:** 6-9
- **Mechanical class:** Stat-control / swap / split moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** medium
- **Bulbapedia:** [Venom Drench](https://bulbapedia.bulbagarden.net/wiki/Venom_Drench_(move))
- **Move table:** [`src/choices.rs:17994`](src/choices.rs#L17994) (`Choices::VENOMDRENCH`)
- **Special-case handlers:** _none_

### Water Sport (`WATERSPORT`)

- **MCTS criticality:** P2 - Field sport / niche control; rarely the best action, but currently a no-op if chosen.
- **Generations:** 3-9
- **Mechanical class:** Field-control moves (terrain / rooms / gravity / sports)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Water Sport](https://bulbapedia.bulbagarden.net/wiki/Water_Sport_(move))
- **Move table:** [`src/choices.rs:18297`](src/choices.rs#L18297) (`Choices::WATERSPORT`)
- **Special-case handlers:** _none_

### Wonder Room (`WONDERROOM`)

- **MCTS criticality:** P2 - Uncommon in singles but can change immunities / item / speed interactions when used.
- **Generations:** 5-9
- **Mechanical class:** Field-control moves (terrain / rooms / gravity / sports)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Wonder Room](https://bulbapedia.bulbagarden.net/wiki/Wonder_Room_(move))
- **Move table:** [`src/choices.rs:18627`](src/choices.rs#L18627) (`Choices::WONDERROOM`)
- **Special-case handlers:** _none_

### Worry Seed (`WORRYSEED`)

- **MCTS criticality:** P2 - Ability/type gizmos are situational; wrong if chosen, rarely central to MCTS policy.
- **Generations:** 4-9
- **Mechanical class:** Type / ability changing moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Worry Seed](https://bulbapedia.bulbagarden.net/wiki/Worry_Seed_(move))
- **Move table:** [`src/choices.rs:18677`](src/choices.rs#L18677) (`Choices::WORRYSEED`)
- **Special-case handlers:** _none_

### Wring Out (`WRINGOUT`)

- **MCTS criticality:** P2 - Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.
- **Generations:** 4-9
- **Mechanical class:** Fixed / variable damage shells (no damage logic)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy-medium
- **Bulbapedia:** [Wring Out](https://bulbapedia.bulbagarden.net/wiki/Wring_Out_(move))
- **Move table:** [`src/choices.rs:18733`](src/choices.rs#L18733) (`Choices::WRINGOUT`)
- **Special-case handlers:** _none_

## P3 - Low (near-unbiased approximations)

Statistical shortcuts whose expected value is close to the real distribution. Fine for MCTS move selection; branching them is polish, not correctness.

_21 moves_

### Arm Thrust (`ARMTHRUST`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 3-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Arm Thrust](https://bulbapedia.bulbagarden.net/wiki/Arm_Thrust_(move))
- **Move table:** [`src/choices.rs:586`](src/choices.rs#L586) (`Choices::ARMTHRUST`)
- **Special-case handlers:** _none_

### Barrage (`BARRAGE`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 1-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Barrage](https://bulbapedia.bulbagarden.net/wiki/Barrage_(move))
- **Move table:** [`src/choices.rs:1015`](src/choices.rs#L1015) (`Choices::BARRAGE`)
- **Special-case handlers:** _none_

### Bone Rush (`BONERUSH`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 2-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Bone Rush](https://bulbapedia.bulbagarden.net/wiki/Bone_Rush_(move))
- **Move table:** [`src/choices.rs:1637`](src/choices.rs#L1637) (`Choices::BONERUSH`)
- **Special-case handlers:** _none_

### Bullet Seed (`BULLETSEED`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 3-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Bullet Seed](https://bulbapedia.bulbagarden.net/wiki/Bullet_Seed_(move))
- **Move table:** [`src/choices.rs:2070`](src/choices.rs#L2070) (`Choices::BULLETSEED`)
- **Special-case handlers:** _none_

### Comet Punch (`COMETPUNCH`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 1-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Comet Punch](https://bulbapedia.bulbagarden.net/wiki/Comet_Punch_(move))
- **Move table:** [`src/choices.rs:2671`](src/choices.rs#L2671) (`Choices::COMETPUNCH`)
- **Special-case handlers:** _none_

### Double Slap (`DOUBLESLAP`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 1-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Double Slap](https://bulbapedia.bulbagarden.net/wiki/Double_Slap_(move))
- **Move table:** [`src/choices.rs:3905`](src/choices.rs#L3905) (`Choices::DOUBLESLAP`)
- **Special-case handlers:** _none_

### Fury Attack (`FURYATTACK`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 1-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Fury Attack](https://bulbapedia.bulbagarden.net/wiki/Fury_Attack_(move))
- **Move table:** [`src/choices.rs:6155`](src/choices.rs#L6155) (`Choices::FURYATTACK`)
- **Special-case handlers:** _none_

### Fury Swipes (`FURYSWIPES`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 1-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Fury Swipes](https://bulbapedia.bulbagarden.net/wiki/Fury_Swipes_(move))
- **Move table:** [`src/choices.rs:6224`](src/choices.rs#L6224) (`Choices::FURYSWIPES`)
- **Special-case handlers:** _none_

### Icicle Spear (`ICICLESPEAR`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 3-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Icicle Spear](https://bulbapedia.bulbagarden.net/wiki/Icicle_Spear_(move))
- **Move table:** [`src/choices.rs:8466`](src/choices.rs#L8466) (`Choices::ICICLESPEAR`)
- **Special-case handlers:** _none_

### Outrage (`OUTRAGE`)

- **MCTS criticality:** P3 - True duration is random 2-3 turns; always modeling 3 is a mild optimistic bias, acceptable for MCTS.
- **Generations:** 2-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** Locked-move duration not branched; engine assumes always 3 turns
- **Easy to fix?** medium
- **Bulbapedia:** [Outrage](https://bulbapedia.bulbagarden.net/wiki/Outrage_(move))
- **Move table:** [`src/choices.rs:11386`](src/choices.rs#L11386) (`Choices::OUTRAGE`)
- **Special-case handlers:** _none_

### Petal Dance (`PETALDANCE`)

- **MCTS criticality:** P3 - Same locked-move 2-3 approximation as Outrage.
- **Generations:** 1-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** Locked-move duration not branched; engine assumes always 3 turns
- **Easy to fix?** medium
- **Bulbapedia:** [Petal Dance](https://bulbapedia.bulbagarden.net/wiki/Petal_Dance_(move))
- **Move table:** [`src/choices.rs:11667`](src/choices.rs#L11667) (`Choices::PETALDANCE`)
- **Special-case handlers:** _none_

### Pin Missile (`PINMISSILE`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 1-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Pin Missile](https://bulbapedia.bulbagarden.net/wiki/Pin_Missile_(move))
- **Move table:** [`src/choices.rs:11771`](src/choices.rs#L11771) (`Choices::PINMISSILE`)
- **Special-case handlers:** _none_

### Raging Fury (`RAGINGFURY`)

- **MCTS criticality:** P3 - Same locked-move 2-3 approximation as Outrage.
- **Generations:** 8-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** Locked-move duration not branched; engine assumes always 3 turns
- **Easy to fix?** medium
- **Bulbapedia:** [Raging Fury](https://bulbapedia.bulbagarden.net/wiki/Raging_Fury_(move))
- **Move table:** [`src/choices.rs:12920`](src/choices.rs#L12920) (`Choices::RAGINGFURY`)
- **Special-case handlers:** _none_

### Return (`RETURN`)

- **MCTS criticality:** P3 - Competitive mons are usually max happiness; BP 102 matches the common case.
- **Generations:** 2-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** Always base power 102 (also RETURN102 variant)
- **Easy to fix?** medium
- **Bulbapedia:** [Return](https://bulbapedia.bulbagarden.net/wiki/Return_(move))
- **Move table:** [`src/choices.rs:13206`](src/choices.rs#L13206) (`Choices::RETURN`)
- **Special-case handlers:** _none_

### Return102 (`RETURN102`)

- **MCTS criticality:** P3 - Explicit max-Return variant; fine for search.
- **Generations:** 1-9 (as applicable)
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** Hardcoded Return at BP 102
- **Easy to fix?** medium
- **Bulbapedia:** [Return102](https://bulbapedia.bulbagarden.net/wiki/Return102_(move))
- **Move table:** [`src/choices.rs:13221`](src/choices.rs#L13221) (`Choices::RETURN102`)
- **Special-case handlers:** _none_

### Rock Blast (`ROCKBLAST`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 3-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Rock Blast](https://bulbapedia.bulbagarden.net/wiki/Rock_Blast_(move))
- **Move table:** [`src/choices.rs:13339`](src/choices.rs#L13339) (`Choices::ROCKBLAST`)
- **Special-case handlers:** _none_

### Scale Shot (`SCALESHOT`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 8-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Scale Shot](https://bulbapedia.bulbagarden.net/wiki/Scale_Shot_(move))
- **Move table:** [`src/choices.rs:13958`](src/choices.rs#L13958) (`Choices::SCALESHOT`)
- **Special-case handlers:** _none_

### Spike Cannon (`SPIKECANNON`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 1-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Spike Cannon](https://bulbapedia.bulbagarden.net/wiki/Spike_Cannon_(move))
- **Move table:** [`src/choices.rs:15454`](src/choices.rs#L15454) (`Choices::SPIKECANNON`)
- **Special-case handlers:** _none_

### Tail Slap (`TAILSLAP`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 5-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Tail Slap](https://bulbapedia.bulbagarden.net/wiki/Tail_Slap_(move))
- **Move table:** [`src/choices.rs:16664`](src/choices.rs#L16664) (`Choices::TAILSLAP`)
- **Special-case handlers:** _none_

### Thrash (`THRASH`)

- **MCTS criticality:** P3 - Same locked-move 2-3 approximation as Outrage.
- **Generations:** 1-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** Locked-move duration not branched; engine assumes always 3 turns
- **Easy to fix?** medium
- **Bulbapedia:** [Thrash](https://bulbapedia.bulbagarden.net/wiki/Thrash_(move))
- **Move table:** [`src/choices.rs:17048`](src/choices.rs#L17048) (`Choices::THRASH`)
- **Special-case handlers:** _none_

### Water Shuriken (`WATERSHURIKEN`)

- **MCTS criticality:** P3 - Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. Variance is lost, expectation is not; low impact on MCTS move choice.
- **Generations:** 6-9
- **Mechanical class:** Known approximations (implemented, but simplified)
- **What is wrong:** 2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)
- **Easy to fix?** medium
- **Bulbapedia:** [Water Shuriken](https://bulbapedia.bulbagarden.net/wiki/Water_Shuriken_(move))
- **Move table:** [`src/choices.rs:18264`](src/choices.rs#L18264) (`Choices::WATERSHURIKEN`)
- **Special-case handlers:** _none_

## P4 - Negligible / out of scope

Gimmicks, OHKOs, doubles-only tools, or moves almost never used in competitive singles search.

_22 moves_

### After You (`AFTERYOU`)

- **MCTS criticality:** P4 - Singles engine; doubles tools should not affect MCTS policy.
- **Generations:** 5-9
- **Mechanical class:** Doubles / team-support (out of scope for singles engine)
- **What is wrong:** Doubles/team-support effect. poke-engine is singles-only, so a stub is expected rather than a singles mechanics bug.
- **Easy to fix?** n/a (out of scope)
- **Bulbapedia:** [After You](https://bulbapedia.bulbagarden.net/wiki/After_You_(move))
- **Move table:** [`src/choices.rs:248`](src/choices.rs#L248) (`Choices::AFTERYOU`)
- **Special-case handlers:** _none_

### Ally Switch (`ALLYSWITCH`)

- **MCTS criticality:** P4 - Singles engine; doubles tools should not affect MCTS policy.
- **Generations:** 5-9
- **Mechanical class:** Doubles / team-support (out of scope for singles engine)
- **What is wrong:** Doubles/team-support effect. poke-engine is singles-only, so a stub is expected rather than a singles mechanics bug.
- **Easy to fix?** n/a (out of scope)
- **Bulbapedia:** [Ally Switch](https://bulbapedia.bulbagarden.net/wiki/Ally_Switch_(move))
- **Move table:** [`src/choices.rs:353`](src/choices.rs#L353) (`Choices::ALLYSWITCH`)
- **Special-case handlers:** _none_

### Assist (`ASSIST`)

- **MCTS criticality:** P4 - Metronome / Assist / Mirror Move / Sketch / etc. are not serious singles search actions.
- **Generations:** 3-9
- **Mechanical class:** Copy / transform / random-call moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Assist](https://bulbapedia.bulbagarden.net/wiki/Assist_(move))
- **Move table:** [`src/choices.rs:636`](src/choices.rs#L636) (`Choices::ASSIST`)
- **Special-case handlers:** _none_

### Copycat (`COPYCAT`)

- **MCTS criticality:** P4 - Metronome / Assist / Mirror Move / Sketch / etc. are not serious singles search actions.
- **Generations:** 4-9
- **Mechanical class:** Copy / transform / random-call moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Copycat](https://bulbapedia.bulbagarden.net/wiki/Copycat_(move))
- **Move table:** [`src/choices.rs:2842`](src/choices.rs#L2842) (`Choices::COPYCAT`)
- **Special-case handlers:** _none_

### Dragon Cheer (`DRAGONCHEER`)

- **MCTS criticality:** P4 - Singles engine; doubles tools should not affect MCTS policy.
- **Generations:** 9
- **Mechanical class:** Doubles / team-support (out of scope for singles engine)
- **What is wrong:** Doubles/team-support effect. poke-engine is singles-only, so a stub is expected rather than a singles mechanics bug.
- **Easy to fix?** n/a (out of scope)
- **Bulbapedia:** [Dragon Cheer](https://bulbapedia.bulbagarden.net/wiki/Dragon_Cheer_(move))
- **Move table:** [`src/choices.rs:4044`](src/choices.rs#L4044) (`Choices::DRAGONCHEER`)
- **Special-case handlers:** _none_

### Fissure (`FISSURE`)

- **MCTS criticality:** P4 - OHKOs are clause-banned or irrelevant in the formats this engine targets.
- **Generations:** 1-9
- **Mechanical class:** OHKO moves (no KO effect)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy
- **Bulbapedia:** [Fissure](https://bulbapedia.bulbagarden.net/wiki/Fissure_(move))
- **Move table:** [`src/choices.rs:5479`](src/choices.rs#L5479) (`Choices::FISSURE`)
- **Special-case handlers:** _none_

### Flower Shield (`FLOWERSHIELD`)

- **MCTS criticality:** P4 - Singles engine; doubles tools should not affect MCTS policy.
- **Generations:** 6-9
- **Mechanical class:** Doubles / team-support (out of scope for singles engine)
- **What is wrong:** Doubles/team-support effect. poke-engine is singles-only, so a stub is expected rather than a singles mechanics bug.
- **Easy to fix?** n/a (out of scope)
- **Bulbapedia:** [Flower Shield](https://bulbapedia.bulbagarden.net/wiki/Flower_Shield_(move))
- **Move table:** [`src/choices.rs:5798`](src/choices.rs#L5798) (`Choices::FLOWERSHIELD`)
- **Special-case handlers:** _none_

### Frustration (`FRUSTRATION`)

- **MCTS criticality:** P4 - Unused in modern competitive (min happiness); removed in gen 9.
- **Generations:** 2-8
- **Mechanical class:** Partial: variable base power missing or wrong
- **What is wrong:** BP 0; happiness scaling not modeled (Return is hardcoded to 102).
- **Easy to fix?** easy
- **Bulbapedia:** [Frustration](https://bulbapedia.bulbagarden.net/wiki/Frustration_(move))
- **Move table:** [`src/choices.rs:6141`](src/choices.rs#L6141) (`Choices::FRUSTRATION`)
- **Special-case handlers:** _none_

### Gear Up (`GEARUP`)

- **MCTS criticality:** P4 - Singles engine; doubles tools should not affect MCTS policy.
- **Generations:** 7-9
- **Mechanical class:** Doubles / team-support (out of scope for singles engine)
- **What is wrong:** Doubles/team-support effect. poke-engine is singles-only, so a stub is expected rather than a singles mechanics bug.
- **Easy to fix?** n/a (out of scope)
- **Bulbapedia:** [Gear Up](https://bulbapedia.bulbagarden.net/wiki/Gear_Up_(move))
- **Move table:** [`src/choices.rs:6345`](src/choices.rs#L6345) (`Choices::GEARUP`)
- **Special-case handlers:** _none_

### Guillotine (`GUILLOTINE`)

- **MCTS criticality:** P4 - OHKOs are clause-banned or irrelevant in the formats this engine targets.
- **Generations:** 1-9
- **Mechanical class:** OHKO moves (no KO effect)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy
- **Bulbapedia:** [Guillotine](https://bulbapedia.bulbagarden.net/wiki/Guillotine_(move))
- **Move table:** [`src/choices.rs:6853`](src/choices.rs#L6853) (`Choices::GUILLOTINE`)
- **Special-case handlers:** _none_

### Horn Drill (`HORNDRILL`)

- **MCTS criticality:** P4 - OHKOs are clause-banned or irrelevant in the formats this engine targets.
- **Generations:** 1-9
- **Mechanical class:** OHKO moves (no KO effect)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy
- **Bulbapedia:** [Horn Drill](https://bulbapedia.bulbagarden.net/wiki/Horn_Drill_(move))
- **Move table:** [`src/choices.rs:7972`](src/choices.rs#L7972) (`Choices::HORNDRILL`)
- **Special-case handlers:** _none_

### Instruct (`INSTRUCT`)

- **MCTS criticality:** P4 - Metronome / Assist / Mirror Move / Sketch / etc. are not serious singles search actions.
- **Generations:** 7-9
- **Mechanical class:** Copy / transform / random-call moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Instruct](https://bulbapedia.bulbagarden.net/wiki/Instruct_(move))
- **Move table:** [`src/choices.rs:8645`](src/choices.rs#L8645) (`Choices::INSTRUCT`)
- **Special-case handlers:** _none_

### Magnetic Flux (`MAGNETICFLUX`)

- **MCTS criticality:** P4 - Singles engine; doubles tools should not affect MCTS policy.
- **Generations:** 6-9
- **Mechanical class:** Doubles / team-support (out of scope for singles engine)
- **What is wrong:** Doubles/team-support effect. poke-engine is singles-only, so a stub is expected rather than a singles mechanics bug.
- **Easy to fix?** n/a (out of scope)
- **Bulbapedia:** [Magnetic Flux](https://bulbapedia.bulbagarden.net/wiki/Magnetic_Flux_(move))
- **Move table:** [`src/choices.rs:9904`](src/choices.rs#L9904) (`Choices::MAGNETICFLUX`)
- **Special-case handlers:** _none_

### Me First (`MEFIRST`)

- **MCTS criticality:** P4 - Metronome / Assist / Mirror Move / Sketch / etc. are not serious singles search actions.
- **Generations:** 4-9
- **Mechanical class:** Copy / transform / random-call moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Me First](https://bulbapedia.bulbagarden.net/wiki/Me_First_(move))
- **Move table:** [`src/choices.rs:10062`](src/choices.rs#L10062) (`Choices::MEFIRST`)
- **Special-case handlers:** _none_

### Metronome (`METRONOME`)

- **MCTS criticality:** P4 - Metronome / Assist / Mirror Move / Sketch / etc. are not serious singles search actions.
- **Generations:** 1-9
- **Mechanical class:** Copy / transform / random-call moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Metronome](https://bulbapedia.bulbagarden.net/wiki/Metronome_(move))
- **Move table:** [`src/choices.rs:10325`](src/choices.rs#L10325) (`Choices::METRONOME`)
- **Special-case handlers:** _none_

### Mimic (`MIMIC`)

- **MCTS criticality:** P4 - Metronome / Assist / Mirror Move / Sketch / etc. are not serious singles search actions.
- **Generations:** 1-9
- **Mechanical class:** Copy / transform / random-call moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Mimic](https://bulbapedia.bulbagarden.net/wiki/Mimic_(move))
- **Move table:** [`src/choices.rs:10369`](src/choices.rs#L10369) (`Choices::MIMIC`)
- **Special-case handlers:** _none_

### Mirror Move (`MIRRORMOVE`)

- **MCTS criticality:** P4 - Metronome / Assist / Mirror Move / Sketch / etc. are not serious singles search actions.
- **Generations:** 1-9
- **Mechanical class:** Copy / transform / random-call moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Mirror Move](https://bulbapedia.bulbagarden.net/wiki/Mirror_Move_(move))
- **Move table:** [`src/choices.rs:10465`](src/choices.rs#L10465) (`Choices::MIRRORMOVE`)
- **Special-case handlers:** _none_

### Quash (`QUASH`)

- **MCTS criticality:** P4 - Singles engine; doubles tools should not affect MCTS policy.
- **Generations:** 5-9
- **Mechanical class:** Doubles / team-support (out of scope for singles engine)
- **What is wrong:** Doubles/team-support effect. poke-engine is singles-only, so a stub is expected rather than a singles mechanics bug.
- **Easy to fix?** n/a (out of scope)
- **Bulbapedia:** [Quash](https://bulbapedia.bulbagarden.net/wiki/Quash_(move))
- **Move table:** [`src/choices.rs:12788`](src/choices.rs#L12788) (`Choices::QUASH`)
- **Special-case handlers:** _none_

### Rototiller (`ROTOTILLER`)

- **MCTS criticality:** P4 - Singles engine; doubles tools should not affect MCTS policy.
- **Generations:** 6-9
- **Mechanical class:** Doubles / team-support (out of scope for singles engine)
- **What is wrong:** Doubles/team-support effect. poke-engine is singles-only, so a stub is expected rather than a singles mechanics bug.
- **Easy to fix?** n/a (out of scope)
- **Bulbapedia:** [Rototiller](https://bulbapedia.bulbagarden.net/wiki/Rototiller_(move))
- **Move table:** [`src/choices.rs:13688`](src/choices.rs#L13688) (`Choices::ROTOTILLER`)
- **Special-case handlers:** _none_

### Sheer Cold (`SHEERCOLD`)

- **MCTS criticality:** P4 - OHKOs are clause-banned or irrelevant in the formats this engine targets.
- **Generations:** 3-9
- **Mechanical class:** OHKO moves (no KO effect)
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** easy
- **Bulbapedia:** [Sheer Cold](https://bulbapedia.bulbagarden.net/wiki/Sheer_Cold_(move))
- **Move table:** [`src/choices.rs:14417`](src/choices.rs#L14417) (`Choices::SHEERCOLD`)
- **Special-case handlers:** _none_

### Sketch (`SKETCH`)

- **MCTS criticality:** P4 - Metronome / Assist / Mirror Move / Sketch / etc. are not serious singles search actions.
- **Generations:** 2-9
- **Mechanical class:** Copy / transform / random-call moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Sketch](https://bulbapedia.bulbagarden.net/wiki/Sketch_(move))
- **Move table:** [`src/choices.rs:14679`](src/choices.rs#L14679) (`Choices::SKETCH`)
- **Special-case handlers:** _none_

### Transform (`TRANSFORM`)

- **MCTS criticality:** P4 - Metronome / Assist / Mirror Move / Sketch / etc. are not serious singles search actions.
- **Generations:** 1-9
- **Mechanical class:** Copy / transform / random-call moves
- **What is wrong:** Present in Choices / move table but no declarative effect fields and no special-case handler in choice_effects / generate_instructions / damage_calc (effectively a no-op or empty attack).
- **Easy to fix?** hard
- **Bulbapedia:** [Transform](https://bulbapedia.bulbagarden.net/wiki/Transform_(move))
- **Move table:** [`src/choices.rs:17595`](src/choices.rs#L17595) (`Choices::TRANSFORM`)
- **Special-case handlers:** _none_

---

## Regenerating this report

```bash
python docs/moves/audit_moves.py
python docs/moves/generate_move_report.py
```

Artifacts: `docs/moves/move_audit.json`, `docs/moves/move_audit_bad.json`, `docs/moves/.cache_moves.json`.
