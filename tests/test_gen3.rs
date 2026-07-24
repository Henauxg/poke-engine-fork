// Generation 3 is always compiled now; the engine is selected at runtime.

use poke_engine::choices::{moves, Choices};
use poke_engine::gen3::abilities::Abilities;
use poke_engine::gen3::generate_instructions::generate_instructions_from_move_pair;
use poke_engine::gen3::items::Items;
use poke_engine::gen3::state::{MoveChoice, PokemonVolatileStatus, Weather};
use poke_engine::instruction::ChangeSideConditionInstruction;
use poke_engine::instruction::{
    ApplyVolatileStatusInstruction, ChangeItemInstruction, ChangeStatusInstruction,
    ChangeVolatileStatusDurationInstruction, DamageInstruction, DecrementFutureAttackInstruction,
    EnableMoveInstruction, FutureAttackKind, HealInstruction, Instruction,
    RemoveVolatileStatusInstruction, SetFutureAttackInstruction, SetSleepTurnsInstruction,
    StateInstructions, SwitchInstruction,
};
use poke_engine::state::PokemonSideCondition;
use poke_engine::state::{
    FutureAttack, Move, PokemonIndex, PokemonMoveIndex, PokemonStatus, PokemonType, SideReference,
    State,
};

pub fn generate_instructions_with_state_assertion(
    state: &mut State,
    side_one_move: &MoveChoice,
    side_two_move: &MoveChoice,
) -> Vec<StateInstructions> {
    let before_state_string = format!("{:?}", state);
    let instructions =
        generate_instructions_from_move_pair(state, side_one_move, side_two_move, false);
    let after_state_string = format!("{:?}", state);
    assert_eq!(before_state_string, after_state_string);
    instructions
}

fn set_moves_on_pkmn_and_call_generate_instructions(
    state: &mut State,
    move_one: Choices,
    move_two: Choices,
) -> Vec<StateInstructions> {
    state
        .side_one
        .get_active()
        .replace_move::<3>(PokemonMoveIndex::M0, move_one);
    state
        .side_two
        .get_active()
        .replace_move::<3>(PokemonMoveIndex::M0, move_two);

    let instructions = generate_instructions_with_state_assertion(
        state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );
    instructions
}

#[test]
fn test_regular_move_with_protect_side_condition() {
    let mut state = State::default();
    state.side_one.side_conditions.protect = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            }),
            Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                side_ref: SideReference::SideOne,
                side_condition: PokemonSideCondition::Protect,
                amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_chestoberry_activates_when_being_put_to_sleep() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::CHESTOBERRY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPORE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ChangeItem(ChangeItemInstruction {
            side_ref: SideReference::SideOne,
            current_item: Items::CHESTOBERRY,
            new_item: Items::NONE,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_chestoberry_activates_when_using_rest() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::CHESTOBERRY;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::REST,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 48,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::NONE,
                new_status: PokemonStatus::SLEEP,
            }),
            Instruction::SetRestTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 3,
                previous_turns: 0,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 48,
            }),
            Instruction::ChangeItem(ChangeItemInstruction {
                side_ref: SideReference::SideOne,
                current_item: Items::CHESTOBERRY,
                new_item: Items::NONE,
            }),
            Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::SLEEP,
                new_status: PokemonStatus::NONE,
            }),
            Instruction::SetRestTurns(SetSleepTurnsInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                new_turns: 0,
                previous_turns: 3,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_taunt_gets_applied_and_duration_increments_end_of_turn() {
    let mut state = State::default();
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TAUNT,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::TAUNT,
            }),
            Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::TAUNT,
                amount: 1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_taunt_volatile_is_removed_end_of_turn_when_it_would_reach_2() {
    let mut state = State::default();
    state.side_one.volatile_status_durations.taunt = 1;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::TAUNT);
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
            }),
            Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
                amount: -1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_taunt_re_enables_disabled_moves_when_being_removed() {
    let mut state = State::default();
    state.side_one.volatile_status_durations.taunt = 1;
    state.side_one.get_active().moves.m1.disabled = true;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::TAUNT);
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
            }),
            Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
                amount: -1,
            }),
            Instruction::EnableMove(EnableMoveInstruction {
                side_ref: SideReference::SideOne,
                move_index: PokemonMoveIndex::M1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_switching_out_with_taunt_resets_duration_to_0() {
    let mut state = State::default();
    state.side_one.volatile_status_durations.taunt = 1;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::TAUNT);
    let vec_of_instructions = generate_instructions_with_state_assertion(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::ChangeVolatileStatusDuration(ChangeVolatileStatusDurationInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
                amount: -1,
            }),
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideOne,
                volatile_status: PokemonVolatileStatus::TAUNT,
            }),
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_taunt_prevents_status_move() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].hp = 0;
    state.side_one.pokemon[PokemonIndex::P2].hp = 0;
    state.side_one.pokemon[PokemonIndex::P3].hp = 0;
    state.side_one.pokemon[PokemonIndex::P4].hp = 0;
    state.side_one.pokemon[PokemonIndex::P5].hp = 0;
    state
        .side_one
        .volatile_statuses
        .insert(PokemonVolatileStatus::TAUNT);

    state.side_one.get_active().moves[&PokemonMoveIndex::M0] = Move {
        id: Choices::TOXIC,
        disabled: false,
        pp: 35,
        choice: moves::<3>().get(&Choices::TOXIC).unwrap().clone(),
    };

    state.side_one.get_active().moves[&PokemonMoveIndex::M1] = Move {
        id: Choices::TACKLE,
        disabled: false,
        pp: 35,
        choice: moves::<3>().get(&Choices::TACKLE).unwrap().clone(),
    };

    state.side_one.get_active().moves[&PokemonMoveIndex::M2] = Move {
        id: Choices::WATERGUN,
        disabled: false,
        pp: 35,
        choice: moves::<3>().get(&Choices::TACKLE).unwrap().clone(),
    };

    state.side_one.get_active().moves[&PokemonMoveIndex::M3] = Move {
        id: Choices::EMBER,
        disabled: false,
        pp: 35,
        choice: moves::<3>().get(&Choices::TACKLE).unwrap().clone(),
    };

    let (side_one_moves, _) = state.gen3_get_all_options();
    assert_eq!(
        vec![
            MoveChoice::Move(PokemonMoveIndex::M1),
            MoveChoice::Move(PokemonMoveIndex::M2),
            MoveChoice::Move(PokemonMoveIndex::M3),
        ],
        side_one_moves
    );
}

#[test]
fn test_rest_does_not_activate_when_fainted() {
    let mut state = State::default();
    state.side_one.get_active().item = Items::CHESTOBERRY;
    state.side_one.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::REST,
        Choices::TACKLE,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_branch_when_a_roll_can_kill() {
    let mut state = State::default();
    state.side_two.get_active().hp = 50;

    let move_one = Choices::TACKLE;
    let move_two = Choices::SPLASH;
    state
        .side_one
        .get_active()
        .replace_move::<3>(PokemonMoveIndex::M0, move_one);
    state
        .side_two
        .get_active()
        .replace_move::<3>(PokemonMoveIndex::M0, move_two);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        true,
    );

    // This damage roll is 44-52, so it can kill
    // Normally without considering the roll, the damage is 48 (0.925 * 52)
    let expected_instructions = vec![
        StateInstructions {
            percentage: 70.3125,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 46,
            })],
        },
        StateInstructions {
            percentage: 29.6875,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 50,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_fast_explosion_makes_other_side_unable_to_move() {
    let mut state = State::default();
    state.side_one.get_active().hp = 500;
    state.side_one.get_active().maxhp = 500;
    state.side_one.get_active().types = (PokemonType::STEEL, PokemonType::FLYING);
    state.side_one.get_active().speed = 45;
    state.side_two.get_active().speed = 50;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPIKES,
        Choices::EXPLOSION,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 100,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 292,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_end_of_turn_sand_kos_before_leftovers() {
    let mut state = State::default();
    state.weather.weather_type = Weather::SAND;
    state.weather.turns_remaining = -1;

    state.side_one.get_active().hp = 5;
    state.side_one.get_active().maxhp = 100;
    state.side_one.get_active().item = Items::LEFTOVERS;

    state.side_two.get_active().hp = 7;
    state.side_two.get_active().maxhp = 100;
    state.side_two.get_active().item = Items::LEFTOVERS;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 6,
            }),
            Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 5,
            }),
            Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideTwo,
                heal_amount: 6,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_intimidate_blocked_by_clearbody() {
    let mut state = State::default();
    state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;
    state.side_two.get_active().ability = Abilities::CLEARBODY;

    let vec_of_instructions = generate_instructions_with_state_assertion(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::Move(PokemonMoveIndex::M0),
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::Switch(SwitchInstruction {
            side_ref: SideReference::SideOne,
            previous_index: PokemonIndex::P0,
            next_index: PokemonIndex::P1,
        })],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_same_speed_branch() {
    let mut state = State::default();
    state.side_one.get_active().speed = 100;
    state.side_one.get_active().hp = 1;
    state.side_two.get_active().speed = 100;
    state.side_two.get_active().hp = 1;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::TACKLE,
        Choices::TACKLE,
    );

    let expected_instructions = vec![
        StateInstructions {
            percentage: 50.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            })],
        },
        StateInstructions {
            percentage: 50.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_gen3_branch_when_a_roll_can_kill() {
    let mut state = State::default();
    state.side_two.get_active().hp = 50;

    let move_one = Choices::TACKLE;
    let move_two = Choices::SPLASH;
    state
        .side_one
        .get_active()
        .replace_move::<3>(PokemonMoveIndex::M0, move_one);
    state
        .side_two
        .get_active()
        .replace_move::<3>(PokemonMoveIndex::M0, move_two);

    let vec_of_instructions = generate_instructions_from_move_pair(
        &mut state,
        &MoveChoice::Move(PokemonMoveIndex::M0),
        &MoveChoice::Move(PokemonMoveIndex::M0),
        true,
    );

    // This damage roll is 44-52, so it can kill
    // Normally without considering the roll, the damage is 48 (0.925 * 52)
    // The roll itself has a 25% chance of killing but the extra chance is accounting for a crit
    let expected_instructions = vec![
        StateInstructions {
            percentage: 70.3125,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 46,
            })],
        },
        StateInstructions {
            percentage: 29.6875,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 50,
            })],
        },
    ];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_consecutive_protect_success_is_floored_at_one_eighth() {
    // 5th consecutive Protect (stack 4). Unclamped (1/2)^4 = 1/16 = 6.25%; the real gen-3
    // floor is 1/8 = 12.5% (4th use onward). Pins the literal floored value so it is not
    // self-referential with CONSECUTIVE_PROTECT_CHANCE.
    let mut state = State::default();
    state.side_one.side_conditions.protect = 4;
    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::PROTECT,
        Choices::TACKLE,
    );
    let success = vec_of_instructions
        .iter()
        .find(|si| {
            si.instruction_list.iter().any(|i| {
                matches!(
                    i,
                    Instruction::ApplyVolatileStatus(a)
                        if a.volatile_status == PokemonVolatileStatus::PROTECT
                )
            })
        })
        .expect("a branch in which Protect succeeds");
    assert!(
        (success.percentage - 100.0 / 8.0).abs() < 1e-4,
        "expected the gen-3 success chance to be floored at 1/8, got {}%",
        success.percentage
    );
}

// Status-cure berries in gen 3 fire at END OF TURN (not before-move, unlike gens 4+). For
// statuses that cannot self-resolve mid-turn (burn/poison/toxic/paralysis) every branch
// therefore ends with the status cured and the berry consumed. (Freeze/sleep can resolve
// on their own before end of turn, in which case the berry is correctly NOT consumed, so
// they are not covered by this simple assertion.)
fn assert_gen3_status_cure_berry(berry: Items, status: PokemonStatus) {
    let mut state = State::default();
    state.side_one.get_active().item = berry;
    state.side_one.get_active().status = status;

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SPLASH,
        Choices::SPLASH,
    );

    assert!(!vec_of_instructions.is_empty());
    for si in &vec_of_instructions {
        let mut branch = state.clone();
        branch.apply_instructions(&si.instruction_list);
        let active = branch.side_one.get_active_immutable();
        assert_eq!(
            PokemonStatus::NONE,
            active.status,
            "{:?} should have cured {:?} by end of turn",
            berry,
            status
        );
        assert_eq!(
            Items::NONE,
            active.item,
            "{:?} should have been consumed",
            berry
        );
    }
}

#[test]
fn test_rawstberry_cures_burn() {
    assert_gen3_status_cure_berry(Items::RAWSTBERRY, PokemonStatus::BURN);
}

#[test]
fn test_cheriberry_cures_paralysis() {
    assert_gen3_status_cure_berry(Items::CHERIBERRY, PokemonStatus::PARALYZE);
}

#[test]
fn test_pechaberry_cures_poison() {
    assert_gen3_status_cure_berry(Items::PECHABERRY, PokemonStatus::POISON);
}

#[test]
fn test_pechaberry_cures_toxic() {
    assert_gen3_status_cure_berry(Items::PECHABERRY, PokemonStatus::TOXIC);
}

#[test]
fn test_dreameater_does_nothing_to_an_awake_target() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DREAMEATER,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_snore_does_nothing_while_the_user_is_awake() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::SNORE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_facade_doubling_condition_and_burn_halving() {
    let facade_damage = |status: PokemonStatus| -> i16 {
        let mut state = State::default();
        state.side_one.get_active().status = status;
        // Damage is clamped to the target's remaining HP, and a default Pokemon has 100 -- at
        // which a doubled Facade and an undoubled one both read as 100.
        state.side_two.get_active().hp = 10000;
        state.side_two.get_active().maxhp = 10000;
        set_moves_on_pkmn_and_call_generate_instructions(
            &mut state,
            Choices::FACADE,
            Choices::SPLASH,
        )
        .iter()
        .flat_map(|branch| branch.instruction_list.iter())
        .filter_map(|i| match i {
            Instruction::Damage(d) if d.side_ref == SideReference::SideTwo => Some(d.damage_amount),
            _ => None,
        })
        .sum()
    };

    let base = facade_damage(PokemonStatus::NONE);
    assert!(base > 0, "sanity: an unstatused facade must deal damage");

    // Integer truncation in the damage formula means doubling the BASE POWER does not double the
    // final number exactly, hence the tolerance.
    let poisoned = facade_damage(PokemonStatus::POISON);
    assert!(
        (poisoned - base * 2).abs() <= 2,
        "poison should double facade: base {}, poisoned {}",
        base,
        poisoned
    );

    let burned = facade_damage(PokemonStatus::BURN);
    assert!(
        (burned - poisoned / 2).abs() <= 2,
        "gen 3: burn still halves facade: poisoned {}, burned {}",
        poisoned,
        burned
    );
}

#[test]
fn test_meanlook_applies_trapped() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::MEANLOOK,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::ApplyVolatileStatus(
            ApplyVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::TRAPPED,
            },
        )],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_meanlook_traps_opponent_from_switching() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::TRAPPED);

    let (_side_one_moves, side_two_moves) = state.gen3_get_all_options();

    assert_eq!(
        vec![
            MoveChoice::Move(PokemonMoveIndex::M0),
            MoveChoice::Move(PokemonMoveIndex::M1),
            MoveChoice::Move(PokemonMoveIndex::M2),
            MoveChoice::Move(PokemonMoveIndex::M3),
        ],
        side_two_moves
    );
}

#[test]
fn test_switching_out_clears_opponent_trapped() {
    let mut state = State::default();
    state
        .side_two
        .volatile_statuses
        .insert(PokemonVolatileStatus::TRAPPED);

    let vec_of_instructions = generate_instructions_with_state_assertion(
        &mut state,
        &MoveChoice::Switch(PokemonIndex::P1),
        &MoveChoice::None,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                side_ref: SideReference::SideTwo,
                volatile_status: PokemonVolatileStatus::TRAPPED,
            }),
            Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_using_doomdesire_does_not_damage_immediately() {
    let mut state = State::default();

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DOOMDESIRE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![
            Instruction::SetFutureAttack(SetFutureAttackInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                previous_pokemon_index: PokemonIndex::P0,
                move_id: FutureAttackKind::DoomDesire,
                previous_move_id: FutureAttackKind::None,
            }),
            Instruction::DecrementFutureAttack(DecrementFutureAttackInstruction {
                side_ref: SideReference::SideOne,
            }),
        ],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}

#[test]
fn test_doomdesire_activating_uses_steel() {
    let mut state = State::default();
    state.side_one.future_attack = FutureAttack {
        turns_remaining: 1,
        pokemon_index: PokemonIndex::P0,
        move_id: Choices::DOOMDESIRE,
    };

    let doomdesire_damage = {
        let instructions = set_moves_on_pkmn_and_call_generate_instructions(
            &mut state,
            Choices::SPLASH,
            Choices::SPLASH,
        );
        instructions
            .iter()
            .flat_map(|branch| branch.instruction_list.iter())
            .find_map(|i| match i {
                Instruction::Damage(d) if d.side_ref == SideReference::SideTwo => {
                    Some(d.damage_amount)
                }
                _ => None,
            })
            .expect("doom desire should deal damage on landing")
    };

    let mut state = State::default();
    state.side_one.future_attack = FutureAttack {
        turns_remaining: 1,
        pokemon_index: PokemonIndex::P0,
        move_id: Choices::FUTURESIGHT,
    };
    let futuresight_damage = {
        let instructions = set_moves_on_pkmn_and_call_generate_instructions(
            &mut state,
            Choices::SPLASH,
            Choices::SPLASH,
        );
        instructions
            .iter()
            .flat_map(|branch| branch.instruction_list.iter())
            .find_map(|i| match i {
                Instruction::Damage(d) if d.side_ref == SideReference::SideTwo => {
                    Some(d.damage_amount)
                }
                _ => None,
            })
            .expect("future sight should deal damage on landing")
    };

    // Gen3: Doom Desire is Steel BP 120; Future Sight is Psychic BP 80.
    assert!(
        doomdesire_damage > futuresight_damage,
        "doom desire (bp 120) should outdamage future sight (bp 80): {} vs {}",
        doomdesire_damage,
        futuresight_damage
    );

    let mut state = State::default();
    state.side_one.future_attack = FutureAttack {
        turns_remaining: 1,
        pokemon_index: PokemonIndex::P0,
        move_id: Choices::DOOMDESIRE,
    };
    state.side_two.get_active().types = (PokemonType::FIRE, PokemonType::TYPELESS);
    let steel_into_fire = {
        let instructions = set_moves_on_pkmn_and_call_generate_instructions(
            &mut state,
            Choices::SPLASH,
            Choices::SPLASH,
        );
        instructions
            .iter()
            .flat_map(|branch| branch.instruction_list.iter())
            .find_map(|i| match i {
                Instruction::Damage(d) if d.side_ref == SideReference::SideTwo => {
                    Some(d.damage_amount)
                }
                _ => None,
            })
            .expect("doom desire should deal damage on landing")
    };

    assert!(
        steel_into_fire < doomdesire_damage,
        "steel into fire should deal less than steel into normal: {} vs {}",
        steel_into_fire,
        doomdesire_damage
    );
}

#[test]
fn test_cannot_stack_doomdesire_with_futuresight() {
    let mut state = State::default();
    state.side_one.future_attack = FutureAttack {
        turns_remaining: 2,
        pokemon_index: PokemonIndex::P0,
        move_id: Choices::FUTURESIGHT,
    };

    let vec_of_instructions = set_moves_on_pkmn_and_call_generate_instructions(
        &mut state,
        Choices::DOOMDESIRE,
        Choices::SPLASH,
    );

    let expected_instructions = vec![StateInstructions {
        percentage: 100.0,
        instruction_list: vec![Instruction::DecrementFutureAttack(
            DecrementFutureAttackInstruction {
                side_ref: SideReference::SideOne,
            },
        )],
    }];
    assert_eq!(expected_instructions, vec_of_instructions);
}
