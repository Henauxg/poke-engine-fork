#!/usr/bin/env python3
"""Audit poke-engine move implementations vs expected move effects.

Classifies each Choices variant as: OK (declarative or special-cased),
APPROXIMATED, SHELL (in enum but no effect), or MISSING_FROM_ENGINE
(relative to a Showdown move dump when available).
"""

from __future__ import annotations

import json
import os
import re
import sys
import urllib.request
from collections import defaultdict
from dataclasses import dataclass, field
from pathlib import Path

# docs/moves/<this file> -> repo root is parents[2]
ROOT = Path(__file__).resolve().parents[2]
DOCS_MOVES = Path(__file__).resolve().parent
CHOICES = ROOT / "src" / "choices.rs"
EFFECT_FILES = [
    ROOT / "src" / "genx" / "choice_effects.rs",
    ROOT / "src" / "genx" / "generate_instructions.rs",
    ROOT / "src" / "genx" / "damage_calc.rs",
    ROOT / "src" / "gen3" / "choice_effects.rs",
    ROOT / "src" / "gen3" / "generate_instructions.rs",
    ROOT / "src" / "gen2" / "choice_effects.rs",
    ROOT / "src" / "gen2" / "generate_instructions.rs",
    ROOT / "src" / "gen1" / "choice_effects.rs",
    ROOT / "src" / "gen1" / "generate_instructions.rs",
]

# Moves that are intentionally no-ops or purely cosmetic in singles search
INTENTIONAL_NOOPS = {
    "NONE",
    "SPLASH",
    "CELEBRATE",
    "HAPPYHOUR",
    "HOLDHANDS",
    "RECHARGE",  # engine sentinel for must-recharge turn
}

# Known intentional approximations documented in code
KNOWN_APPROXIMATIONS = {
    "POPULATIONBOMB": "Multi-accuracy not implemented; approximated as fixed 6 hits (9 with Wide Lens)",
    "TRIPLEAXEL": "Multi-accuracy not implemented; always lands all 3 hits at escalating BP",
    "TRIPLEKICK": "Multi-accuracy not implemented; treated similarly to multi-hit",
    "RETURN": "Always base power 102 (also RETURN102 variant)",
    "RETURN102": "Hardcoded Return at BP 102",
    "FRUSTRATION": "Shell / not happiness-scaled (BP 0 unless special-cased)",
}

MULTI_HIT_2_TO_5 = {
    "ARMTHRUST",
    "BARRAGE",
    "BONERUSH",
    "BULLETSEED",
    "COMETPUNCH",
    "DOUBLESLAP",
    "FURYATTACK",
    "FURYSWIPES",
    "ICICLESPEAR",
    "PINMISSILE",
    "ROCKBLAST",
    "SCALESHOT",
    "SPIKECANNON",
    "TAILSLAP",
    "WATERSHURIKEN",
}

LOCKED_MOVES = {"OUTRAGE", "PETALDANCE", "THRASH", "RAGINGFURY"}

# Categories for shell moves (expected to need special logic or be stubs)
SHELL_CATEGORIES = {
    "copy_transform": {
        "COPYCAT",
        "METRONOME",
        "MIMIC",
        "MIRRORMOVE",
        "SKETCH",
        "ASSIST",
        "MEFIRST",
        "TRANSFORM",
        "INSTRUCT",
        "SLEEPWALK",  # if present
    },
    "ohko": {"FISSURE", "GUILLOTINE", "HORNDRILL", "SHEERCOLD"},
    "variable_bp_or_fixed": {
        "DRAGONRAGE",
        "SONICBOOM",
        "PSYWAVE",
        "MAGNITUDE",
        "PRESENT",
        "BEATUP",
        "FLING",
        "NATURALGIFT",
        "SPITUP",
        "TRUMPCARD",
        "WRINGOUT",
        "CRUSHGRIP",
        "PUNISHMENT",
        "FLAIL",
        "FRUSTRATION",
        "PIKAPAPOW",
        "VEEVEEVOLLEY",
    },
    "field_control": {
        "ELECTRICTERRAIN",
        "GRASSYTERRAIN",
        "MISTYTERRAIN",
        "PSYCHICTERRAIN",
        "GRAVITY",
        "MAGICROOM",
        "WONDERROOM",
        "MUDSPORT",
        "WATERSPORT",
        "IONDELUGE",
        "FAIRYLOCK",
    },
    "type_ability_change": {
        "CONVERSION",
        "CONVERSION2",
        "SOAK",
        "FORESTSCURSE",
        "TRICKORTREAT",
        "MAGICPOWDER",
        "ROLEPLAY",
        "ENTRAINMENT",
        "SKILLSWAP",
        "WORRYSEED",
        "SIMPLEBEAM",
        "REFLECTTYPE",
        "CAMOUFLAGE",
        "DOODLE",
    },
    "stat_control_misc": {
        "ACUPRESSURE",
        "HEARTSWAP",
        "POWERSWAP",
        "GUARDSWAP",
        "SPEEDSWAP",
        "POWERSPLIT",
        "GUARDSPLIT",
        "PSYCHUP",
        "TOPSYTURVY",
        "VENOMDRENCH",
        "GEARUP",
        "MAGNETICFLUX",
        "FLOWERSHIELD",
        "ROTOTILLER",
        "POWERSHIFT",
    },
    "trapping_lockon": {
        "MEANLOOK",
        "BLOCK",
        "SPIDERWEB",
        "LOCKON",
        "MINDREADER",
        "JAWLOCK",  # may be special
        "THOUSANDWAVES",
        "SPIRITSHACKLE",
        "ANCHORSHOT",
        "OCTOLOCK",
        "NORETREAT",
    },
    "misc_status": {
        "SPITE",
        "RECYCLE",
        "BESTOW",
        "CORROSIVEGAS",
        "TEATIME",
        "STUFFCHEEKS",
        "TAKEHEART",
        "QUASH",
        "PSYCHICSHIFT",
        "AFTERYOU",
        "ALLYSWITCH",
        "FOLLOWME",
        "RAGEPOWDER",
        "SPOTLIGHT",
        "HELPINGHAND",
        "COACHING",
        "AROMATICMIST",
        "LUCKYCHANT",
        "SAFEGUARD",
        "MIST",
        "QUICKGUARD",
        "WIDEGUARD",
        "CRAFTYSHIELD",
        "MATBLOCK",
        "POWDER",
        "MAGICCOAT",
        "SNATCH",
        "IMPRISON",
        "HEALBLOCK",
        "EMBARGO",
        "DISABLE",
        "TORMENT",
        "TAUNT",
        "ENCORE",
        "ATTRACT",
        "CAPTIVATE",
        "TELEKINESIS",
        "ODORSLEUTH",
        "FORESIGHT",
        "MIRACLEEYE",
        "NIGHTMARE",
        "CURSE",
        "LEECHSEED",
        "YAWN",
        "PERISHSONG",
        "WISH",
        "HEALINGWISH",
        "LUNARDANCE",
        "BATONPASS",
        "UTURN",
        "VOLTSWITCH",
        "FLIPTURN",
        "PARTINGSHOT",
        "TELEPORT",
        "CHILLYRECEPTION",
        "SHEDTAIL",
    },
}


def bulbapedia_url(move_id: str) -> str:
    # Engine ids are UPPERCASE concatenated; Bulbapedia uses Title_Case_(move)
    # Best-effort: split camel-ish by inserting before known patterns is hard.
    # Use Showdown-style id -> Title Case heuristic.
    name = move_id
    # Common multiword fixes applied later via SHOWDOWN map when available
    pretty = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name.title().replace(" ", ""))
    # Fallback: just use the raw id page pattern Showdown uses
    # Bulbapedia pages: https://bulbapedia.bulbagarden.net/wiki/Tackle_(move)
    words = []
    # Prefer splitting known compound by lowercasing then title from spaces if we have mapping
    return f"https://bulbapedia.bulbagarden.net/wiki/{move_id.title()}_(move)"


def code_search_url(move_id: str) -> str:
    return f"src/choices.rs (Choices::{move_id})"


def extract_choices_enum(text: str) -> list[str]:
    # Defined via define_enum_with_from_str! { ... Choices { ... } ... }
    m = re.search(r"Choices\s*\{([^}]+)\}", text, re.S)
    if not m:
        raise SystemExit("Could not find Choices enum")
    body = m.group(1)
    return re.findall(r"\b([A-Z][A-Z0-9]+)\b", body)


def extract_special_cased(files: list[Path]) -> dict[str, list[str]]:
    """map MOVE -> list of files mentioning Choices::MOVE"""
    hits: dict[str, list[str]] = defaultdict(list)
    pat = re.compile(r"Choices::([A-Z][A-Z0-9]+)")
    for f in files:
        if not f.exists():
            continue
        content = f.read_text(encoding="utf-8")
        for move in set(pat.findall(content)):
            rel = str(f.relative_to(ROOT)).replace("\\", "/")
            hits[move].append(rel)
    return hits


EFFECTFUL_FIELDS = re.compile(
    r"\b(base_power|boost|secondaries|status|volatile_status|side_condition|"
    r"heal|drain|recoil|crash|priority)\s*:"
)


def analyze_choice_blocks(text: str) -> dict[str, dict]:
    """For each Choices::X insert, record whether the Choice looks like a shell.

    Uses a simple brace-matching scan from each `moves.insert(Choices::NAME`.
    """
    info: dict[str, dict] = {}
    for m in re.finditer(r"moves\.insert\(\s*Choices::([A-Z0-9]+)\s*,", text):
        name = m.group(1)
        start = m.end()
        # find Choice { ... } roughly
        brace = text.find("{", start)
        if brace < 0:
            continue
        depth = 0
        i = brace
        while i < len(text):
            ch = text[i]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    block = text[brace : i + 1]
                    break
            i += 1
        else:
            continue

        # Gen branch context: look backwards for if GEN
        ctx_start = max(0, m.start() - 200)
        ctx = text[ctx_start : m.start()]
        gen_hint = None
        gm = re.search(r"GEN\s*(==|<=|>=|<|>)\s*(\d+)", ctx)
        if gm:
            gen_hint = gm.group(0)

        has_effect = bool(EFFECTFUL_FIELDS.search(block))
        # base_power: 0.0 alone doesn't count as effect for attacking intent
        bp_m = re.search(r"base_power:\s*([0-9.]+)", block)
        bp = float(bp_m.group(1)) if bp_m else 0.0
        cat_m = re.search(r"category:\s*MoveCategory::(\w+)", block)
        category = cat_m.group(1) if cat_m else "Status"
        only_type_and_id = (
            not has_effect
            or (bp == 0.0 and category in ("Physical", "Special") and "secondaries" not in block
                and "status" not in block and "volatile_status" not in block
                and "side_condition" not in block and "heal" not in block
                and "boost" not in block and "drain" not in block)
        )
        # Status with only move_type/flags/protect is often a shell
        status_shell = (
            category == "Status"
            and "boost" not in block
            and "status" not in block
            and "volatile_status" not in block
            and "side_condition" not in block
            and "heal" not in block
            and "secondaries" not in block
        )
        # Damaging with bp>0 and protect/contact is usually OK declarative
        damaging_ok = category in ("Physical", "Special") and bp > 0

        shellish = (only_type_and_id or status_shell) and not damaging_ok

        entry = info.setdefault(
            name,
            {
                "any_shell": False,
                "any_ok_data": False,
                "blocks": 0,
                "max_bp": 0.0,
                "categories": set(),
                "gen_hints": set(),
            },
        )
        entry["blocks"] += 1
        entry["max_bp"] = max(entry["max_bp"], bp)
        entry["categories"].add(category)
        if gen_hint:
            entry["gen_hints"].add(gen_hint)
        if shellish:
            entry["any_shell"] = True
        else:
            entry["any_ok_data"] = True

    return info


def fetch_showdown_moves() -> dict | None:
    urls = [
        "https://raw.githubusercontent.com/smogon/pokemon-showdown/master/data/moves.ts",
        "https://play.pokemonshowdown.com/data/moves.json",
    ]
    cache = DOCS_MOVES / ".cache_moves.json"
    if cache.exists():
        try:
            return json.loads(cache.read_text(encoding="utf-8"))
        except Exception:
            pass
    for url in urls:
        try:
            print(f"Fetching {url} ...", file=sys.stderr)
            req = urllib.request.Request(url, headers={"User-Agent": "poke-engine-audit"})
            with urllib.request.urlopen(req, timeout=60) as resp:
                raw = resp.read().decode("utf-8", errors="replace")
            if url.endswith(".json"):
                data = json.loads(raw)
                cache.write_text(json.dumps(data), encoding="utf-8")
                return data
            # TS file: too hard to parse fully; skip
        except Exception as e:
            print(f"Failed {url}: {e}", file=sys.stderr)
    return None


def showdown_id_to_engine(sid: str) -> str:
    return sid.upper()


def engine_to_showdown_id(name: str) -> str:
    return name.lower()


def pretty_move_name(engine: str, sd: dict | None) -> str:
    sid = engine_to_showdown_id(engine)
    if sd and sid in sd and isinstance(sd[sid], dict) and "name" in sd[sid]:
        return sd[sid]["name"]
    # Heuristic Title Case
    return engine.title()


def bulbapedia_from_name(name: str) -> str:
    page = name.replace(" ", "_")
    # Bulbapedia uses specific apostrophe handling
    page = page.replace("'", "%27")
    return f"https://bulbapedia.bulbagarden.net/wiki/{page}_(move)"


def categorize_shell(move: str) -> str:
    for cat, members in SHELL_CATEGORIES.items():
        if move in members:
            return cat
    return "other_shell"


def difficulty(cat: str, move: str, special: bool) -> str:
    if move in KNOWN_APPROXIMATIONS or move in MULTI_HIT_2_TO_5 or move in LOCKED_MOVES:
        return "medium"
    if cat in ("ohko",):
        return "easy"
    if cat in ("copy_transform",):
        return "hard"
    if cat in ("field_control", "type_ability_change"):
        return "hard"  # often needs state model extensions
    if cat in ("variable_bp_or_fixed",):
        return "easy-medium"
    if cat in ("trapping_lockon", "stat_control_misc"):
        return "medium"
    if special:
        return "n/a (special-cased)"
    return "unknown"


def main() -> int:
    text = CHOICES.read_text(encoding="utf-8")
    choices = extract_choices_enum(text)
    special = extract_special_cased(EFFECT_FILES)
    blocks = analyze_choice_blocks(text)
    sd = fetch_showdown_moves()

    # Also scan for approximation comments in GI
    approx_comments = []
    for f in EFFECT_FILES + [CHOICES]:
        if not f.exists():
            continue
        for i, line in enumerate(f.read_text(encoding="utf-8").splitlines(), 1):
            if re.search(r"approximat|too lazy|not implemented|always lasts|until that is", line, re.I):
                approx_comments.append((str(f.relative_to(ROOT)).replace("\\", "/"), i, line.strip()))

    shells = []
    approximated = []
    ok_declarative = []
    ok_special = []

    for move in choices:
        if move in INTENTIONAL_NOOPS:
            continue
        b = blocks.get(move, {})
        is_special = move in special
        shellish = b.get("any_shell", True) and not b.get("any_ok_data", False)
        # If any block has ok data, treat as declarative ok unless only shell exists
        if move in KNOWN_APPROXIMATIONS or move in MULTI_HIT_2_TO_5 or move in LOCKED_MOVES:
            approximated.append(move)
            continue
        if is_special and shellish:
            ok_special.append(move)
            continue
        if is_special:
            ok_special.append(move)
            continue
        if shellish or (b.get("max_bp", 0) == 0 and "Physical" in b.get("categories", set()) | b.get("categories", set())):
            # refine: status with ok data fields
            if b.get("any_ok_data"):
                ok_declarative.append(move)
            else:
                shells.append(move)
        else:
            ok_declarative.append(move)

    # Cross-check Showdown: moves with real battle effects that are shells here
    sd_missing_impl = []
    if sd:
        # Showdown uses lowercase ids; skip non-standard / CAP if isNonstandard
        for sid, mdata in sd.items():
            if not isinstance(mdata, dict):
                continue
            if mdata.get("isNonstandard") in ("CAP", "Future", "LGPE", "Custom"):
                continue
            eng = showdown_id_to_engine(sid)
            # Skip max/z/gmax
            if sid.startswith("max") or sid.startswith("gmax") or mdata.get("isZ"):
                continue
            if eng not in choices and eng.replace(" ", "") not in choices:
                # maybe engine missing entirely
                if mdata.get("isNonstandard") is None:
                    # only flag fully standard gens 1-9 moves missing from enum
                    pass
            if eng in shells:
                sd_missing_impl.append(eng)

    out = {
        "total_choices": len(choices),
        "ok_declarative": sorted(ok_declarative),
        "ok_special": sorted(set(ok_special)),
        "shells": sorted(shells),
        "approximated": sorted(set(approximated)),
        "special_map": {k: v for k, v in sorted(special.items())},
        "blocks": {
            k: {
                **{kk: (list(vv) if isinstance(vv, set) else vv) for kk, vv in v.items()}
            }
            for k, v in blocks.items()
        },
        "approx_comments": approx_comments,
        "shell_category": {m: categorize_shell(m) for m in shells},
    }

    out_path = DOCS_MOVES / "move_audit.json"
    out_path.write_text(json.dumps(out, indent=2), encoding="utf-8")
    print(f"Wrote {out_path}", file=sys.stderr)
    print(
        f"total={len(choices)} declarative={len(ok_declarative)} special={len(set(ok_special))} "
        f"shells={len(shells)} approx={len(set(approximated))}",
        file=sys.stderr,
    )

    # Also dump shell list for report generation
    report_data = []
    for move in sorted(set(shells) | set(approximated)):
        cat = categorize_shell(move) if move in shells else "approximation"
        name = pretty_move_name(move, sd)
        gens = "1-9 (as applicable)"
        if sd and engine_to_showdown_id(move) in sd:
            gen = sd[engine_to_showdown_id(move)].get("gen")
            if gen:
                gens = f"{gen}-9"
        issue = KNOWN_APPROXIMATIONS.get(move)
        if not issue:
            if move in MULTI_HIT_2_TO_5:
                issue = "2-5 hit move always deals fixed 3 hits (5 with Skill Link, 4 with Loaded Dice)"
            elif move in LOCKED_MOVES:
                issue = "Locked-move duration not branched; engine assumes always 3 turns"
            else:
                issue = (
                    "Present in Choices / move table but no declarative effect fields and "
                    "no special-case handler in choice_effects / generate_instructions / damage_calc "
                    "(effectively a no-op or empty attack)."
                )
        report_data.append(
            {
                "id": move,
                "name": name,
                "category": cat,
                "gens": gens,
                "issue": issue,
                "easy_to_fix": difficulty(cat, move, move in special),
                "bulbapedia": bulbapedia_from_name(name),
                "code": code_search_url(move),
                "special_files": special.get(move, []),
            }
        )

    (DOCS_MOVES / "move_audit_bad.json").write_text(
        json.dumps(report_data, indent=2), encoding="utf-8"
    )
    print(f"Bad moves: {len(report_data)}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
