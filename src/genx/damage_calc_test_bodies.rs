    #[test]
    fn test_basic_damaging_move() {
        let state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.move_id = Choices::TACKLE;
        choice.move_type = PokemonType::TYPELESS;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage::<GEN>(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        // level 100 tackle with 100 base stats across the board (attacker & defender)
        assert_eq!(32, dmg.unwrap().0);
    }

    #[test]
    fn test_basic_non_damaging_move() {
        let state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.move_id = Choices::PROTECT;
        choice.category = MoveCategory::Status;

        let dmg = calculate_damage::<GEN>(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(None, dmg);
    }

    #[test]
    fn test_move_with_zero_base_power() {
        let state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        choice.move_id = Choices::TACKLE;
        choice.move_type = PokemonType::TYPELESS;
        choice.base_power = 0.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage::<GEN>(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(0, dmg.unwrap().0);
    }

    #[test]
    fn test_boosted_damaging_move() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        state.side_one.attack_boost = 1;
        choice.move_id = Choices::TACKLE;
        choice.move_type = PokemonType::TYPELESS;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage::<GEN>(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(48, dmg.unwrap().0);
    }

    #[test]
    fn test_unaware_does_not_get_damaged_by_boosted_stats() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        state.side_one.attack_boost = 1;
        state.side_two.get_active().ability = Abilities::UNAWARE;
        choice.move_id = Choices::TACKLE;
        choice.move_type = PokemonType::TYPELESS;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage::<GEN>(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(32, dmg.unwrap().0);
    }

    #[test]
    fn test_unaware_does_get_damaged_by_boosted_stats_if_attacker_has_moldbreaker() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };
        state.side_one.attack_boost = 1;
        state.side_two.get_active().ability = Abilities::UNAWARE;
        state.side_one.get_active().ability = Abilities::MOLDBREAKER;
        choice.move_id = Choices::TACKLE;
        choice.move_type = PokemonType::TYPELESS;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Physical;

        let dmg = calculate_damage::<GEN>(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(48, dmg.unwrap().0);
    }

    #[test]
    fn test_basic_super_effective_move() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };

        state.side_two.get_active().types = (PokemonType::FIRE, PokemonType::TYPELESS);
        choice.move_id = Choices::WATERGUN;
        choice.move_type = PokemonType::WATER;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Special;
        let dmg = calculate_damage::<GEN>(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(64, dmg.unwrap().0);
    }

    #[test]
    fn test_basic_not_very_effective_move() {
        let mut state = State::default();
        let mut choice = Choice {
            ..Default::default()
        };

        state.side_two.get_active().types = (PokemonType::WATER, PokemonType::TYPELESS);
        choice.move_id = Choices::WATERGUN;
        choice.move_type = PokemonType::WATER;
        choice.base_power = 40.0;
        choice.category = MoveCategory::Special;
        let dmg = calculate_damage::<GEN>(
            &state,
            &SideReference::SideOne,
            &choice,
            DamageRolls::Average,
        );

        assert_eq!(15, dmg.unwrap().0);
    }

    macro_rules! weather_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (weather_type, move_type, expected_damage_amount) = $value;
                    let mut state = State::default();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.weather.weather_type = weather_type;

                    choice.move_type = move_type;
                    choice.base_power = 40.0;
                    choice.category = MoveCategory::Special;
                    let dmg = calculate_damage::<GEN>(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap().0);
                }
             )*
        }
    }
    weather_tests! {
        test_rain_boosting_water: (Weather::RAIN, PokemonType::WATER, 48),
        test_rain_not_boosting_normal: (Weather::RAIN, PokemonType::NORMAL, 48),
        test_sun_boosting_fire: (Weather::SUN, PokemonType::FIRE, 48),
        test_sun_reducing_water: (Weather::SUN, PokemonType::WATER, 15),
        test_sun_not_boosting_normal: (Weather::SUN, PokemonType::NORMAL, 48),
        test_heavy_rain_makes_fire_do_zero: (Weather::HEAVYRAIN, PokemonType::FIRE, 0),
        test_heavy_rain_boost_water: (Weather::HEAVYRAIN, PokemonType::WATER, 48),
        test_harsh_sun_makes_water_do_zero: (Weather::HARSHSUN, PokemonType::WATER, 0),
        test_harsh_sun_boosting_fire: (Weather::HARSHSUN, PokemonType::FIRE, 48),
    }

    macro_rules! stab_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (attacker_types, attacking_move_type, expected_damage_amount) = $value;
                    let mut state = State::default();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.side_one.get_active().types = attacker_types;

                    choice.move_type = attacking_move_type;
                    choice.base_power = 40.0;
                    choice.category = MoveCategory::Special;
                    let dmg = calculate_damage::<GEN>(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap().0);
                }
             )*
        }
    }
    stab_tests! {
        test_basic_stab: ((PokemonType::WATER, PokemonType::FIRE), PokemonType::WATER, 48),
        test_basic_without_stab: ((PokemonType::WATER, PokemonType::FIRE), PokemonType::NORMAL, 32),
    }

    macro_rules! burn_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (attacking_move_category, expected_damage_amount) = $value;
                    let mut state = State::default();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.side_one.get_active().status = PokemonStatus::BURN;

                    choice.category = attacking_move_category;
                    choice.move_type = PokemonType::TYPELESS;
                    choice.base_power = 40.0;
                    let dmg = calculate_damage::<GEN>(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap().0);
                }
             )*
        }
    }
    burn_tests! {
        test_physical_move_when_burned_reduces: (MoveCategory::Physical, 15),
        test_special_move_when_burned_does_not_reduce: (MoveCategory::Special, 32),
    }

    macro_rules! screens_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (reflect_count, lightscreen_count, auroraveil_count, move_category, expected_damage_amount) = $value;
                    let mut state = State::default();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    state.side_two.side_conditions.reflect = reflect_count;
                    state.side_two.side_conditions.light_screen = lightscreen_count;
                    state.side_two.side_conditions.aurora_veil = auroraveil_count;

                    choice.category = move_category;
                    choice.base_power = 40.0;
                    choice.move_type = PokemonType::TYPELESS;
                    let dmg = calculate_damage::<GEN>(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap().0);
                }
             )*
        }
    }
    screens_tests! {
        test_reflect_reduces_physical_damage_by_half: (1, 0, 0, MoveCategory::Physical, 15),
        test_lightscreen_reduces_special_damage_by_half: (0, 1, 0, MoveCategory::Special, 15),
        test_auroraveil_reduces_physical_damage_by_half: (0, 0, 1, MoveCategory::Physical, 15),
        test_auroraveil_reduces_special_damage_by_half: (0, 0, 1, MoveCategory::Special, 15),
        test_reflect_does_not_reduce_special_damage: (1, 0, 0, MoveCategory::Special, 32),
        test_light_screen_does_not_reduce_physical_damage: (0, 1, 0, MoveCategory::Physical, 32),
        test_auroraveil_does_not_stack_with_reflect: (1, 1, 1, MoveCategory::Physical, 15),
        test_auroraveil_does_not_stack_with_lightscreen: (1, 1, 1, MoveCategory::Special, 15),
    }

    macro_rules! volatile_status_tests{
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (attacking_volatile_status, defending_volatile_status, move_type, move_name, expected_damage_amount) = $value;
                    let mut state = State::default();
                    let mut choice = Choice {
                        ..Default::default()
                    };
                    let mut s1_vs = VolatileStatusBitset::default();
                    for vs in attacking_volatile_status {
                        s1_vs.insert(vs);
                    }
                    let mut s2_vs = VolatileStatusBitset::default();
                    for vs in defending_volatile_status {
                        s2_vs.insert(vs);
                    }
                    state.side_one.volatile_statuses = s1_vs;
                    state.side_two.volatile_statuses = s2_vs;

                    choice.move_id = move_name;
                    choice.category = MoveCategory::Physical;
                    choice.move_type = move_type;
                    choice.base_power = 40.0;
                    let dmg = calculate_damage::<GEN>(&state, &SideReference::SideOne, &choice, DamageRolls::Average);

                    assert_eq!(expected_damage_amount, dmg.unwrap().0);
                }
             )*
        }
    }
    volatile_status_tests! {
        test_flashfire_boosts_fire_move: (
            vec![PokemonVolatileStatus::FLASHFIRE],
            vec![],
            PokemonType::FIRE,
            Choices::NONE,
            48
        ),
        test_flashfire_does_not_boost_normal_move: (
            vec![PokemonVolatileStatus::FLASHFIRE],
            vec![],
            PokemonType::TYPELESS,
            Choices::NONE,
            32
        ),
        test_magnetrise_makes_pkmn_immune_to_ground_move: (
            vec![],
            vec![PokemonVolatileStatus::MAGNETRISE],
            PokemonType::GROUND,
            Choices::NONE,
            0
        ),
        test_thousandarrows_can_hit_magnetrise_pokemon: (
            vec![],
            vec![PokemonVolatileStatus::MAGNETRISE],
            PokemonType::GROUND,
            Choices::THOUSANDARROWS,
            32
        ),
        test_tarshot_boosts_fire_move: (
            vec![],
            vec![PokemonVolatileStatus::TARSHOT],
            PokemonType::FIRE,
            Choices::NONE,
            64
        ),
        test_slowstart_halves_move: (
            vec![PokemonVolatileStatus::SLOWSTART],
            vec![],
            PokemonType::NORMAL,
            Choices::NONE,
            24
        ),
        test_tarshot_and_flashfire_together: (
            vec![PokemonVolatileStatus::FLASHFIRE],
            vec![PokemonVolatileStatus::TARSHOT],
            PokemonType::FIRE,
            Choices::NONE,
            97
        ),
        test_glaiverush_doubles_damage_against: (
            vec![],
            vec![PokemonVolatileStatus::GLAIVERUSH],
            PokemonType::NORMAL,
            Choices::NONE,
            97
        ),
        test_phantomforce_on_defender_causes_0_damage: (
            vec![],
            vec![PokemonVolatileStatus::PHANTOMFORCE],
            PokemonType::NORMAL,
            Choices::NONE,
            0
        ),
    }
