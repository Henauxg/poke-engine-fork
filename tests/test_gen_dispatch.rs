//! End-to-end checks that one build really serves every supported generation at runtime.
//!
//! Generations 4-9 come from the const-generic `genx` engine and 1-3 from their own
//! engines; both are reached through the same `gen_dispatch` facade. These tests guard
//! the dispatch wiring itself, not battle mechanics (the per-generation mechanics suites
//! do that).

use poke_engine::gen_dispatch::for_gen;
use poke_engine::state::State;
use poke_engine::{MAX_GEN, MIN_GEN};

fn all_gens() -> impl Iterator<Item = u8> {
    MIN_GEN..=MAX_GEN
}

#[test]
fn every_supported_generation_round_trips_a_state() {
    let serialized = State::default().serialize();
    for gen in all_gens() {
        let state = for_gen::deserialize(gen, &serialized);
        assert_eq!(
            serialized,
            state.serialize(),
            "state did not round-trip for generation {}",
            gen
        );
    }
}

#[test]
fn every_supported_generation_produces_root_options() {
    let serialized = State::default().serialize();
    for gen in all_gens() {
        let state = for_gen::deserialize(gen, &serialized);
        let (s1, s2) = for_gen::root_get_all_options(gen, &state);
        assert!(
            !s1.is_empty() && !s2.is_empty(),
            "generation {} produced no root options",
            gen
        );
    }
}

#[test]
fn every_supported_generation_evaluates_and_generates_instructions() {
    let serialized = State::default().serialize();
    for gen in all_gens() {
        let mut state = for_gen::deserialize(gen, &serialized);
        // evaluate() is a per-engine heuristic; just assert it is a real number.
        assert!(
            for_gen::evaluate(gen, &state).is_finite(),
            "generation {} produced a non-finite evaluation",
            gen
        );

        let (s1, s2) = for_gen::root_get_all_options(gen, &state);
        let before = state.serialize();
        let instructions =
            for_gen::generate_instructions_from_move_pair(gen, &mut state, &s1[0], &s2[0], false);
        assert!(
            !instructions.is_empty(),
            "generation {} produced no instructions",
            gen
        );
        // generating instructions must leave the state untouched
        assert_eq!(
            before,
            state.serialize(),
            "generation {} mutated the state while generating instructions",
            gen
        );
    }
}

#[test]
#[should_panic(expected = "unsupported generation")]
fn an_unsupported_generation_is_rejected() {
    for_gen::deserialize(MAX_GEN + 1, &State::default().serialize());
}
