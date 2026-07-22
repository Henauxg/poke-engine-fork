//! Generation dispatch layer.
//!
//! The `genx` engine is generic over `const GEN: u8` (gens 4..=9). Two kinds of callers
//! need help bridging to it:
//!
//! 1. The shared MCTS / search / IO code, which must itself be generic over `GEN` but is
//!    compiled for BOTH the genx engine and the standalone gen1/2/3 engines. The
//!    `dispatch::*` wrappers are generic over `GEN` on every path: on genx they forward
//!    `GEN`; on gen1/2/3 they ignore it and call the compile-time-selected engine. This
//!    keeps the gen1/2/3 modules untouched while letting the shared code turbofish
//!    `::<GEN>` uniformly.
//!
//! 2. Callers that receive the generation as runtime DATA (a `u8`), e.g. the CLI's
//!    `--gen` flag or an FFI boundary. The `for_gen::*` functions match the runtime value
//!    to the right monomorphized instantiation once, at the crate edge.

use crate::engine::state::MoveChoice;
use crate::instruction::StateInstructions;
use crate::state::State;

/// Generic wrappers used by the shared engine-agnostic code (mcts, search, io). Every
/// wrapper is generic over `GEN` so callers can turbofish uniformly regardless of engine.
pub mod dispatch {
    use super::*;

    // `generate_instructions_from_move_pair` and the option methods are generic only on
    // the genx path; on gen1/2/3 they are non-generic, hence the cfg split. `GEN` is
    // ignored on the gen1/2/3 path (it is always `CURRENT_GEN` there anyway).

    #[cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]
    #[inline(always)]
    pub fn generate_instructions_from_move_pair<const GEN: u8>(
        state: &mut State,
        s1: &MoveChoice,
        s2: &MoveChoice,
        branch_on_damage: bool,
    ) -> Vec<StateInstructions> {
        crate::engine::generate_instructions::generate_instructions_from_move_pair::<GEN>(
            state,
            s1,
            s2,
            branch_on_damage,
        )
    }
    #[cfg(any(feature = "gen1", feature = "gen2", feature = "gen3"))]
    #[inline(always)]
    pub fn generate_instructions_from_move_pair<const GEN: u8>(
        state: &mut State,
        s1: &MoveChoice,
        s2: &MoveChoice,
        branch_on_damage: bool,
    ) -> Vec<StateInstructions> {
        crate::engine::generate_instructions::generate_instructions_from_move_pair(
            state,
            s1,
            s2,
            branch_on_damage,
        )
    }

    #[cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]
    #[inline(always)]
    pub fn get_all_options<const GEN: u8>(state: &State) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        state.get_all_options::<GEN>()
    }
    #[cfg(any(feature = "gen1", feature = "gen2", feature = "gen3"))]
    #[inline(always)]
    pub fn get_all_options<const GEN: u8>(state: &State) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        state.get_all_options()
    }

    #[cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]
    #[inline(always)]
    pub fn root_get_all_options<const GEN: u8>(
        state: &State,
    ) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        state.root_get_all_options::<GEN>()
    }
    #[cfg(any(feature = "gen1", feature = "gen2", feature = "gen3"))]
    #[inline(always)]
    pub fn root_get_all_options<const GEN: u8>(
        state: &State,
    ) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        state.root_get_all_options()
    }

    // `State::deserialize` is generic over `GEN` on every path (the shared crate-root
    // `state.rs` threads it uniformly), so no cfg split is needed here.
    #[inline(always)]
    pub fn deserialize<const GEN: u8>(serialized: &str) -> State {
        State::deserialize::<GEN>(serialized)
    }

    #[cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]
    #[inline(always)]
    pub fn calculate_both_damage_rolls<const GEN: u8>(
        state: &State,
        s1_choice: crate::choices::Choice,
        s2_choice: crate::choices::Choice,
        side_one_moves_first: bool,
    ) -> (Option<Vec<i16>>, Option<Vec<i16>>) {
        crate::engine::generate_instructions::calculate_both_damage_rolls::<GEN>(
            state,
            s1_choice,
            s2_choice,
            side_one_moves_first,
        )
    }
    #[cfg(any(feature = "gen1", feature = "gen2", feature = "gen3"))]
    #[inline(always)]
    pub fn calculate_both_damage_rolls<const GEN: u8>(
        state: &State,
        s1_choice: crate::choices::Choice,
        s2_choice: crate::choices::Choice,
        side_one_moves_first: bool,
    ) -> (Option<Vec<i16>>, Option<Vec<i16>>) {
        crate::engine::generate_instructions::calculate_both_damage_rolls(
            state,
            s1_choice,
            s2_choice,
            side_one_moves_first,
        )
    }
}

// ---------------------------------------------------------------------------
// Runtime facade (genx only): turn a `gen: u8` value into the right instantiation.
// `champions`/`bss` builds always behave as generation 9; pass gen = 9 for them.
// ---------------------------------------------------------------------------

/// Runtime-generation entry points for callers holding the generation as data.
#[cfg(not(any(feature = "gen1", feature = "gen2", feature = "gen3")))]
pub mod for_gen {
    use super::*;

    /// Deserialize a state string using generation `gen`'s move data.
    pub fn deserialize(gen: u8, serialized: &str) -> State {
        match gen {
            4 => State::deserialize::<4>(serialized),
            5 => State::deserialize::<5>(serialized),
            6 => State::deserialize::<6>(serialized),
            7 => State::deserialize::<7>(serialized),
            8 => State::deserialize::<8>(serialized),
            9 => State::deserialize::<9>(serialized),
            other => panic!("unsupported generation {}: genx serves 4..=9", other),
        }
    }

    /// Generate the instruction branches for a move pair under generation `gen`.
    pub fn generate_instructions_from_move_pair(
        gen: u8,
        state: &mut State,
        s1: &MoveChoice,
        s2: &MoveChoice,
        branch_on_damage: bool,
    ) -> Vec<StateInstructions> {
        use crate::engine::generate_instructions::generate_instructions_from_move_pair as gi;
        match gen {
            4 => gi::<4>(state, s1, s2, branch_on_damage),
            5 => gi::<5>(state, s1, s2, branch_on_damage),
            6 => gi::<6>(state, s1, s2, branch_on_damage),
            7 => gi::<7>(state, s1, s2, branch_on_damage),
            8 => gi::<8>(state, s1, s2, branch_on_damage),
            9 => gi::<9>(state, s1, s2, branch_on_damage),
            other => panic!("unsupported generation {}: genx serves 4..=9", other),
        }
    }

    /// All legal option pairs at the root under generation `gen`.
    pub fn root_get_all_options(gen: u8, state: &State) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        match gen {
            4 => state.root_get_all_options::<4>(),
            5 => state.root_get_all_options::<5>(),
            6 => state.root_get_all_options::<6>(),
            7 => state.root_get_all_options::<7>(),
            8 => state.root_get_all_options::<8>(),
            9 => state.root_get_all_options::<9>(),
            other => panic!("unsupported generation {}: genx serves 4..=9", other),
        }
    }
}
