#!/usr/bin/env python3
"""Generate MOVE_IMPLEMENTATION_REPORT.md from audit JSON + manual partials."""

from __future__ import annotations

import json
import re
from collections import Counter
from pathlib import Path

# docs/moves/<this file> -> repo root is parents[2]
ROOT = Path(__file__).resolve().parents[2]
DOCS_MOVES = Path(__file__).resolve().parent
CHOICES = (ROOT / "src" / "choices.rs").read_text(encoding="utf-8")
SD = json.loads((DOCS_MOVES / ".cache_moves.json").read_text(encoding="utf-8"))
BAD = json.loads((DOCS_MOVES / "move_audit_bad.json").read_text(encoding="utf-8"))
AUDIT = json.loads((DOCS_MOVES / "move_audit.json").read_text(encoding="utf-8"))
SPECIAL = AUDIT["special_map"]

FALSE_POSITIVE = {
    "ROAR",
    "WHIRLWIND",  # drag flag implemented in generate_instructions
    "TELEPORT",  # pivot gen8+; earlier gens correctly fail in battle
    "NOTHING",
    "SPLASH",
    "CELEBRATE",
    "HAPPYHOUR",
    "HOLDHANDS",
    "NONE",
}

DOUBLES = {
    "AFTERYOU",
    "ALLYSWITCH",
    "QUASH",
    "HELPINGHAND",
    "FOLLOWME",
    "RAGEPOWDER",
    "SPOTLIGHT",
    "COACHING",
    "AROMATICMIST",
    "CRAFTYSHIELD",
    "MATBLOCK",
    "QUICKGUARD",
    "WIDEGUARD",
    "DRAGONCHEER",
    "FLOWERSHIELD",
    "GEARUP",
    "MAGNETICFLUX",
    "ROTOTILLER",
}

PARTIAL = [
    {
        "id": "FLAIL",
        "name": "Flail",
        "category": "partial_variable_bp",
        "gens": "2-9",
        "issue": (
            "HP-ratio base power is not implemented (unlike Reversal, which is special-cased "
            "in `modify_choice`). Table BP is 0, so the move deals no damage."
        ),
        "easy_to_fix": "easy",
        "note": "Mirror the existing `Choices::REVERSAL` branch in `src/genx/choice_effects.rs`.",
    },
    {
        "id": "RAGEFIST",
        "name": "Rage Fist",
        "category": "partial_variable_bp",
        "gens": "9",
        "issue": "Always BP 50. Does not add +50 per time the user was hit (cap 350).",
        "easy_to_fix": "medium",
        "note": "Needs a hit-counter on the Pokemon plus `modify_choice` scaling.",
    },
    {
        "id": "BEATUP",
        "name": "Beat Up",
        "category": "partial_variable_bp",
        "gens": "2-9",
        "issue": (
            "Gen 2-4 uses a single fixed BP 10 hit (not one hit per healthy ally). "
            "Gen 5+ has `base_power: 0` and no ally-Attack scaling, so it deals no damage."
        ),
        "easy_to_fix": "medium",
    },
    {
        "id": "ROLLOUT",
        "name": "Rollout",
        "category": "partial_lock_ramp",
        "gens": "2-9",
        "issue": "No consecutive-use power doubling, no Defense Curl boost, no 5-turn lock-in.",
        "easy_to_fix": "medium",
    },
    {
        "id": "ICEBALL",
        "name": "Ice Ball",
        "category": "partial_lock_ramp",
        "gens": "3-9",
        "issue": "Same incomplete ramp/lock behavior as Rollout.",
        "easy_to_fix": "medium",
    },
    {
        "id": "FURYCUTTER",
        "name": "Fury Cutter",
        "category": "partial_lock_ramp",
        "gens": "2-9",
        "issue": "Static BP only; successive-use doubling is not modeled.",
        "easy_to_fix": "medium",
    },
    {
        "id": "LASTRESORT",
        "name": "Last Resort",
        "category": "partial_fail_condition",
        "gens": "4-9",
        "issue": "Always usable at full BP. Does not require all other moves to have been used.",
        "easy_to_fix": "hard",
        "note": "Needs per-Pokemon used-move tracking.",
    },
    {
        "id": "RAGE",
        "name": "Rage",
        "category": "partial",
        "gens": "1-7",
        "issue": (
            "Deals fixed 20 BP damage only. Rage volatile / Attack boost-when-hit is not "
            "wired from the move path in genx special effects."
        ),
        "easy_to_fix": "medium",
    },
    {
        "id": "BIDE",
        "name": "Bide",
        "category": "partial",
        "gens": "1-4",
        "issue": (
            "Sets `BIDE` volatile, but there is no genx damage-store / double-return "
            "implementation."
        ),
        "easy_to_fix": "hard",
    },
    {
        "id": "UPROAR",
        "name": "Uproar",
        "category": "partial",
        "gens": "3-9",
        "issue": (
            "Has BP and an `UPROAR` volatile exists, but multi-turn lock / wake-from-sleep "
            "behavior is not fully special-cased."
        ),
        "easy_to_fix": "medium",
    },
    {
        "id": "STOCKPILE",
        "name": "Stockpile",
        "category": "partial",
        "gens": "3-9",
        "issue": (
            "Sets `STOCKPILE` volatile only; no Def/SpD boosts and no stockpile count (1-3)."
        ),
        "easy_to_fix": "medium",
    },
    {
        "id": "SPITUP",
        "name": "Spit Up",
        "category": "partial",
        "gens": "3-9",
        "issue": "BP 0 shell; does not consume Stockpile counts for damage.",
        "easy_to_fix": "medium",
    },
    {
        "id": "SWALLOW",
        "name": "Swallow",
        "category": "partial",
        "gens": "3-9",
        "issue": "Incomplete without stockpile counts; cannot correctly scale healing.",
        "easy_to_fix": "medium",
    },
    {
        "id": "NATUREPOWER",
        "name": "Nature Power",
        "category": "partial",
        "gens": "3-9",
        "issue": "Does not become the terrain/environment move (e.g. Tri Attack). Pure no-op.",
        "easy_to_fix": "easy-medium",
    },
    {
        "id": "FRUSTRATION",
        "name": "Frustration",
        "category": "partial_variable_bp",
        "gens": "2-8",
        "issue": "BP 0; happiness scaling not modeled (Return is hardcoded to 102).",
        "easy_to_fix": "easy",
    },
]

CAT_TITLES = {
    "approximation": "Known approximations (implemented, but simplified)",
    "partial_variable_bp": "Partial: variable base power missing or wrong",
    "partial_lock_ramp": "Partial: consecutive-use / ramp moves",
    "partial_fail_condition": "Partial: fail conditions missing",
    "partial": "Partial: other incomplete mechanics",
    "ohko": "OHKO moves (no KO effect)",
    "variable_bp_or_fixed": "Fixed / variable damage shells (no damage logic)",
    "field_control": "Field-control moves (terrain / rooms / gravity / sports)",
    "type_ability_change": "Type / ability changing moves",
    "copy_transform": "Copy / transform / random-call moves",
    "trapping_lockon": "Trapping / Lock-On style moves",
    "stat_control_misc": "Stat-control / swap / split moves",
    "misc_status": "Other status shells",
    "other_shell": "Other shells",
    "doubles_out_of_scope": "Doubles / team-support (out of scope for singles engine)",
}

# MCTS / statistical-simulation criticality.
# Variance that matches the mean is cheap; systematic bias and "move does nothing"
# on real singles options are expensive.
CRIT_META = {
    "P0": {
        "title": "P0 - Critical for MCTS",
        "blurb": (
            "Common (or format-relevant) singles options that are non-functional or "
            "drastically wrong. Search will systematically mis-rank these lines."
        ),
    },
    "P1": {
        "title": "P1 - High (systematic bias / field state)",
        "blurb": (
            "Optimistic or pessimistic bias on used moves, or missing field effects that "
            "change later damage. Means of rollouts are skewed, not just noisy."
        ),
    },
    "P2": {
        "title": "P2 - Medium (niche / situational)",
        "blurb": (
            "Wrong or incomplete, but rarely chosen in the formats this engine targets, "
            "or only matters in narrow states."
        ),
    },
    "P3": {
        "title": "P3 - Low (near-unbiased approximations)",
        "blurb": (
            "Statistical shortcuts whose expected value is close to the real distribution. "
            "Fine for MCTS move selection; branching them is polish, not correctness."
        ),
    },
    "P4": {
        "title": "P4 - Negligible / out of scope",
        "blurb": (
            "Gimmicks, OHKOs, doubles-only tools, or moves almost never used in "
            "competitive singles search."
        ),
    },
}

CRIT_ORDER = ["P0", "P1", "P2", "P3", "P4"]

# Explicit overrides (move id -> (priority, why_this_priority))
CRIT_OVERRIDE: dict[str, tuple[str, str]] = {
    # P0: staples / real options that are broken
    "RAGEFIST": (
        "P0",
        "Annihilape staple; fixed BP 50 instead of hit-scaling makes the whole mon mis-evaluated.",
    ),
    "FLAIL": (
        "P0",
        "Deals 0 damage while Reversal works; any Flail user is treated as having a blank move.",
    ),
    "ELECTRICTERRAIN": (
        "P1",
        "Move does not set terrain (abilities like Electric Surge still do). Move-based setters "
        "never enable Expanding Force / Rising Voltage / terrain mods in search.",
    ),
    "GRASSYTERRAIN": (
        "P1",
        "Move is a no-op; Grassy Surge still works. Same search gap for move-based setters.",
    ),
    "PSYCHICTERRAIN": (
        "P1",
        "Move is a no-op; Psychic Surge still works.",
    ),
    "MISTYTERRAIN": (
        "P1",
        "Move is a no-op; Misty Surge still works.",
    ),
    # P1: biased common approximations / trapping that matters
    "POPULATIONBOMB": (
        "P1",
        "Always 6 (or 9) hits; real multi-accuracy EV is lower. Optimistic damage bias on a used move.",
    ),
    "TRIPLEAXEL": (
        "P1",
        "Always lands all three escalating hits; real multi-accuracy often stops early. Optimistic bias.",
    ),
    "TRIPLEKICK": (
        "P1",
        "Same multi-accuracy optimism as Triple Axel (less common modernly).",
    ),
    "MEANLOOK": (
        "P1",
        "Trapping not applied; phantom switches stay legal in search.",
    ),
    "BLOCK": (
        "P1",
        "Trapping not applied; phantom switches stay legal in search.",
    ),
    "SPIDERWEB": (
        "P1",
        "Trapping not applied; phantom switches stay legal in search.",
    ),
    "SOAK": (
        "P1",
        "Type change missing; breaks Water-type tricks / Immuno / coverage interactions in search.",
    ),
    # P3: near-unbiased
    "OUTRAGE": (
        "P3",
        "True duration is random 2-3 turns; always modeling 3 is a mild optimistic bias, acceptable for MCTS.",
    ),
    "THRASH": (
        "P3",
        "Same locked-move 2-3 approximation as Outrage.",
    ),
    "PETALDANCE": (
        "P3",
        "Same locked-move 2-3 approximation as Outrage.",
    ),
    "RAGINGFURY": (
        "P3",
        "Same locked-move 2-3 approximation as Outrage.",
    ),
    "RETURN": (
        "P3",
        "Competitive mons are usually max happiness; BP 102 matches the common case.",
    ),
    "RETURN102": (
        "P3",
        "Explicit max-Return variant; fine for search.",
    ),
    # P4
    "FRUSTRATION": (
        "P4",
        "Unused in modern competitive (min happiness); removed in gen 9.",
    ),
}


def assign_criticality(row: dict) -> tuple[str, str]:
    """Return (P0..P4, rationale) for MCTS statistical use."""
    mid = row["id"]
    if mid in CRIT_OVERRIDE:
        return CRIT_OVERRIDE[mid]

    cat = row.get("category", "")
    issue = (row.get("issue") or "").lower()

    # 2-5 hit fixed-3: near EV of ~3.2 → low for MCTS
    if "2-5 hit move always deals fixed 3 hits" in issue:
        return (
            "P3",
            "Hit count ≈ distribution mean (~3.2). Skill Link / Loaded Dice already handled. "
            "Variance is lost, expectation is not; low impact on MCTS move choice.",
        )

    if cat == "doubles_out_of_scope":
        return ("P4", "Singles engine; doubles tools should not affect MCTS policy.")

    if cat == "ohko":
        return ("P4", "OHKOs are clause-banned or irrelevant in the formats this engine targets.")

    if cat == "copy_transform":
        return (
            "P4",
            "Metronome / Assist / Mirror Move / Sketch / etc. are not serious singles search actions.",
        )

    if cat == "approximation":
        return ("P3", "Documented statistical shortcut; check for optimistic bias case-by-case.")

    if cat == "field_control":
        # terrains overridden above; rooms/gravity less common but can matter
        if mid in {"GRAVITY", "MAGICROOM", "WONDERROOM"}:
            return (
                "P2",
                "Uncommon in singles but can change immunities / item / speed interactions when used.",
            )
        return (
            "P2",
            "Field sport / niche control; rarely the best action, but currently a no-op if chosen.",
        )

    if cat == "type_ability_change":
        if mid in {"SOAK", "MAGICPOWDER", "FORESTSCURSE", "TRICKORTREAT"}:
            return (
                "P1",
                "Type-changing support can enable KO lines; missing effect hides those futures.",
            )
        return (
            "P2",
            "Ability/type gizmos are situational; wrong if chosen, rarely central to MCTS policy.",
        )

    if cat == "trapping_lockon":
        return (
            "P1",
            "Missing trap/lock-on lets the search over-value illegal switches or miss sure hits.",
        )

    if cat == "partial_variable_bp":
        return (
            "P2",
            "Variable BP wrong/zero; impact depends on whether the move is actually used.",
        )

    if cat == "partial_lock_ramp":
        return (
            "P2",
            "Ramp moves are niche; first-hit BP may still be non-zero so not fully blank.",
        )

    if cat == "partial_fail_condition":
        return (
            "P2",
            "Last Resort always available overvalues a niche move when other moves exist.",
        )

    if cat == "partial":
        return ("P2", "Incomplete multi-turn / stockpile-style mechanic; situational.")

    if cat == "variable_bp_or_fixed":
        return (
            "P2",
            "Fixed-damage / variable shells deal nothing; usually gimmick picks in singles.",
        )

    if cat == "stat_control_misc":
        return ("P2", "Swap/split/acupressure-style tools are rare in standard singles search.")

    if cat == "misc_status":
        return ("P2", "Status shell; only matters if that exact utility is on the moveset.")

    return ("P2", "Incomplete relative to Bulbapedia/Showdown; situational for MCTS.")


def bulb(name: str) -> str:
    page = name.replace(" ", "_").replace("'", "%27")
    return f"https://bulbapedia.bulbagarden.net/wiki/{page}_(move)"


def gen_from_move_num(num: int | None) -> int | None:
    """Best-effort introduction generation from national move number."""
    if num is None:
        return None
    thresholds = [
        (165, 1),
        (251, 2),
        (354, 3),
        (467, 4),
        (559, 5),
        (621, 6),
        (742, 7),
        (850, 8),
        (10_000, 9),
    ]
    for upper, gen in thresholds:
        if num <= upper:
            return gen
    return 9


def gens_label(sid: str, fallback: str = "see Bulbapedia") -> str:
    d = SD.get(sid)
    if not isinstance(d, dict):
        return fallback
    gen = d.get("gen") or gen_from_move_num(d.get("num"))
    if not gen:
        return fallback
    # Frustration removed in gen9; Return still exists historically
    if sid == "frustration":
        return f"{gen}-8"
    if gen >= 9:
        return "9"
    return f"{gen}-9"


def line_of_moves() -> dict[str, int]:
    out: dict[str, int] = {}
    for m in re.finditer(r"moves\.insert\(\s*Choices::([A-Z0-9]+)\s*,", CHOICES):
        name = m.group(1)
        if name not in out:
            out[name] = CHOICES.count("\n", 0, m.start()) + 1
    return out


def code_link(move_id: str, line: int | None) -> str:
    if line:
        return f"[`src/choices.rs:{line}`](src/choices.rs#L{line}) (`Choices::{move_id}`)"
    return f"[`src/choices.rs`](src/choices.rs) (`Choices::{move_id}`)"


def special_links(files: list[str]) -> str:
    if not files:
        return "_none_"
    return ", ".join(f"[`{f}`]({f})" for f in files)


def main() -> None:
    lines = line_of_moves()
    rows: list[dict] = []
    seen: set[str] = set()

    for x in BAD:
        mid = x["id"]
        if mid in FALSE_POSITIVE:
            continue
        row = dict(x)
        if mid in DOUBLES:
            row["category"] = "doubles_out_of_scope"
            row["issue"] = (
                "Doubles/team-support effect. poke-engine is singles-only, so a stub is "
                "expected rather than a singles mechanics bug."
            )
            row["easy_to_fix"] = "n/a (out of scope)"
        sid = mid.lower()
        if sid in SD and isinstance(SD[sid], dict):
            if "name" in SD[sid]:
                row["name"] = SD[sid]["name"]
                # Engine-only aliases should link to the real move page
                if mid == "RETURN102":
                    row["bulbapedia"] = bulb("Return")
                else:
                    row["bulbapedia"] = bulb(row["name"])
            row["gens"] = gens_label(sid, row.get("gens", "see Bulbapedia"))
        row["line"] = lines.get(mid)
        rows.append(row)
        seen.add(mid)

    for p in PARTIAL:
        mid = p["id"]
        if mid in seen:
            for r in rows:
                if r["id"] == mid:
                    r.update(p)
                    r["bulbapedia"] = bulb(p["name"])
                    r["line"] = lines.get(mid)
                    r["special_files"] = SPECIAL.get(mid, [])
            continue
        rows.append(
            {
                **p,
                "bulbapedia": bulb(p["name"]),
                "code": f"src/choices.rs (Choices::{mid})",
                "special_files": SPECIAL.get(mid, []),
                "line": lines.get(mid),
            }
        )
        seen.add(mid)

    for r in rows:
        pri, why = assign_criticality(r)
        r["priority"] = pri
        r["priority_why"] = why

    rows.sort(
        key=lambda r: (
            CRIT_ORDER.index(r["priority"]),
            r.get("name") or r["id"],
        )
    )

    crit_counts = Counter(r["priority"] for r in rows)
    cat_counts = Counter(r["category"] for r in rows)
    total_choices = AUDIT["total_choices"]
    shellish = sum(
        cat_counts[c] for c in cat_counts if c != "doubles_out_of_scope"
    )
    doubles_n = cat_counts.get("doubles_out_of_scope", 0)

    md: list[str] = []
    md.append("# Move implementation audit (gens 1-9)")
    md.append("")
    md.append("## Summary")
    md.append("")
    md.append(
        f"Audited **{total_choices}** `Choices` variants in poke-engine against "
        "declarative move data (`src/choices.rs`), special-case handlers "
        "(`choice_effects` / `generate_instructions` / `damage_calc` per generation), "
        "and [Pokemon Showdown `moves.json`](https://play.pokemonshowdown.com/data/moves.json) "
        "as a machine-readable effect oracle. Documentation links point at "
        "[Bulbapedia](https://bulbapedia.bulbagarden.net/) move pages."
    )
    md.append("")
    md.append(
        "Entries are sorted by **criticality for MCTS / statistical simulation** "
        "(the engine's real use case: many rollouts to pick a move mid-fight). "
        "Losing variance is cheap when the mean is right; systematic bias or a "
        "blank competitive option is expensive."
    )
    md.append("")
    md.append("| Metric | Count |")
    md.append("|---|---:|")
    md.append(f"| Total `Choices` | {total_choices} |")
    md.append(
        f"| Looks OK (declarative and/or special-cased) | ~{total_choices - len(BAD)}* |"
    )
    md.append(f"| Flagged incomplete / approximate / shell | **{shellish}** |")
    md.append(f"| Doubles/out-of-scope stubs | {doubles_n} |")
    md.append("")
    md.append(
        "\\*OK count is approximate: a move with correct BP/flags can still miss a niche "
        "interaction not covered by this static audit."
    )
    md.append("")
    md.append("### Counts by MCTS criticality")
    md.append("")
    md.append("| Priority | Meaning | Count |")
    md.append("|---|---|---:|")
    for pri in CRIT_ORDER:
        meta = CRIT_META[pri]
        md.append(
            f"| **{pri}** | {meta['title'].split('-', 1)[-1].strip()} | {crit_counts.get(pri, 0)} |"
        )
    md.append("")
    md.append("### Criticality rubric")
    md.append("")
    md.append("| Priority | When to care |")
    md.append("|---|---|")
    md.append(
        "| **P0** | Real singles options that are non-functional or drastically wrong "
        "(search will never value them correctly). |"
    )
    md.append(
        "| **P1** | Systematic optimistic/pessimistic bias on used moves, or missing "
        "field/type/trap state that changes later EV. |"
    )
    md.append(
        "| **P2** | Wrong, but niche / situational for the formats this engine targets. |"
    )
    md.append(
        "| **P3** | Near-unbiased statistical shortcuts (e.g. 2-5 hits -> always 3 ~ mean "
        "~3.2). Fine for MCTS; branching is polish. |"
    )
    md.append(
        "| **P4** | Gimmicks, OHKOs, doubles-only, or essentially unused in singles search. |"
    )
    md.append("")
    md.append(
        "**Example:** fixing \"2-5 hit moves always roll 3\" is low value for MCTS because "
        "the mean is already right. Fixing Rage Fist (always BP 50) or terrain *moves* "
        "(no-op) changes which lines the search believes are good."
    )
    md.append("")
    md.append("### Counts by mechanical class (secondary)")
    md.append("")
    md.append("| Class | Count |")
    md.append("|---|---:|")
    for cat in CAT_TITLES:
        if cat_counts.get(cat):
            md.append(f"| {CAT_TITLES[cat]} | {cat_counts[cat]} |")
    md.append("")
    md.append("## Methodology")
    md.append("")
    md.append("1. Enumerate every `Choices::{MOVE}` and its `moves.insert` `Choice` blocks.")
    md.append(
        "2. Treat a move as a **shell** if it has no effectful declarative fields "
        "(`base_power>0`, boosts, status, volatile, side condition, heal, drain, secondaries) "
        "and no `Choices::MOVE` match arm in gen1/2/3/x `choice_effects`, "
        "`generate_instructions`, or `damage_calc`."
    )
    md.append(
        "3. Collect **documented approximations** from code comments "
        "(multi-hit average, locked-move duration, Population Bomb, etc.)."
    )
    md.append(
        "4. Manually add **partial** implementations that have table data / BP but are missing "
        "the characteristic mechanic (Flail vs Reversal, Rage Fist, Beat Up, Rollout, ...)."
    )
    md.append(
        "5. Cross-check names/gens via Showdown `moves.json`; link each entry to Bulbapedia."
    )
    md.append(
        "6. Rank for **MCTS criticality**: prefer fixing blank/biased competitive options "
        "over recovering variance around a correct mean."
    )
    md.append("")
    md.append("### False positives removed")
    md.append("")
    md.append(
        "- **Roar / Whirlwind**: implemented via `flags.drag` in "
        "`src/genx/generate_instructions.rs`."
    )
    md.append(
        "- **Teleport**: `flags.pivot` in gen 8+; earlier generations correctly do nothing in battle."
    )
    md.append("- Intentional no-ops: Splash, Celebrate, Happy Hour, Hold Hands, `NONE`/`NOTHING`.")
    md.append("")
    md.append("### Generation coverage notes")
    md.append("")
    md.append(
        "- Move **data** for all gens lives in shared `src/choices.rs` (`add_all_moves::<GEN>`)."
    )
    md.append(
        "- **Mechanics** for gens 4-9 share `src/genx/`; gens 1-3 use `src/gen1|gen2|gen3/`."
    )
    md.append(
        "- Terrain **abilities** (Electric/Grassy/Psychic/Misty Surge) can set terrain in genx; "
        "terrain **moves** may be shells or recently implemented depending on branch."
    )
    md.append("")
    md.append("### Weather / terrain duration and extender items")
    md.append("")
    md.append("In real Pokemon, duration can be extended by held items:")
    md.append("")
    md.append(
        "- **Weather moves / weather abilities:** Heat Rock, Damp Rock, Smooth Rock, Icy Rock "
        "make sun/rain/sand/hail last **8** turns instead of **5**."
    )
    md.append(
        "- **Terrain moves / surge abilities:** Terrain Extender makes terrain last **8** turns "
        "instead of **5**."
    )
    md.append("")
    md.append(
        "poke-engine hardcodes **5** turns for weather and terrain, same as Rain Dance / "
        "Sunny Day and Electric Surge / Grassy Surge in this repo. Those extender items are "
        "**not implemented here at all** (no item enums, no duration branching)."
    )
    md.append("")
    md.append(
        "For MCTS this is usually a mild bias on residual turns rather than a blank move, but "
        "it matters for long weather/terrain offense (e.g. sand teams, Expanding Force turns "
        "remaining)."
    )
    md.append("")
    md.append("---")
    md.append("")

    for pri in CRIT_ORDER:
        group = [r for r in rows if r["priority"] == pri]
        if not group:
            continue
        meta = CRIT_META[pri]
        md.append(f"## {meta['title']}")
        md.append("")
        md.append(meta["blurb"])
        md.append("")
        md.append(f"_{len(group)} moves_")
        md.append("")
        for r in group:
            name = r.get("name") or r["id"]
            md.append(f"### {name} (`{r['id']}`)")
            md.append("")
            md.append(f"- **MCTS criticality:** {r['priority']} - {r['priority_why']}")
            md.append(f"- **Generations:** {r.get('gens', 'see Bulbapedia')}")
            md.append(
                f"- **Mechanical class:** {CAT_TITLES.get(r.get('category', ''), r.get('category'))}"
            )
            md.append(f"- **What is wrong:** {r['issue']}")
            md.append(f"- **Easy to fix?** {r.get('easy_to_fix', 'unknown')}")
            if r.get("note"):
                md.append(f"- **Fix hint:** {r['note']}")
            md.append(f"- **Bulbapedia:** [{name}]({r['bulbapedia']})")
            md.append(f"- **Move table:** {code_link(r['id'], r.get('line'))}")
            md.append(
                f"- **Special-case handlers:** {special_links(r.get('special_files') or [])}"
            )
            md.append("")

    md.append("---")
    md.append("")
    md.append("## Regenerating this report")
    md.append("")
    md.append("```bash")
    md.append("python docs/moves/audit_moves.py")
    md.append("python docs/moves/generate_move_report.py")
    md.append("```")
    md.append("")
    md.append(
        "Artifacts: `docs/moves/move_audit.json`, `docs/moves/move_audit_bad.json`, "
        "`docs/moves/.cache_moves.json`."
    )
    md.append("")

    out = DOCS_MOVES / "MOVE_IMPLEMENTATION_REPORT.md"
    out.write_text("\n".join(md), encoding="utf-8")
    print(f"Wrote {out} ({len(rows)} entries)")
    print("by criticality:", dict(crit_counts))
    print("by class:", dict(cat_counts))


if __name__ == "__main__":
    main()
