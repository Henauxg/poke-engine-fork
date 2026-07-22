use poke_engine::choices::{moves, Choices, MoveCategory};
use poke_engine::engine::abilities::Abilities;
use poke_engine::engine::generate_instructions::{
    generate_instructions_from_move_pair, FULLY_PARALYZED_CHANCE, THAW_CHANCE,
};
use poke_engine::engine::items::Items;
use poke_engine::engine::state::{MoveChoice, PokemonVolatileStatus, Terrain, Weather};
use poke_engine::instruction::Instruction::ToggleSideOneForceSwitch;
use poke_engine::instruction::{
    ApplyVolatileStatusInstruction, BoostInstruction, ChangeAbilityInstruction,
    ChangeDamageDealtDamageInstruction, ChangeDamageDealtMoveCategoryInstruction,
    ChangeItemInstruction, ChangeSideConditionInstruction, ChangeStatInstruction,
    ChangeStatusInstruction, ChangeSubsituteHealthInstruction, ChangeTerrain, ChangeType,
    ChangeVolatileStatusDurationInstruction, ChangeWeather, ChangeWishInstruction,
    DamageInstruction, DecrementFutureSightInstruction, DecrementPPInstruction,
    DecrementRestTurnsInstruction, DecrementWishInstruction, DisableMoveInstruction,
    EnableMoveInstruction, FormeChangeInstruction, HealInstruction, Instruction,
    RemoveVolatileStatusInstruction, SetFutureSightInstruction, SetLastUsedMoveInstruction,
    SetSecondMoveSwitchOutMoveInstruction, SetSleepTurnsInstruction, StateInstructions,
    SwitchInstruction, ToggleBatonPassingInstruction, ToggleMegaEvolvedInstruction,
    ToggleShedTailingInstruction, ToggleTerastallizedInstruction, ToggleTrickRoomInstruction,
};
use poke_engine::pokemon::PokemonName;
use poke_engine::state::{
    pokemon_index_iter, LastUsedMove, Move, PokemonBoostableStat, PokemonIndex, PokemonMoveIndex,
    PokemonSideCondition, PokemonStatus, PokemonType, SideReference, State, StateWeather,
};

// The genx engine serves gens 4..=9 from one build. Each `gen_tests!` module re-includes
// the shared test bodies (tests/impls/battle_mechanics_impl.rs) with a different `const GEN`,
// so a single `cargo test` runs every test once per generation (genN::test_x). The bodies
// turbofish the engine entry points with the enclosing module's `GEN`, and the shadow consts
// below replace the const-fns that used to be plain `pub const`s.
macro_rules! gen_tests {
    ($modname:ident, $gen:literal) => {
        mod $modname {
            use super::*;
            const GEN: u8 = $gen;
            const CRIT_MULTIPLIER: f32 = poke_engine::engine::damage_calc::crit_multiplier::<GEN>();
            const BASE_CRIT_CHANCE: f32 =
                poke_engine::engine::generate_instructions::base_crit_chance::<GEN>();
            const MAX_SLEEP_TURNS: i8 =
                poke_engine::engine::generate_instructions::max_sleep_turns::<GEN>();
            const BURN_RESIDUAL_DAMAGE_PCT: f32 =
                poke_engine::engine::generate_instructions::burn_residual_damage_pct::<GEN>();
            const CONSECUTIVE_PROTECT_CHANCE: f32 =
                poke_engine::engine::generate_instructions::consecutive_protect_chance::<GEN>();
            const WEATHER_ABILITY_TURNS: i8 =
                poke_engine::engine::abilities::weather_ability_turns::<GEN>();
            include!("impls/battle_mechanics_impl.rs");
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
