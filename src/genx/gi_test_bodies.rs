    #[test]
    fn test_drag_move_as_second_move_exits_early_if_opponent_used_drag_move() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::DRAGONTAIL).unwrap().to_owned();
        choice.first_move = false;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::DRAGONTAIL).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_electric_move_does_nothing_versus_ground_type() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::THUNDERBOLT).unwrap().to_owned();
        state.side_two.get_active().types = (PokemonType::GROUND, PokemonType::TYPELESS);
        choice.first_move = false;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_grass_type_cannot_have_powder_move_used_against_it() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::SPORE).unwrap().to_owned(); // Spore is a powder move
        state.side_two.get_active().types = (PokemonType::GRASS, PokemonType::TYPELESS);
        choice.first_move = false;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = if GEN == 4 || GEN == 5 {
            vec![StateInstructions {
                percentage: 100.0,
                instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::NONE,
                    new_status: PokemonStatus::SLEEP,
                })],
            }]
        } else {
            vec![StateInstructions::default()]
        };

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_spikes_sets_first_layer() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::SPIKES).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeSideCondition(
                ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Spikes,
                    amount: 1,
                },
            )],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_spikes_layers_cannot_exceed_3() {
        let mut state: State = State::default();
        state.side_two.side_conditions.spikes = 3;
        let mut choice = moves::<GEN>().get(&Choices::SPIKES).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_aurora_veil_works_in_hail() {
        let mut state: State = State::default();
        state.weather.weather_type = Weather::HAIL;
        let mut choice = moves::<GEN>().get(&Choices::AURORAVEIL).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeSideCondition(
                ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::AuroraVeil,
                    amount: 5,
                },
            )],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_auroa_veil_fails_outside_hail() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::AURORAVEIL).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_auroa_veil_fails_outside_of_hail() {
        let mut state: State = State::default();
        state.weather.weather_type = Weather::NONE;
        let mut choice = moves::<GEN>().get(&Choices::AURORAVEIL).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_stealthrock_cannot_exceed_1_layer() {
        let mut state: State = State::default();
        state.side_two.side_conditions.stealth_rock = 1;
        let mut choice = moves::<GEN>().get(&Choices::STEALTHROCK).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_stoneaxe_damage_and_stealthrock_setting() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::STONEAXE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 51,
                    }),
                    Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                        side_ref: SideReference::SideTwo,
                        side_condition: PokemonSideCondition::Stealthrock,
                        amount: 1,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_ceaselessedge_damage_and_stealthrock_setting() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::CEASELESSEDGE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 51,
                    }),
                    Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                        side_ref: SideReference::SideTwo,
                        side_condition: PokemonSideCondition::Spikes,
                        amount: 1,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_100_percent_secondary_volatilestatus() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::CHATTER).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 51,
                }),
                Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_possible_secondary_volatilestatus() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::CONFUSION).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 40,
                })],
            },
            StateInstructions {
                percentage: 10.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 40,
                    }),
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::CONFUSION,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_possible_secondary_volatilestatus_with_possible_accuracy() {
        let mut state: State = State::default();
        state.side_two.get_active().hp = 400;
        state.side_two.get_active().maxhp = 400;
        let mut choice = moves::<GEN>().get(&Choices::AXEKICK).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 50, // This move has recoil lol
                })],
            },
            StateInstructions {
                percentage: 63.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 188,
                })],
            },
            StateInstructions {
                percentage: 27.0000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 188,
                    }),
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::CONFUSION,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_volatile_status_applied_to_self() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::AQUARING).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ApplyVolatileStatus(
                ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::AQUARING,
                },
            )],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_volatile_status_applied_to_opponent() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::ATTRACT).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ApplyVolatileStatus(
                ApplyVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::ATTRACT,
                },
            )],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_cannot_apply_volatile_status_twice() {
        let mut state: State = State::default();
        state
            .side_two
            .volatile_statuses
            .insert(PokemonVolatileStatus::ATTRACT);
        let mut choice = moves::<GEN>().get(&Choices::ATTRACT).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_substitute_failing_if_user_has_less_than_25_percent_hp() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 25;
        let mut choice = moves::<GEN>().get(&Choices::SUBSTITUTE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_shedtail_failing_if_user_has_less_than_50_percent_hp() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 50;
        let mut choice = moves::<GEN>().get(&Choices::SHEDTAIL).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_drag_move() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::WHIRLWIND).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P2,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P3,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P4,
                })],
            },
            StateInstructions {
                percentage: 20.0,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P5,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_drag_move_with_fainted_reserve() {
        let mut state: State = State::default();
        state.side_two.pokemon[PokemonIndex::P1].hp = 0;
        state.side_two.pokemon[PokemonIndex::P3].hp = 0;
        let mut choice = moves::<GEN>().get(&Choices::WHIRLWIND).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 33.333336,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P2,
                })],
            },
            StateInstructions {
                percentage: 33.333336,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P4,
                })],
            },
            StateInstructions {
                percentage: 33.333336,
                instruction_list: vec![Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideTwo,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P5,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_damaging_drag_move_with_fainted_reserve() {
        let mut state: State = State::default();
        state.side_two.pokemon[PokemonIndex::P1].hp = 0;
        state.side_two.pokemon[PokemonIndex::P3].hp = 0;
        let mut choice = moves::<GEN>().get(&Choices::DRAGONTAIL).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.0000019,
                instruction_list: vec![], // The move missed
            },
            StateInstructions {
                percentage: 30.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P2,
                    }),
                ],
            },
            StateInstructions {
                percentage: 30.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P4,
                    }),
                ],
            },
            StateInstructions {
                percentage: 30.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P5,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_damaging_drag_that_knocks_out_defender() {
        let mut state: State = State::default();
        state.side_two.pokemon[PokemonIndex::P1].hp = 0;
        state.side_two.pokemon[PokemonIndex::P3].hp = 0;
        state.side_two.get_active().hp = 5;
        let mut choice = moves::<GEN>().get(&Choices::DRAGONTAIL).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.0000019,
                instruction_list: vec![], // The move missed
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 5,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_drag_versus_no_alive_reserved() {
        let mut state: State = State::default();
        state.side_two.pokemon[PokemonIndex::P1].hp = 0;
        state.side_two.pokemon[PokemonIndex::P2].hp = 0;
        state.side_two.pokemon[PokemonIndex::P3].hp = 0;
        state.side_two.pokemon[PokemonIndex::P4].hp = 0;
        state.side_two.pokemon[PokemonIndex::P5].hp = 0;
        let mut choice = moves::<GEN>().get(&Choices::WHIRLWIND).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_drag_move_with_fainted_reserve_and_prior_instruction() {
        let mut state: State = State::default();
        state.side_two.pokemon[PokemonIndex::P1].hp = 0;
        state.side_two.pokemon[PokemonIndex::P3].hp = 0;
        let mut choice = moves::<GEN>().get(&Choices::WHIRLWIND).unwrap().to_owned();

        let previous_instruction = StateInstructions {
            percentage: 50.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 5,
            })],
        };

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            previous_instruction,
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 16.666668,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideOne,
                        damage_amount: 5,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P2,
                    }),
                ],
            },
            StateInstructions {
                percentage: 16.666668,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideOne,
                        damage_amount: 5,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P4,
                    }),
                ],
            },
            StateInstructions {
                percentage: 16.666668,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideOne,
                        damage_amount: 5,
                    }),
                    Instruction::Switch(SwitchInstruction {
                        side_ref: SideReference::SideTwo,
                        previous_index: PokemonIndex::P0,
                        next_index: PokemonIndex::P5,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_status_move() {
        if GEN != 9 {
            return;
        }
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::GLARE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideTwo,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::NONE,
                new_status: PokemonStatus::PARALYZE,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_status_move_that_can_miss() {
        if GEN != 9 {
            return;
        }
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::THUNDERWAVE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::NONE,
                    new_status: PokemonStatus::PARALYZE,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_status_move_that_can_miss_but_is_blocked_by_ability() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::LIMBER;
        let mut choice = moves::<GEN>().get(&Choices::THUNDERWAVE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody_conditional_burn_on_contact() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        let mut choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 70.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                })],
            },
            StateInstructions {
                percentage: 30.0000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideOne,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_protectivepads_stops_flamebody() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        state.side_one.get_active().item = Items::PROTECTIVEPADS;
        let mut choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 48,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody_versus_noncontact_move() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        let mut choice = moves::<GEN>().get(&Choices::WATERGUN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 32,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody_versus_fire_type() {
        let mut state: State = State::default();
        state.side_one.get_active().types.0 = PokemonType::FIRE;
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        let mut choice = moves::<GEN>().get(&Choices::WATERGUN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 32,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_move_with_multiple_secondaries() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::FIREFANG).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 5.00000095,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 76.9499969,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 51,
                })],
            },
            StateInstructions {
                percentage: 8.55000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 51,
                    }),
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::FLINCH,
                    }),
                ],
            },
            StateInstructions {
                percentage: 8.55000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 51,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
            StateInstructions {
                percentage: 0.949999988,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 51,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                    Instruction::ApplyVolatileStatus(ApplyVolatileStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        volatile_status: PokemonVolatileStatus::FLINCH,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        let mut choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 70.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 48,
                })],
            },
            StateInstructions {
                percentage: 30.000002,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 48,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideOne,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flamebody_creating_a_move_with_multiple_secondaries() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::FLAMEBODY;
        let mut choice = moves::<GEN>().get(&Choices::FIREPUNCH).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 63.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 60,
                })],
            },
            StateInstructions {
                percentage: 27.0000019,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 60,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideOne,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
            StateInstructions {
                percentage: 7.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 60,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
            StateInstructions {
                percentage: 3.0,
                instruction_list: vec![
                    Instruction::Damage(DamageInstruction {
                        side_ref: SideReference::SideTwo,
                        damage_amount: 60,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideTwo,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                    Instruction::ChangeStatus(ChangeStatusInstruction {
                        side_ref: SideReference::SideOne,
                        pokemon_index: PokemonIndex::P0,
                        old_status: PokemonStatus::NONE,
                        new_status: PokemonStatus::BURN,
                    }),
                ],
            },
        ];
        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_substitute_does_not_block_rest() {
        let mut state: State = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::SUBSTITUTE);
        state.side_one.get_active().hp = state.side_one.get_active().maxhp - 1;
        let mut choice = moves::<GEN>().get(&Choices::REST).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
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
                    heal_amount: 1,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_heal_move() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 1;
        let mut choice = moves::<GEN>().get(&Choices::RECOVER).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 50,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_heal_move_generates_no_instruction_at_maxhp() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::RECOVER).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_negative_heal_move() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::EXPLOSION).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 100,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_negative_heal_move_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 1;
        let mut choice = moves::<GEN>().get(&Choices::EXPLOSION).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_heal_move_does_not_overheal() {
        let mut state: State = State::default();
        state.side_one.get_active().hp = 55;
        let mut choice = moves::<GEN>().get(&Choices::RECOVER).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 45,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_boosting_move() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::SWORDSDANCE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 2,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_does_not_overboost() {
        let mut state: State = State::default();
        state.side_one.attack_boost = 5;
        let mut choice = moves::<GEN>().get(&Choices::SWORDSDANCE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Attack,
                amount: 1,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_no_instruction_when_boosting_at_max() {
        let mut state: State = State::default();
        state.side_one.attack_boost = 6;
        let mut choice = moves::<GEN>().get(&Choices::SWORDSDANCE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_boost_lowering_that_can_miss() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::KINESIS).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 19.999998,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 80.0,
                instruction_list: vec![Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideTwo,
                    stat: PokemonBoostableStat::Accuracy,
                    amount: -1,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_basic_boost_lowering() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::CHARM).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: -2,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_cannot_boost_lower_than_negative_6() {
        let mut state: State = State::default();
        state.side_two.attack_boost = -5;
        let mut choice = moves::<GEN>().get(&Choices::CHARM).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideTwo,
                stat: PokemonBoostableStat::Attack,
                amount: -1,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_no_boost_when_already_at_minimum() {
        let mut state: State = State::default();
        state.side_two.attack_boost = -6;
        let mut choice = moves::<GEN>().get(&Choices::CHARM).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_clearbody_blocks_stat_lowering() {
        let mut state: State = State::default();
        state.side_two.get_active().ability = Abilities::CLEARBODY;
        let mut choice = moves::<GEN>().get(&Choices::CHARM).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_clearbody_does_not_block_self_stat_lowering() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::CLEARBODY;
        let mut choice = moves::<GEN>().get(&Choices::SHELLSMASH).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Attack,
                    amount: 2,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Defense,
                    amount: -1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::SpecialAttack,
                    amount: 2,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::SpecialDefense,
                    amount: -1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: 2,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_defog_does_not_change_terrain_if_terrain_is_none() {
        let mut state: State = State::default();

        let mut choice = moves::<GEN>().get(&Choices::DEFOG).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_defog_clears_terrain() {
        let mut state: State = State::default();
        state.terrain.terrain_type = Terrain::ELECTRICTERRAIN;
        state.terrain.turns_remaining = 1;

        let mut choice = moves::<GEN>().get(&Choices::DEFOG).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeTerrain(ChangeTerrain {
                new_terrain: Terrain::NONE,
                new_terrain_turns_remaining: 0,
                previous_terrain: Terrain::ELECTRICTERRAIN,
                previous_terrain_turns_remaining: 1,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_defog_clears_terrain_and_side_conditions() {
        let mut state: State = State::default();
        state.terrain.terrain_type = Terrain::ELECTRICTERRAIN;
        state.terrain.turns_remaining = 1;
        state.side_one.side_conditions.reflect = 1;
        state.side_two.side_conditions.reflect = 1;

        let mut choice = moves::<GEN>().get(&Choices::DEFOG).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeTerrain(ChangeTerrain {
                    new_terrain: Terrain::NONE,
                    new_terrain_turns_remaining: 0,
                    previous_terrain: Terrain::ELECTRICTERRAIN,
                    previous_terrain_turns_remaining: 1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Reflect,
                    amount: -1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Reflect,
                    amount: -1,
                }),
            ],
        }];
        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_tidyup_clears_side_conditions_and_substitutes() {
        let mut state: State = State::default();
        state.terrain.terrain_type = Terrain::ELECTRICTERRAIN;
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::SUBSTITUTE);
        state
            .side_two
            .volatile_statuses
            .insert(PokemonVolatileStatus::SUBSTITUTE);
        state.side_one.substitute_health = 10;
        state.side_two.substitute_health = 25;
        state.terrain.turns_remaining = 1;
        state.side_one.side_conditions.spikes = 2;
        state.side_two.side_conditions.stealth_rock = 1;

        let mut choice = moves::<GEN>().get(&Choices::TIDYUP).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Spikes,
                    amount: -2,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: -1,
                }),
                Instruction::ChangeSubstituteHealth(ChangeSubsituteHealthInstruction {
                    side_ref: SideReference::SideOne,
                    health_change: -10,
                }),
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::SUBSTITUTE,
                }),
                Instruction::ChangeSubstituteHealth(ChangeSubsituteHealthInstruction {
                    side_ref: SideReference::SideTwo,
                    health_change: -25,
                }),
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideTwo,
                    volatile_status: PokemonVolatileStatus::SUBSTITUTE,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Attack,
                    amount: 1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: 1,
                }),
            ],
        }];
        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_rapidspin_clears_hazards() {
        if !(GEN == 8 || GEN == 9) {
            return;
        }
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;

        let mut choice = moves::<GEN>().get(&Choices::RAPIDSPIN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 61,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: -1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    amount: 1,
                    stat: PokemonBoostableStat::Speed,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_missing_rapidspin_does_not_clear_hazards() {
        let mut state: State = State::default();
        state.side_two.get_active().types = (PokemonType::GHOST, PokemonType::NORMAL);
        state.side_one.side_conditions.stealth_rock = 1;

        let mut choice = moves::<GEN>().get(&Choices::RAPIDSPIN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];
        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_acid_into_steel_type() {
        let mut state: State = State::default();
        state.side_two.get_active().types = (PokemonType::STEEL, PokemonType::NORMAL);

        let mut choice = moves::<GEN>().get(&Choices::ACID).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        }];
        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_rapidspin_clears_multiple_hazards() {
        if !(GEN == 8 || GEN == 9) {
            return;
        }
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.side_conditions.toxic_spikes = 2;
        state.side_one.side_conditions.spikes = 3;
        state.side_one.side_conditions.sticky_web = 1;

        let mut choice = moves::<GEN>().get(&Choices::RAPIDSPIN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 61,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: -1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Spikes,
                    amount: -3,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicSpikes,
                    amount: -2,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::StickyWeb,
                    amount: -1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    amount: 1,
                    stat: PokemonBoostableStat::Speed,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_rapidspin_does_not_clear_opponent_hazards() {
        if !(GEN == 8 || GEN == 9) {
            return;
        }
        let mut state: State = State::default();
        state.side_two.side_conditions.stealth_rock = 1;
        state.side_two.side_conditions.toxic_spikes = 2;
        state.side_two.side_conditions.spikes = 3;
        state.side_two.side_conditions.sticky_web = 1;

        let mut choice = moves::<GEN>().get(&Choices::RAPIDSPIN).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 61,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    amount: 1,
                    stat: PokemonBoostableStat::Speed,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_courtchange_basic_swap() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;

        let mut choice = moves::<GEN>().get(&Choices::COURTCHANGE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: -1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: 1,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_courtchange_complicated_swap() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_two.side_conditions.toxic_spikes = 2;
        state.side_two.side_conditions.spikes = 3;
        state.side_two.side_conditions.sticky_web = 1;

        let mut choice = moves::<GEN>().get(&Choices::COURTCHANGE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: -1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Stealthrock,
                    amount: 1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::Spikes,
                    amount: -3,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Spikes,
                    amount: 3,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::ToxicSpikes,
                    amount: -2,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicSpikes,
                    amount: 2,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideTwo,
                    side_condition: PokemonSideCondition::StickyWeb,
                    amount: -1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::StickyWeb,
                    amount: 1,
                }),
            ],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_stoneaxe_does_not_set_stealthrock_if_already_set() {
        let mut state: State = State::default();
        state.side_two.side_conditions.stealth_rock = 1;
        let mut choice = moves::<GEN>().get(&Choices::STONEAXE).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions = vec![
            StateInstructions {
                percentage: 10.000002,
                instruction_list: vec![],
            },
            StateInstructions {
                percentage: 90.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 51,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_flinched_pokemon_cannot_move() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::FLINCH);

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_dead_pokemon_moving_second_does_nothing() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        choice.first_move = false;
        state.side_one.get_active().hp = 0;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );
        assert_eq!(instructions, vec![StateInstructions::default()])
    }

    #[test]
    fn test_cannot_ohko_versus_study() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::EARTHQUAKE).unwrap().to_owned();
        state.side_two.get_active().ability = Abilities::STURDY;
        state.side_two.get_active().hp = 50;
        state.side_two.get_active().maxhp = 50;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 49,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_cannot_ohko_versus_sash() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::EARTHQUAKE).unwrap().to_owned();
        state.side_two.get_active().item = Items::FOCUSSASH;
        state.side_two.get_active().hp = 50;
        state.side_two.get_active().maxhp = 50;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 49,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_sturdy_does_not_affect_non_ohko_move() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::EARTHQUAKE).unwrap().to_owned();
        state.side_two.get_active().ability = Abilities::STURDY;
        state.side_two.get_active().hp = 45;
        state.side_two.get_active().maxhp = 50;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 45,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_beastboost_boosts_on_kill() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().ability = Abilities::BEASTBOOST;
        state.side_one.get_active().attack = 500; // highest stat
        state.side_two.get_active().hp = 1;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Attack,
                    amount: 1,
                }),
            ],
        };
        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_beastboost_boosts_different_stat_on_kill() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().ability = Abilities::BEASTBOOST;
        state.side_one.get_active().defense = 500; // highest stat
        state.side_two.get_active().hp = 1;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Defense,
                    amount: 1,
                }),
            ],
        };
        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_beastboost_does_not_overboost() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().ability = Abilities::BEASTBOOST;
        state.side_one.get_active().attack = 500; // highest stat
        state.side_one.attack_boost = 6; // max boosts already
        state.side_two.get_active().hp = 1;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 1,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_beastboost_does_not_boost_without_kill() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().ability = Abilities::BEASTBOOST;
        state.side_one.get_active().attack = 150; // highest stat
        state.side_two.get_active().hp = 100;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 72,
            })],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_drain_move_heals() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::ABSORB).unwrap().to_owned();
        state.side_one.get_active().hp = 100;
        state.side_one.get_active().maxhp = 200;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 16,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 8,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_drain_move_does_not_overheal() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::ABSORB).unwrap().to_owned();
        state.side_one.get_active().hp = 100;
        state.side_one.get_active().maxhp = 105;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 16,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 5,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_recoil_damage() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::BRAVEBIRD).unwrap().to_owned();
        state.side_one.get_active().hp = 105;
        state.side_one.get_active().maxhp = 105;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 94,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 31,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_recoil_cannot_overkill() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::BRAVEBIRD).unwrap().to_owned();
        state.side_one.get_active().hp = 5;
        state.side_one.get_active().maxhp = 105;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 94,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_drain_and_recoil_together() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::ABSORB).unwrap().to_owned();
        choice.recoil = Some(0.33);
        state.side_one.get_active().hp = 1;
        state.side_one.get_active().maxhp = 105;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 16,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 8,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_crash_move_missing() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::JUMPKICK).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: Vec<StateInstructions> = vec![
            StateInstructions {
                percentage: 5.000001,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 50,
                })],
            },
            StateInstructions {
                percentage: 95.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_crash_move_missing_versus_ghost_type() {
        let mut state: State = State::default();
        state.side_two.get_active().types.0 = PokemonType::GHOST;
        let mut choice = moves::<GEN>().get(&Choices::JUMPKICK).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: Vec<StateInstructions> = vec![StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 50,
            })],
        }];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_crash_move_missing_cannot_overkill() {
        let mut state: State = State::default();
        state.get_side(&SideReference::SideOne).get_active().hp = 5;
        let mut choice = moves::<GEN>().get(&Choices::JUMPKICK).unwrap().to_owned();

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: Vec<StateInstructions> = vec![
            StateInstructions {
                percentage: 5.000001,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                })],
            },
            StateInstructions {
                percentage: 95.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions)
    }

    #[test]
    fn test_knockoff_removing_item() {
        if GEN != 9 {
            return;
        }
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::KNOCKOFF).unwrap().to_owned();
        state.get_side(&SideReference::SideTwo).get_active().item = Items::HEAVYDUTYBOOTS;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 76,
                }),
                Instruction::ChangeItem(ChangeItemInstruction {
                    side_ref: SideReference::SideTwo,
                    current_item: Items::HEAVYDUTYBOOTS,
                    new_item: Items::NONE,
                }),
            ],
        };

        assert_eq!(instructions, vec![expected_instructions])
    }

    #[test]
    fn test_blunderpolicy_boost() {
        let mut state: State = State::default();
        let mut choice = moves::<GEN>().get(&Choices::CROSSCHOP).unwrap().to_owned();
        state.get_side(&SideReference::SideOne).get_active().item = Items::BLUNDERPOLICY;

        let mut instructions = vec![];
        generate_instructions_from_move::<GEN>(
            &mut state,
            &mut choice,
            &moves::<GEN>().get(&Choices::TACKLE).unwrap(),
            SideReference::SideOne,
            StateInstructions::default(),
            &mut instructions,
            false,
        );

        let expected_instructions: Vec<StateInstructions> = vec![
            StateInstructions {
                percentage: 19.999998,
                instruction_list: vec![
                    Instruction::Boost(BoostInstruction {
                        side_ref: SideReference::SideOne,
                        stat: PokemonBoostableStat::Speed,
                        amount: 2,
                    }),
                    Instruction::ChangeItem(ChangeItemInstruction {
                        side_ref: SideReference::SideOne,
                        current_item: Items::BLUNDERPOLICY,
                        new_item: Items::NONE,
                    }),
                ],
            },
            StateInstructions {
                percentage: 80.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 100,
                })],
            },
        ];

        assert_eq!(instructions, expected_instructions);
    }

    #[test]
    fn test_basic_switch_functionality_with_no_prior_instructions() {
        let mut state: State = State::default();
        let mut choice = Choice {
            ..Default::default()
        };

        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_volatile_statuses() {
        let mut state: State = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::LEECHSEED);
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::LEECHSEED,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_toxic_count() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_count = 2;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicCount,
                    amount: -2,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_boost() {
        let mut state: State = State::default();
        state.side_one.attack_boost = 2;
        state.side_one.speed_boost = 5;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Attack,
                    amount: -2,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: -5,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_disabled_move() {
        let mut state: State = State::default();
        state.side_one.get_active().moves.m0 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m1 = Move {
            id: Choices::NONE,
            disabled: false,
            pp: 32,
            ..Default::default()
        };

        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M0,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_with_multiple_disabled_moves() {
        let mut state: State = State::default();
        state.side_one.get_active().moves.m0 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m1 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m2 = Move {
            id: Choices::NONE,
            disabled: false,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m3 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M0,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M1,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M3,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_basic_switch_functionality_with_a_prior_instruction() {
        let mut state: State = State::default();
        let mut incoming_instructions = StateInstructions::default();
        let mut choice = Choice {
            ..Default::default()
        };

        choice.switch_id = PokemonIndex::P1;
        incoming_instructions
            .instruction_list
            .push(Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            }));

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 1,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switch_with_regenerator() {
        let mut state: State = State::default();
        state.side_one.get_active().hp -= 10;
        state.side_one.get_active().ability = Abilities::REGENERATOR;
        state.side_one.get_active().base_ability = Abilities::REGENERATOR;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 10,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switch_with_regenerator_plus_move_enabling() {
        let mut state: State = State::default();
        state.side_one.get_active().moves.m0 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m1 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m2 = Move {
            id: Choices::NONE,
            disabled: false,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().moves.m3 = Move {
            id: Choices::NONE,
            disabled: true,
            pp: 32,
            ..Default::default()
        };
        state.side_one.get_active().hp -= 10;
        state.side_one.get_active().ability = Abilities::REGENERATOR;
        state.side_one.get_active().base_ability = Abilities::REGENERATOR;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M0,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M1,
                }),
                Instruction::EnableMove(EnableMoveInstruction {
                    side_ref: SideReference::SideOne,
                    move_index: PokemonMoveIndex::M3,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 10,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switch_with_regenerator_but_no_damage_taken() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::REGENERATOR;
        state.side_one.get_active().base_ability = Abilities::REGENERATOR;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_fainted_pokemon_with_regenerator_does_not_heal() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::REGENERATOR;
        state.side_one.get_active().base_ability = Abilities::REGENERATOR;
        state.side_one.get_active().hp = 0;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_regenerator_only_heals_one_third() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::REGENERATOR;
        state.side_one.get_active().base_ability = Abilities::REGENERATOR;
        state.side_one.get_active().hp = 3;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 33,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_naturalcure() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::NATURALCURE;
        state.side_one.get_active().base_ability = Abilities::NATURALCURE;
        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    old_status: PokemonStatus::PARALYZE,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_naturalcure_with_no_status() {
        let mut state: State = State::default();
        state.side_one.get_active().ability = Abilities::NATURALCURE;
        state.side_one.get_active().base_ability = Abilities::NATURALCURE;
        state.side_one.get_active().status = PokemonStatus::NONE;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: state.side_one.get_active().hp / 8,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_resisted_stealthrock() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.pokemon[PokemonIndex::P1].types = (PokemonType::GROUND, PokemonType::NORMAL);
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: state.side_one.get_active().hp / 16,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.pokemon[PokemonIndex::P1].hp = 5;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stickyweb() {
        let mut state: State = State::default();
        state.side_one.side_conditions.sticky_web = 1;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: -1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stickyweb_with_heavydutyboots() {
        let mut state: State = State::default();
        state.side_one.side_conditions.sticky_web = 1;
        state.side_one.pokemon[PokemonIndex::P1].item = Items::HEAVYDUTYBOOTS;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stickyweb_with_contrary() {
        let mut state: State = State::default();
        state.side_one.side_conditions.sticky_web = 1;
        state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::CONTRARY;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideOne,
                    stat: PokemonBoostableStat::Speed,
                    amount: 1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_single_layer_toxicspikes() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 1;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P1,
                    old_status: PokemonStatus::NONE,
                    new_status: PokemonStatus::POISON,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 2;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P1,
                    old_status: PokemonStatus::NONE,
                    new_status: PokemonStatus::TOXIC,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes_as_flying_type() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 2;
        state.side_one.pokemon[PokemonIndex::P1].types.0 = PokemonType::FLYING;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes_as_poison_and_flying_type() {
        let mut state: State = State::default();
        state.side_one.side_conditions.toxic_spikes = 2;
        state.side_one.pokemon[PokemonIndex::P1].types.0 = PokemonType::FLYING;
        state.side_one.pokemon[PokemonIndex::P1].types.1 = PokemonType::POISON;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_in_with_intimidate() {
        let mut state: State = State::default();
        state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Boost(BoostInstruction {
                    side_ref: SideReference::SideTwo,
                    stat: PokemonBoostableStat::Attack,
                    amount: -1,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_in_with_intimidate_when_opponent_is_already_lowest_atk_boost() {
        let mut state: State = State::default();
        state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;
        state.side_two.attack_boost = -6;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_in_with_intimidate_versus_clearbody() {
        let mut state: State = State::default();
        state.side_one.pokemon[PokemonIndex::P1].ability = Abilities::INTIMIDATE;
        state.side_two.get_active().ability = Abilities::CLEARBODY;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Switch(SwitchInstruction {
                side_ref: SideReference::SideOne,
                previous_index: PokemonIndex::P0,
                next_index: PokemonIndex::P1,
            })],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_double_layer_toxicspikes_as_poison_type() {
        let mut state: State = State::default();
        state.side_one.pokemon[PokemonIndex::P1].types.0 = PokemonType::POISON;
        state.side_one.side_conditions.toxic_spikes = 2;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicSpikes,
                    amount: -2,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock_and_spikes_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.side_conditions.spikes = 1;
        state.side_one.pokemon[PokemonIndex::P1].hp = 15;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 3,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_switching_into_stealthrock_and_multiple_layers_of_spikes_does_not_overkill() {
        let mut state: State = State::default();
        state.side_one.side_conditions.stealth_rock = 1;
        state.side_one.side_conditions.spikes = 3;
        state.side_one.pokemon[PokemonIndex::P1].hp = 25;
        let mut choice = Choice {
            ..Default::default()
        };
        choice.switch_id = PokemonIndex::P1;

        let expected_instructions: StateInstructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Switch(SwitchInstruction {
                    side_ref: SideReference::SideOne,
                    previous_index: PokemonIndex::P0,
                    next_index: PokemonIndex::P1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 13,
                }),
            ],
            ..Default::default()
        };

        let mut incoming_instructions = StateInstructions::default();
        generate_instructions_from_switch::<GEN>(
            &mut state,
            choice.switch_id,
            SideReference::SideOne,
            &mut incoming_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_healthy_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions::default();

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            &mut vec![],
        );

        assert_eq!(expected_instructions, incoming_instructions);
    }

    #[test]
    fn test_rest_turns_at_3_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().rest_turns = 3;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::DecrementRestTurns(
                DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                },
            )],
        };

        let expected_frozen_instructions: &mut Vec<StateInstructions> = &mut vec![];

        let frozen_instructions = &mut vec![];
        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_rest_turns_at_2_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().rest_turns = 2;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::DecrementRestTurns(
                DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                },
            )],
        };

        let expected_frozen_instructions: &mut Vec<StateInstructions> = &mut vec![];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_paralyzed_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: (1.0 - FULLY_PARALYZED_CHANCE) * 100.0,
            instruction_list: vec![],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: FULLY_PARALYZED_CHANCE * 100.0,
            instruction_list: vec![],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_confused_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::CONFUSION);
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0 * (1.0 - HIT_SELF_IN_CONFUSION_CHANCE),
            instruction_list: vec![],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 100.0 * (HIT_SELF_IN_CONFUSION_CHANCE),
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 35,
            })],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_confused_pokemon_with_prior_instruction() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::CONFUSION);
        let mut incoming_instructions = StateInstructions::default();
        incoming_instructions.instruction_list = vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })];

        let expected_instructions = StateInstructions {
            percentage: 100.0 * (1.0 - HIT_SELF_IN_CONFUSION_CHANCE),
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 100.0 * HIT_SELF_IN_CONFUSION_CHANCE,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 35,
                }),
            ],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_confused_pokemon_with_prior_instruction_does_not_overkill() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::CONFUSION);
        let mut incoming_instructions = StateInstructions::default();
        state.side_one.get_active().hp = 2;
        incoming_instructions.instruction_list = vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })];

        let expected_instructions = StateInstructions {
            percentage: 100.0 * (1.0 - HIT_SELF_IN_CONFUSION_CHANCE),
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 100.0 * HIT_SELF_IN_CONFUSION_CHANCE,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 1,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 2,
                }),
            ],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_frozen_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::FREEZE;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: THAW_CHANCE * 100.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: state.side_one.active_index,
                old_status: PokemonStatus::FREEZE,
                new_status: PokemonStatus::NONE,
            })],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: (1.0 - THAW_CHANCE) * 100.0,
            instruction_list: vec![],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_asleep_pokemon_with_no_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().sleep_turns = MAX_SLEEP_TURNS;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: state.side_one.active_index,
                    old_status: PokemonStatus::SLEEP,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 0,
                    previous_turns: MAX_SLEEP_TURNS,
                }),
            ],
        };

        let expected_frozen_instructions: &mut Vec<StateInstructions> = &mut vec![];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_asleep_waking_up_and_confused() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().sleep_turns = MAX_SLEEP_TURNS;
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::CONFUSION);
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0 * (1.0 - HIT_SELF_IN_CONFUSION_CHANCE),
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: state.side_one.active_index,
                    old_status: PokemonStatus::SLEEP,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 0,
                    previous_turns: MAX_SLEEP_TURNS,
                }),
            ],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: 100.0 * HIT_SELF_IN_CONFUSION_CHANCE,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: state.side_one.active_index,
                    old_status: PokemonStatus::SLEEP,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::SetSleepTurns(SetSleepTurnsInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: PokemonIndex::P0,
                    new_turns: 0,
                    previous_turns: MAX_SLEEP_TURNS,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 35,
                }),
            ],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_asleep_pokemon_waking_up_with_1_rest_turn() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().rest_turns = 1;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: state.side_one.active_index,
                    old_status: PokemonStatus::SLEEP,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::DecrementRestTurns(DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                }),
            ],
        };

        let expected_frozen_instructions: &mut Vec<StateInstructions> = &mut vec![];
        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_asleep_pokemon_staying_asleep_with_two_rest_turns() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::SLEEP;
        state.side_one.get_active().rest_turns = 1;
        let mut incoming_instructions = StateInstructions::default();

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeStatus(ChangeStatusInstruction {
                    side_ref: SideReference::SideOne,
                    pokemon_index: state.side_one.active_index,
                    old_status: PokemonStatus::SLEEP,
                    new_status: PokemonStatus::NONE,
                }),
                Instruction::DecrementRestTurns(DecrementRestTurnsInstruction {
                    side_ref: SideReference::SideOne,
                }),
            ],
        };

        let expected_frozen_instructions: &mut Vec<StateInstructions> = &mut vec![];
        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_paralyzed_pokemon_preserves_prior_instructions() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        let mut incoming_instructions = StateInstructions::default();
        incoming_instructions.instruction_list = vec![Instruction::Damage(DamageInstruction {
            side_ref: SideReference::SideOne,
            damage_amount: 1,
        })];

        let expected_instructions = StateInstructions {
            percentage: (1.0 - FULLY_PARALYZED_CHANCE) * 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        };

        let expected_frozen_instructions = &mut vec![StateInstructions {
            percentage: FULLY_PARALYZED_CHANCE * 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 1,
            })],
        }];

        let frozen_instructions = &mut vec![];

        generate_instructions_from_existing_status_conditions::<GEN>(
            &mut state,
            &SideReference::SideOne,
            &Choice::default(),
            &mut incoming_instructions,
            frozen_instructions,
        );

        assert_eq!(expected_instructions, incoming_instructions);
        assert_eq!(expected_frozen_instructions, frozen_instructions);
    }

    #[test]
    fn test_basic_side_two_moves_first() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            SideMovesFirst::SideTwo,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_custap_berry_when_less_than_25_percent_activates() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().item = Items::CUSTAPBERRY;
        state.side_one.get_active().hp = 24;
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            SideMovesFirst::SideOne,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_quarkdrivespe_boost_works() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::QUARKDRIVESPE);
        state.side_one.get_active().hp = 24;
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            SideMovesFirst::SideOne,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_protosynthesisspe_boost_works() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::PROTOSYNTHESISSPE);
        state.side_one.get_active().hp = 24;
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            SideMovesFirst::SideOne,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_custap_berry_when_greater_than_25_percent_does_not_activate() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().item = Items::CUSTAPBERRY;
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            SideMovesFirst::SideTwo,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_custap_berry_does_not_matter_when_opponent_uses_increased_priority_move() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::QUICKATTACK).unwrap().to_owned();
        state.side_one.get_active().item = Items::CUSTAPBERRY;
        state.side_one.get_active().hp = 24;
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            SideMovesFirst::SideTwo,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_slowstart_halves_effective_speed() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;
        state
            .side_two
            .volatile_statuses
            .insert(PokemonVolatileStatus::SLOWSTART);

        assert_eq!(
            SideMovesFirst::SideOne,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_basic_side_one_moves_first() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().speed = 101;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            SideMovesFirst::SideOne,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_paralysis_reduces_effective_speed() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();

        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        state.side_one.get_active().speed = 101;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            SideMovesFirst::SideTwo,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_later_gen_speed_cutting_in_half() {
        if !(GEN == 7 || GEN == 8 || GEN == 9) {
            return;
        }
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        state.side_one.get_active().speed = 100;

        assert_eq!(50, get_effective_speed::<GEN>(&state, &SideReference::SideOne))
    }

    #[test]
    fn test_earlier_gen_speed_cutting_by_75_percent() {
        if !(GEN == 4 || GEN == 5 || GEN == 6) {
            return;
        }
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::PARALYZE;
        state.side_one.get_active().speed = 100;

        assert_eq!(25, get_effective_speed::<GEN>(&state, &SideReference::SideOne))
    }

    #[test]
    fn test_choicescarf_multiplying_speed() {
        let mut state = State::default();
        state.side_one.get_active().speed = 100;
        state.side_one.get_active().item = Items::CHOICESCARF;

        assert_eq!(150, get_effective_speed::<GEN>(&state, &SideReference::SideOne))
    }

    #[test]
    fn test_iron_ball_halving_speed() {
        let mut state = State::default();
        state.side_one.get_active().speed = 100;
        state.side_one.get_active().item = Items::IRONBALL;

        assert_eq!(50, get_effective_speed::<GEN>(&state, &SideReference::SideOne))
    }

    #[test]
    fn test_speed_tie_goes_to_side_two() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            SideMovesFirst::SpeedTie,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_higher_priority_ignores_speed_diff() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::QUICKATTACK).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        state.side_one.get_active().speed = 100;
        state.side_two.get_active().speed = 101;

        assert_eq!(
            SideMovesFirst::SideOne,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_side_two_higher_priority_ignores_speed_diff() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::TACKLE).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::QUICKATTACK).unwrap().to_owned();
        state.side_one.get_active().speed = 101;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            SideMovesFirst::SideTwo,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_both_higher_priority_defaults_back_to_speed() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::QUICKATTACK).unwrap().to_owned();
        let side_two_choice = moves::<GEN>().get(&Choices::QUICKATTACK).unwrap().to_owned();
        state.side_one.get_active().speed = 101;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            SideMovesFirst::SideOne,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_switch_always_goes_first() {
        let mut state = State::default();
        let mut side_one_choice = moves::<GEN>().get(&Choices::SPLASH).unwrap().to_owned();
        side_one_choice.category = MoveCategory::Switch;
        let side_two_choice = moves::<GEN>().get(&Choices::QUICKATTACK).unwrap().to_owned();
        state.side_one.get_active().speed = 99;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            SideMovesFirst::SideOne,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_double_switch_checks_higher_speed() {
        let mut state = State::default();
        let mut side_one_choice = moves::<GEN>().get(&Choices::SPLASH).unwrap().to_owned();
        side_one_choice.category = MoveCategory::Switch;
        let mut side_two_choice = moves::<GEN>().get(&Choices::SPLASH).unwrap().to_owned();
        side_two_choice.category = MoveCategory::Switch;

        state.side_one.get_active().speed = 99;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            SideMovesFirst::SideTwo,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_pursuit_goes_before_switch() {
        let mut state = State::default();
        let side_one_choice = moves::<GEN>().get(&Choices::PURSUIT).unwrap().to_owned();
        let mut side_two_choice = moves::<GEN>().get(&Choices::SPLASH).unwrap().to_owned();
        side_two_choice.category = MoveCategory::Switch;

        state.side_one.get_active().speed = 50;
        state.side_two.get_active().speed = 100;

        assert_eq!(
            SideMovesFirst::SideOne,
            moves_first::<GEN>(
                &state,
                &side_one_choice,
                &side_two_choice,
                &mut StateInstructions::default()
            )
        )
    }

    #[test]
    fn test_end_of_turn_hail_damage() {
        let mut state = State::default();
        state.weather.weather_type = Weather::HAIL;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 6,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_end_of_turn_hail_damage_against_ice_type() {
        let mut state = State::default();
        state.weather.weather_type = Weather::HAIL;
        state.side_two.get_active().types.0 = PokemonType::ICE;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                // no damage to side_two
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_end_of_turn_sand_damage() {
        let mut state = State::default();
        state.weather.weather_type = Weather::SAND;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 6,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_end_of_turn_sand_damage_against_ground_type() {
        let mut state = State::default();
        state.weather.weather_type = Weather::SAND;
        state.side_two.get_active().types.0 = PokemonType::GROUND;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,

            // no damage to side_two
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_hail_does_not_overkill() {
        let mut state = State::default();
        state.weather.weather_type = Weather::HAIL;
        state.side_one.get_active().hp = 3;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 3,
                }),
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideTwo,
                    damage_amount: 6,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_fainted_pkmn_does_not_take_hail_dmg() {
        let mut state = State::default();
        state.weather.weather_type = Weather::HAIL;
        state.side_one.get_active().hp = 0;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideTwo,
                damage_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_wished_pokemon_gets_healed() {
        if GEN == 4 {
            return;
        }
        let mut state = State::default();
        state.side_one.wish = (1, 5);
        state.side_one.get_active().hp = 50;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 5,
                }),
                Instruction::DecrementWish(DecrementWishInstruction {
                    side_ref: SideReference::SideOne,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_wish_does_not_overheal() {
        let mut state = State::default();
        state.side_one.wish = (1, 50);
        state.side_one.get_active().hp = 95;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 5,
                }),
                Instruction::DecrementWish(DecrementWishInstruction {
                    side_ref: SideReference::SideOne,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_wish_does_nothing_when_maxhp() {
        let mut state = State::default();
        state.side_one.wish = (1, 50);
        state.side_one.get_active().hp = 100;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::DecrementWish(DecrementWishInstruction {
                side_ref: SideReference::SideOne,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_wish_does_nothing_when_fainted() {
        let mut state = State::default();
        state.side_one.wish = (1, 50);
        state.side_one.get_active().hp = 0;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::DecrementWish(DecrementWishInstruction {
                side_ref: SideReference::SideOne,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_wish_at_2_does_not_heal() {
        let mut state = State::default();
        state.side_one.wish = (2, 50);
        state.side_one.get_active().hp = 95;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::DecrementWish(DecrementWishInstruction {
                side_ref: SideReference::SideOne,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leftovers_heals_at_end_of_turn() {
        let mut state = State::default();
        state.side_one.get_active().hp = 50;
        state.side_one.get_active().item = Items::LEFTOVERS;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leftovers_does_not_overheal() {
        let mut state = State::default();
        state.side_one.get_active().hp = 99;
        state.side_one.get_active().item = Items::LEFTOVERS;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 1,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leftovers_generates_no_instruction_at_maxhp() {
        let mut state = State::default();
        state.side_one.get_active().hp = 100;
        state.side_one.get_active().item = Items::LEFTOVERS;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leftovers_generates_no_instruction_when_fainted() {
        let mut state = State::default();
        state.side_one.get_active().hp = 0;
        state.side_one.get_active().item = Items::LEFTOVERS;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_blacksludge_heal_as_poison_type() {
        let mut state = State::default();
        state.side_one.get_active().hp = 50;
        state.side_one.get_active().item = Items::BLACKSLUDGE;
        state.side_one.get_active().types.0 = PokemonType::POISON;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_blacksludge_damage_as_non_poison_type() {
        let mut state = State::default();
        state.side_one.get_active().hp = 50;
        state.side_one.get_active().item = Items::BLACKSLUDGE;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_blacksludge_does_not_overheal() {
        let mut state = State::default();
        state.side_one.get_active().hp = 99;
        state.side_one.get_active().item = Items::BLACKSLUDGE;
        state.side_one.get_active().types.0 = PokemonType::POISON;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 1,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_flameorb_end_of_turn_burn() {
        let mut state = State::default();
        state.side_one.get_active().item = Items::FLAMEORB;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::NONE,
                new_status: PokemonStatus::BURN,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_fire_type_cannot_be_burned_by_flameorb() {
        let mut state = State::default();
        state.side_one.get_active().item = Items::FLAMEORB;
        state.side_one.get_active().types.0 = PokemonType::FIRE;
        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_toxicorb_applies_status() {
        let mut state = State::default();
        state.side_one.get_active().item = Items::TOXICORB;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: SideReference::SideOne,
                pokemon_index: PokemonIndex::P0,
                old_status: PokemonStatus::NONE,
                new_status: PokemonStatus::TOXIC,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_toxicorb_does_not_apply_to_poison_type() {
        let mut state = State::default();
        state.side_one.get_active().item = Items::TOXICORB;
        state.side_one.get_active().types.0 = PokemonType::POISON;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_poisonheal_heals_at_end_of_turn() {
        let mut state = State::default();
        state.side_one.get_active().ability = Abilities::POISONHEAL;
        state.side_one.get_active().status = PokemonStatus::POISON;
        state.side_one.get_active().hp = 50;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 12,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_poisonheal_while_toxiced_still_increases_toxic_count() {
        let mut state = State::default();
        state.side_one.get_active().ability = Abilities::POISONHEAL;
        state.side_one.get_active().status = PokemonStatus::TOXIC;
        state.side_one.get_active().hp = 50;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicCount,
                    amount: 1,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideOne,
                    heal_amount: 12,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_poisonheal_does_not_overheal() {
        let mut state = State::default();
        state.side_one.get_active().ability = Abilities::POISONHEAL;
        state.side_one.get_active().status = PokemonStatus::POISON;
        state.side_one.get_active().hp = 99;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Heal(HealInstruction {
                side_ref: SideReference::SideOne,
                heal_amount: 1,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_poisonheal_does_nothing_at_maxhp() {
        let mut state = State::default();
        state.side_one.get_active().ability = Abilities::POISONHEAL;
        state.side_one.get_active().status = PokemonStatus::POISON;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_speedboost() {
        let mut state = State::default();
        state.side_one.get_active().ability = Abilities::SPEEDBOOST;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Boost(BoostInstruction {
                side_ref: SideReference::SideOne,
                stat: PokemonBoostableStat::Speed,
                amount: 1,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_speedboost_does_not_boost_beyond_6() {
        let mut state = State::default();
        state.side_one.get_active().ability = Abilities::SPEEDBOOST;
        state.side_one.speed_boost = 6;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_end_of_turn_poison_damage() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::POISON;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 12,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_poison_damage_does_not_overkill() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::POISON;
        state.side_one.get_active().hp = 5;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 5,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_end_of_turn_burn_damage() {
        if !(GEN == 7 || GEN == 8 || GEN == 9) {
            return;
        }
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::BURN;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 6,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_early_generation_burn_one_eigth() {
        if !(GEN == 4 || GEN == 5 || GEN == 6) {
            return;
        }
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::BURN;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 12,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_burn_damage_does_not_overkill() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::BURN;
        state.side_one.get_active().hp = 5;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 5,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_burn_damage_ignored_if_has_magicguard() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::BURN;
        state.side_one.get_active().ability = Abilities::MAGICGUARD;
        state.side_one.get_active().hp = 5;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_first_toxic_damage() {
        let mut state = State::default();
        state.side_one.get_active().status = PokemonStatus::TOXIC;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::ToxicCount,
                    amount: 1,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leechseed_sap() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::LEECHSEED);
        state.side_one.get_active().hp = 50;
        state.side_two.get_active().hp = 50;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideTwo,
                    heal_amount: 12,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leechseed_sap_does_not_heal_if_receiving_side_is_maxhp() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::LEECHSEED);
        state.side_one.get_active().hp = 50;
        state.side_two.get_active().hp = 100;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: 12,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leechseed_sap_does_not_overkill() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::LEECHSEED);
        state.side_one.get_active().hp = 5;
        state.side_two.get_active().hp = 50;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 5,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideTwo,
                    heal_amount: 5,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_leechseed_sap_does_not_overheal() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::LEECHSEED);
        state.side_one.get_active().hp = 50;
        state.side_two.get_active().hp = 95;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                }),
                Instruction::Heal(HealInstruction {
                    side_ref: SideReference::SideTwo,
                    heal_amount: 5,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_protect_volatile_being_removed() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::PROTECT);

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![
                Instruction::RemoveVolatileStatus(RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::PROTECT,
                }),
                Instruction::ChangeSideCondition(ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Protect,
                    amount: 1,
                }),
            ],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_protect_side_condition_being_removed() {
        let mut state = State::default();
        state.side_one.side_conditions.protect = 2;

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::ChangeSideCondition(
                ChangeSideConditionInstruction {
                    side_ref: SideReference::SideOne,
                    side_condition: PokemonSideCondition::Protect,
                    amount: -2,
                },
            )],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_roost_vs_removal() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::ROOST);

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::RemoveVolatileStatus(
                RemoveVolatileStatusInstruction {
                    side_ref: SideReference::SideOne,
                    volatile_status: PokemonVolatileStatus::ROOST,
                },
            )],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_partiallytrapped_damage() {
        let mut state = State::default();
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::PARTIALLYTRAPPED);

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_instructions = if GEN == 4 || GEN == 5 {
            StateInstructions {
                percentage: 100.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 6,
                })],
            }
        } else {
            StateInstructions {
                percentage: 100.0,
                instruction_list: vec![Instruction::Damage(DamageInstruction {
                    side_ref: SideReference::SideOne,
                    damage_amount: 12,
                })],
            }
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_saltcure_on_water_type_damage() {
        let mut state = State::default();
        state.side_one.get_active().types.0 = PokemonType::WATER;
        state
            .side_one
            .volatile_statuses
            .insert(PokemonVolatileStatus::SALTCURE);

        let mut incoming_instructions = StateInstructions::default();
        add_end_of_turn_instructions::<GEN>(
            &mut state,
            &mut incoming_instructions,
            &SideReference::SideOne,
        );

        let expected_damage = (2.0 * (100.0 / SALT_CURE_DAMAGE_DIVISOR)) as i16;
        let expected_instructions = StateInstructions {
            percentage: 100.0,
            instruction_list: vec![Instruction::Damage(DamageInstruction {
                side_ref: SideReference::SideOne,
                damage_amount: expected_damage,
            })],
        };

        assert_eq!(expected_instructions, incoming_instructions)
    }

    #[test]
    fn test_chance_to_wake_up_with_no_turns_asleep_is_0() {
        assert_eq!(0.0, chance_to_wake_up::<GEN>(0));
    }

    #[test]
    fn test_gen4_25_percent_to_wake_after_1_sleep_turn() {
        if GEN != 4 {
            return;
        }
        assert_eq!(0.25, chance_to_wake_up::<GEN>(1));
    }

    #[test]
    fn test_gen4_100_percent_to_wake_after_4_sleep_turn() {
        if GEN != 4 {
            return;
        }
        assert_eq!(1.0, chance_to_wake_up::<GEN>(4));
    }
