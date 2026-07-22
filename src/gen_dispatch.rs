//! Generation dispatch layer.
//!
//! This library serves generations 1..=9 from a single build. Generations 4-9 come from
//! the `genx` engine, which is generic over `const GEN: u8`; generations 1, 2 and 3 are
//! separate engine implementations. Two kinds of callers need bridging:
//!
//! 1. The shared MCTS / search / IO code, which is itself generic over `GEN`. The
//!    `dispatch::*` wrappers take `GEN` and fold to a direct call into the right engine,
//!    so that code never needs to know which engine backs a generation.
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

    // `GEN` is a constant per instantiation, so every branch below folds away: a
    // `::<5>` call compiles to a direct call into genx, a `::<1>` call to gen1, etc.
    // Generations 1-3 are separate engine implementations; 4-9 come from genx.
    //
    // The genx arms use literal consts (`::<4>` .. `::<9>`) rather than `::<GEN>`.
    // Monomorphization happens before the match folds, so `::<GEN>` would instantiate
    // genx with GEN = 1..3 on the (unreachable) fallthrough and trip genx's
    // `AssertGenInRange` compile-time guard.

    #[inline(always)]
    pub fn generate_instructions_from_move_pair<const GEN: u8>(
        state: &mut State,
        s1: &MoveChoice,
        s2: &MoveChoice,
        branch_on_damage: bool,
    ) -> Vec<StateInstructions> {
        match GEN {
            1 => crate::gen1::generate_instructions::generate_instructions_from_move_pair(
                state,
                s1,
                s2,
                branch_on_damage,
            ),
            2 => crate::gen2::generate_instructions::generate_instructions_from_move_pair(
                state,
                s1,
                s2,
                branch_on_damage,
            ),
            3 => crate::gen3::generate_instructions::generate_instructions_from_move_pair(
                state,
                s1,
                s2,
                branch_on_damage,
            ),
            4 => crate::genx::generate_instructions::generate_instructions_from_move_pair::<4>(
                state,
                s1,
                s2,
                branch_on_damage,
            ),
            5 => crate::genx::generate_instructions::generate_instructions_from_move_pair::<5>(
                state,
                s1,
                s2,
                branch_on_damage,
            ),
            6 => crate::genx::generate_instructions::generate_instructions_from_move_pair::<6>(
                state,
                s1,
                s2,
                branch_on_damage,
            ),
            7 => crate::genx::generate_instructions::generate_instructions_from_move_pair::<7>(
                state,
                s1,
                s2,
                branch_on_damage,
            ),
            8 => crate::genx::generate_instructions::generate_instructions_from_move_pair::<8>(
                state,
                s1,
                s2,
                branch_on_damage,
            ),
            _ => crate::genx::generate_instructions::generate_instructions_from_move_pair::<9>(
                state,
                s1,
                s2,
                branch_on_damage,
            ),
        }
    }

    #[inline(always)]
    pub fn get_all_options<const GEN: u8>(state: &State) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        match GEN {
            1 => state.gen1_get_all_options(),
            2 => state.gen2_get_all_options(),
            3 => state.gen3_get_all_options(),
            4 => state.get_all_options::<4>(),
            5 => state.get_all_options::<5>(),
            6 => state.get_all_options::<6>(),
            7 => state.get_all_options::<7>(),
            8 => state.get_all_options::<8>(),
            _ => state.get_all_options::<9>(),
        }
    }

    #[inline(always)]
    pub fn root_get_all_options<const GEN: u8>(
        state: &State,
    ) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        match GEN {
            1 => state.gen1_root_get_all_options(),
            2 => state.gen2_root_get_all_options(),
            3 => state.gen3_root_get_all_options(),
            4 => state.root_get_all_options::<4>(),
            5 => state.root_get_all_options::<5>(),
            6 => state.root_get_all_options::<6>(),
            7 => state.root_get_all_options::<7>(),
            8 => state.root_get_all_options::<8>(),
            _ => state.root_get_all_options::<9>(),
        }
    }

    /// Static evaluation of a position. Each engine has its own heuristic.
    #[inline(always)]
    pub fn evaluate<const GEN: u8>(state: &State) -> f32 {
        match GEN {
            1 => crate::gen1::evaluate::evaluate(state),
            2 => crate::gen2::evaluate::evaluate(state),
            3 => crate::gen3::evaluate::evaluate(state),
            _ => crate::genx::evaluate::evaluate(state),
        }
    }

    /// Turn on the per-generation optional mechanics after deserializing a state.
    #[inline(always)]
    pub fn set_conditional_mechanics<const GEN: u8>(state: &mut State) {
        match GEN {
            1 => state.gen1_set_conditional_mechanics(),
            2 => state.gen2_set_conditional_mechanics(),
            3 => state.gen3_set_conditional_mechanics(),
            _ => state.set_conditional_mechanics(),
        }
    }

    #[inline(always)]
    pub fn deserialize<const GEN: u8>(serialized: &str) -> State {
        State::deserialize::<GEN>(serialized)
    }

    #[inline(always)]
    pub fn calculate_both_damage_rolls<const GEN: u8>(
        state: &State,
        s1_choice: crate::choices::Choice,
        s2_choice: crate::choices::Choice,
        side_one_moves_first: bool,
    ) -> (Option<Vec<i16>>, Option<Vec<i16>>) {
        match GEN {
            1 => crate::gen1::generate_instructions::calculate_both_damage_rolls(
                state,
                s1_choice,
                s2_choice,
                side_one_moves_first,
            ),
            2 => crate::gen2::generate_instructions::calculate_both_damage_rolls(
                state,
                s1_choice,
                s2_choice,
                side_one_moves_first,
            ),
            3 => crate::gen3::generate_instructions::calculate_both_damage_rolls(
                state,
                s1_choice,
                s2_choice,
                side_one_moves_first,
            ),
            4 => crate::genx::generate_instructions::calculate_both_damage_rolls::<4>(
                state,
                s1_choice,
                s2_choice,
                side_one_moves_first,
            ),
            5 => crate::genx::generate_instructions::calculate_both_damage_rolls::<5>(
                state,
                s1_choice,
                s2_choice,
                side_one_moves_first,
            ),
            6 => crate::genx::generate_instructions::calculate_both_damage_rolls::<6>(
                state,
                s1_choice,
                s2_choice,
                side_one_moves_first,
            ),
            7 => crate::genx::generate_instructions::calculate_both_damage_rolls::<7>(
                state,
                s1_choice,
                s2_choice,
                side_one_moves_first,
            ),
            8 => crate::genx::generate_instructions::calculate_both_damage_rolls::<8>(
                state,
                s1_choice,
                s2_choice,
                side_one_moves_first,
            ),
            _ => crate::genx::generate_instructions::calculate_both_damage_rolls::<9>(
                state,
                s1_choice,
                s2_choice,
                side_one_moves_first,
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// Runtime facade: turn a `gen: u8` value into the right instantiation, once.
// `champions`/`bss` builds behave as generation 9; pass gen = 9 for them.
// ---------------------------------------------------------------------------

/// Dispatch a runtime generation to the matching `const GEN` instantiation of `$f`.
macro_rules! for_each_gen {
    ($gen:expr, $f:ident $(, $arg:expr)* $(,)?) => {
        match $gen {
            1 => dispatch::$f::<1>($($arg),*),
            2 => dispatch::$f::<2>($($arg),*),
            3 => dispatch::$f::<3>($($arg),*),
            4 => dispatch::$f::<4>($($arg),*),
            5 => dispatch::$f::<5>($($arg),*),
            6 => dispatch::$f::<6>($($arg),*),
            7 => dispatch::$f::<7>($($arg),*),
            8 => dispatch::$f::<8>($($arg),*),
            9 => dispatch::$f::<9>($($arg),*),
            other => panic!(
                "unsupported generation {}: this build serves {}..={}",
                other,
                crate::MIN_GEN,
                crate::MAX_GEN
            ),
        }
    };
}

/// Runtime-generation entry points for callers holding the generation as data.
pub mod for_gen {
    use super::*;

    /// Deserialize a state string using generation `gen`'s move data and mechanics.
    pub fn deserialize(gen: u8, serialized: &str) -> State {
        for_each_gen!(gen, deserialize, serialized)
    }

    /// Generate the instruction branches for a move pair under generation `gen`.
    pub fn generate_instructions_from_move_pair(
        gen: u8,
        state: &mut State,
        s1: &MoveChoice,
        s2: &MoveChoice,
        branch_on_damage: bool,
    ) -> Vec<StateInstructions> {
        for_each_gen!(
            gen,
            generate_instructions_from_move_pair,
            state,
            s1,
            s2,
            branch_on_damage,
        )
    }

    /// All legal option pairs at the root under generation `gen`.
    pub fn root_get_all_options(gen: u8, state: &State) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        for_each_gen!(gen, root_get_all_options, state)
    }

    /// Static evaluation of a position under generation `gen`.
    pub fn evaluate(gen: u8, state: &State) -> f32 {
        for_each_gen!(gen, evaluate, state)
    }
}
