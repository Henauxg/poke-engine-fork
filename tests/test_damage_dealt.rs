#![cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]

use poke_engine::choices::{Choices, MoveCategory};
use poke_engine::engine::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::engine::state::{MoveChoice, PokemonVolatileStatus, Weather};
use poke_engine::instruction::{
    ChangeDamageDealtDamageInstruction, ChangeDamageDealtMoveCategoryInstruction,
    DamageInstruction, Instruction, RemoveVolatileStatusInstruction, StateInstructions,
    ToggleDamageDealtHitSubstituteInstruction,
};
use poke_engine::state::{PokemonMoveIndex, PokemonType, SideReference, State};

// The genx engine serves gens 4..=9 from one build. Each `gen_tests!` module re-includes
// the shared test bodies (tests/impls/damage_dealt_impl.rs) with a different `const GEN`,
// so a single `cargo test` runs every test once per generation (genN::test_x). The
// bodies turbofish the engine entry points with the enclosing module's `GEN`.
macro_rules! gen_tests {
    ($modname:ident, $gen:literal) => {
        mod $modname {
            use super::*;
            const GEN: u8 = $gen;
            include!("impls/damage_dealt_impl.rs");
        }
    };
}

#[cfg(not(feature = "champions"))]
gen_tests!(gen4, 4);
#[cfg(not(feature = "champions"))]
gen_tests!(gen5, 5);
#[cfg(not(feature = "champions"))]
gen_tests!(gen6, 6);
#[cfg(not(feature = "champions"))]
gen_tests!(gen7, 7);
#[cfg(not(feature = "champions"))]
gen_tests!(gen8, 8);
gen_tests!(gen9, 9);
