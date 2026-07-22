use crate::choices::Choices;
use crate::instruction::{
    ChangeSideConditionInstruction, ChangeStatusInstruction, Instruction,
    RemoveVolatileStatusInstruction,
};
use crate::state::VolatileStatusBitset;
use crate::state::{
    LastUsedMove, Pokemon, PokemonBoostableStat, PokemonSideCondition, PokemonStatus, PokemonType,
    Side, SideReference, State,
};
use core::panic;

fn multiply_boost(boost_num: i8, stat_value: i16) -> i16 {
    match boost_num {
        -6 => stat_value * 25 / 100,
        -5 => stat_value * 28 / 100,
        -4 => stat_value * 33 / 100,
        -3 => stat_value * 40 / 100,
        -2 => stat_value * 50 / 100,
        -1 => stat_value * 66 / 100,
        0 => stat_value,
        1 => stat_value * 3 / 2,
        2 => stat_value * 4 / 2,
        3 => stat_value * 5 / 2,
        4 => stat_value * 6 / 2,
        5 => stat_value * 7 / 2,
        6 => stat_value * 8 / 2,
        _ => panic!("Invalid boost number"),
    }
}

// Unified across all engines (genx is the superset: it adds MoveTera,
// MoveMega and TeamPreview, which the gen1-3 engines never construct).
pub use crate::genx::state::MoveChoice;

pub use crate::genx::state::PokemonVolatileStatus;

pub use crate::genx::state::Weather;

pub use crate::genx::state::Terrain;

impl Pokemon {
    pub fn gen2_add_available_moves(
        &self,
        vec: &mut Vec<MoveChoice>,
        last_used_move: &LastUsedMove,
        encored: bool,
    ) {
        let mut iter = self.moves.into_iter();
        while let Some(p) = iter.next() {
            if !p.disabled && p.pp > 0 {
                match last_used_move {
                    LastUsedMove::Move(last_used_move) => {
                        if encored && last_used_move != &iter.pokemon_move_index {
                            continue;
                        } else if (self.moves[last_used_move].id == Choices::BLOODMOON
                            || self.moves[last_used_move].id == Choices::GIGATONHAMMER)
                            && &iter.pokemon_move_index == last_used_move
                        {
                            continue;
                        }
                    }
                    _ => {
                        // there are some situations where you switched out and got encored into
                        // a move from a different pokemon because you also have that move.
                        // just assume nothing is locked in this case
                    }
                }
                vec.push(MoveChoice::Move(iter.pokemon_move_index));
            }
        }
    }

    pub fn gen2_add_move_from_choice(&self, vec: &mut Vec<MoveChoice>, choice: Choices) {
        let mut iter = self.moves.into_iter();
        while let Some(p) = iter.next() {
            if p.id == choice {
                vec.push(MoveChoice::Move(iter.pokemon_move_index));
            }
        }
    }

    pub fn gen2_has_type(&self, pkmn_type: &PokemonType) -> bool {
        pkmn_type == &self.types.0 || pkmn_type == &self.types.1
    }

    pub fn gen2_item_is_permanent(&self) -> bool {
        false
    }

    pub fn gen2_item_can_be_removed(&self) -> bool {
        !self.gen2_item_is_permanent()
    }

    pub fn gen2_is_grounded(&self) -> bool {
        if self.gen2_has_type(&PokemonType::FLYING) {
            return false;
        }
        true
    }

    pub fn gen2_volatile_status_can_be_applied(
        &self,
        volatile_status: &PokemonVolatileStatus,
        active_volatiles: &VolatileStatusBitset,
        first_move: bool,
    ) -> bool {
        if active_volatiles.contains(volatile_status) || self.hp == 0 {
            return false;
        }
        match volatile_status {
            // grass immunity to leechseed covered by `powder`
            PokemonVolatileStatus::LEECHSEED | PokemonVolatileStatus::CONFUSION => {
                if active_volatiles.contains(&PokemonVolatileStatus::SUBSTITUTE) {
                    return false;
                }
                true
            }
            PokemonVolatileStatus::SUBSTITUTE => self.hp > self.maxhp / 4,
            PokemonVolatileStatus::FLINCH => {
                if !first_move {
                    return false;
                }
                true
            }
            PokemonVolatileStatus::PROTECT => first_move,
            _ => true,
        }
    }

    pub fn gen2_immune_to_stats_lowered_by_opponent(
        &self,
        _stat: &PokemonBoostableStat,
        volatiles: &VolatileStatusBitset,
    ) -> bool {
        if volatiles.contains(&PokemonVolatileStatus::SUBSTITUTE) {
            return true;
        }
        false
    }
}

impl Side {
    pub fn gen2_get_boost_from_boost_enum(&self, boost_enum: &PokemonBoostableStat) -> i8 {
        match boost_enum {
            PokemonBoostableStat::Attack => self.attack_boost,
            PokemonBoostableStat::Defense => self.defense_boost,
            PokemonBoostableStat::SpecialAttack => self.special_attack_boost,
            PokemonBoostableStat::SpecialDefense => self.special_defense_boost,
            PokemonBoostableStat::Speed => self.speed_boost,
            PokemonBoostableStat::Evasion => self.evasion_boost,
            PokemonBoostableStat::Accuracy => self.accuracy_boost,
        }
    }

    pub fn gen2_calculate_boosted_stat(&self, stat: PokemonBoostableStat) -> i16 {
        let active = self.get_active_immutable();
        match stat {
            PokemonBoostableStat::Attack => {
                let boost = self.attack_boost;
                multiply_boost(boost, active.attack)
            }
            PokemonBoostableStat::Defense => {
                let boost = self.defense_boost;
                multiply_boost(boost, active.defense)
            }
            PokemonBoostableStat::SpecialAttack => {
                let boost = self.special_attack_boost;
                multiply_boost(boost, active.special_attack)
            }
            PokemonBoostableStat::SpecialDefense => {
                let boost = self.special_defense_boost;
                multiply_boost(boost, active.special_defense)
            }
            PokemonBoostableStat::Speed => {
                let boost = self.speed_boost;
                multiply_boost(boost, active.speed)
            }
            _ => {
                panic!("Not implemented")
            }
        }
    }

    pub fn gen2_has_alive_non_rested_sleeping_pkmn(&self) -> bool {
        for p in self.pokemon.into_iter() {
            if p.status == PokemonStatus::SLEEP && p.hp > 0 && p.rest_turns == 0 {
                return true;
            }
        }
        false
    }

    pub fn gen2_has_alive_frozen_pokemon(&self) -> bool {
        for p in self.pokemon.into_iter() {
            if p.status == PokemonStatus::FREEZE && p.hp > 0 {
                return true;
            }
        }
        false
    }

    pub fn gen2_add_switches(&self, vec: &mut Vec<MoveChoice>) {
        let mut iter = self.pokemon.into_iter();
        while let Some(p) = iter.next() {
            if p.hp > 0 && iter.pokemon_index != self.active_index {
                vec.push(MoveChoice::Switch(iter.pokemon_index));
            }
        }
        if vec.len() == 0 {
            vec.push(MoveChoice::None);
        }
    }

    pub fn gen2_trapped(&self, _opponent_active: &Pokemon) -> bool {
        if self
            .volatile_statuses
            .contains(&PokemonVolatileStatus::LOCKEDMOVE)
        {
            return true;
        } else if self
            .volatile_statuses
            .contains(&PokemonVolatileStatus::PARTIALLYTRAPPED)
        {
            return true;
        }
        false
    }
}

impl State {
    pub fn gen2_root_get_all_options(&self) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        if self.team_preview {
            let mut s1_options = Vec::with_capacity(6);
            let mut s2_options = Vec::with_capacity(6);

            let mut pkmn_iter = self.side_one.pokemon.into_iter();
            while let Some(_) = pkmn_iter.next() {
                if self.side_one.pokemon[pkmn_iter.pokemon_index].hp > 0 {
                    s1_options.push(MoveChoice::Switch(pkmn_iter.pokemon_index));
                }
            }
            let mut pkmn_iter = self.side_two.pokemon.into_iter();
            while let Some(_) = pkmn_iter.next() {
                if self.side_two.pokemon[pkmn_iter.pokemon_index].hp > 0 {
                    s2_options.push(MoveChoice::Switch(pkmn_iter.pokemon_index));
                }
            }
            return (s1_options, s2_options);
        }

        let (mut s1_options, mut s2_options) = self.gen2_get_all_options();

        if self.side_one.force_trapped {
            s1_options.retain(|x| match x {
                MoveChoice::Move(_) => true,
                MoveChoice::Switch(_) => false,
                MoveChoice::None => true,
                MoveChoice::MoveTera(_) | MoveChoice::MoveMega(_) | MoveChoice::TeamPreview(..) => {
                    false
                }
            });
        }
        if self.side_one.slow_uturn_move {
            s1_options.clear();
            let encored = self
                .side_one
                .volatile_statuses
                .contains(&PokemonVolatileStatus::ENCORE);
            self.side_one
                .get_active_immutable()
                .gen2_add_available_moves(&mut s1_options, &self.side_one.last_used_move, encored);
        }

        if self.side_two.force_trapped {
            s2_options.retain(|x| match x {
                MoveChoice::Move(_) => true,
                MoveChoice::Switch(_) => false,
                MoveChoice::None => true,
                MoveChoice::MoveTera(_) | MoveChoice::MoveMega(_) | MoveChoice::TeamPreview(..) => {
                    false
                }
            });
        }
        if self.side_two.slow_uturn_move {
            s2_options.clear();
            let encored = self
                .side_two
                .volatile_statuses
                .contains(&PokemonVolatileStatus::ENCORE);
            self.side_two
                .get_active_immutable()
                .gen2_add_available_moves(&mut s2_options, &self.side_two.last_used_move, encored);
        }

        if s1_options.len() == 0 {
            s1_options.push(MoveChoice::None);
        }
        if s2_options.len() == 0 {
            s2_options.push(MoveChoice::None);
        }

        (s1_options, s2_options)
    }

    pub fn gen2_get_all_options(&self) -> (Vec<MoveChoice>, Vec<MoveChoice>) {
        let mut side_one_options: Vec<MoveChoice> = Vec::with_capacity(9);
        let mut side_two_options: Vec<MoveChoice> = Vec::with_capacity(9);

        let side_one_active = self.side_one.get_active_immutable();
        let side_two_active = self.side_two.get_active_immutable();

        if self.side_one.force_switch {
            self.side_one.gen2_add_switches(&mut side_one_options);
            if self.side_two.switch_out_move_second_saved_move == Choices::NONE {
                side_two_options.push(MoveChoice::None);
            } else {
                self.side_two
                    .get_active_immutable()
                    .gen2_add_move_from_choice(
                        &mut side_two_options,
                        self.side_two.switch_out_move_second_saved_move,
                    );
            }
            return (side_one_options, side_two_options);
        }

        if self.side_two.force_switch {
            self.side_two.gen2_add_switches(&mut side_two_options);
            if self.side_one.switch_out_move_second_saved_move == Choices::NONE {
                side_one_options.push(MoveChoice::None);
            } else {
                self.side_one
                    .get_active_immutable()
                    .gen2_add_move_from_choice(
                        &mut side_one_options,
                        self.side_one.switch_out_move_second_saved_move,
                    );
            }
            return (side_one_options, side_two_options);
        }

        let side_one_force_switch = self.side_one.get_active_immutable().hp <= 0;
        let side_two_force_switch = self.side_two.get_active_immutable().hp <= 0;

        if side_one_force_switch && side_two_force_switch {
            self.side_one.gen2_add_switches(&mut side_one_options);
            self.side_two.gen2_add_switches(&mut side_two_options);
            return (side_one_options, side_two_options);
        }
        if side_one_force_switch {
            self.side_one.gen2_add_switches(&mut side_one_options);
            side_two_options.push(MoveChoice::None);
            return (side_one_options, side_two_options);
        }
        if side_two_force_switch {
            side_one_options.push(MoveChoice::None);
            self.side_two.gen2_add_switches(&mut side_two_options);
            return (side_one_options, side_two_options);
        }

        if self
            .side_one
            .volatile_statuses
            .contains(&PokemonVolatileStatus::MUSTRECHARGE)
        {
            side_one_options.push(MoveChoice::None);
        } else {
            let encored = self
                .side_one
                .volatile_statuses
                .contains(&PokemonVolatileStatus::ENCORE);
            self.side_one
                .get_active_immutable()
                .gen2_add_available_moves(
                    &mut side_one_options,
                    &self.side_one.last_used_move,
                    encored,
                );
            if !self.side_one.gen2_trapped(side_two_active) {
                self.side_one.gen2_add_switches(&mut side_one_options);
            }
        }

        if self
            .side_two
            .volatile_statuses
            .contains(&PokemonVolatileStatus::MUSTRECHARGE)
        {
            side_two_options.push(MoveChoice::None);
        } else {
            let encored = self
                .side_two
                .volatile_statuses
                .contains(&PokemonVolatileStatus::ENCORE);
            self.side_two
                .get_active_immutable()
                .gen2_add_available_moves(
                    &mut side_two_options,
                    &self.side_two.last_used_move,
                    encored,
                );
            if !self.side_two.gen2_trapped(side_one_active) {
                self.side_two.gen2_add_switches(&mut side_two_options);
            }
        }

        if side_one_options.len() == 0 {
            side_one_options.push(MoveChoice::None);
        }
        if side_two_options.len() == 0 {
            side_two_options.push(MoveChoice::None);
        }

        (side_one_options, side_two_options)
    }

    pub fn gen2_reset_toxic(
        &mut self,
        side_ref: &SideReference,
        vec_to_add_to: &mut Vec<Instruction>,
    ) {
        let side = self.get_side(side_ref);
        if side.side_conditions.toxic_count > 0 {
            vec_to_add_to.push(Instruction::ChangeSideCondition(
                ChangeSideConditionInstruction {
                    side_ref: *side_ref,
                    side_condition: PokemonSideCondition::ToxicCount,
                    amount: -1 * side.side_conditions.toxic_count,
                },
            ));
            side.side_conditions.toxic_count = 0;
        }
        let active = side.get_active();
        if active.status == PokemonStatus::TOXIC {
            active.status = PokemonStatus::POISON;
            vec_to_add_to.push(Instruction::ChangeStatus(ChangeStatusInstruction {
                side_ref: *side_ref,
                pokemon_index: side.active_index,
                old_status: PokemonStatus::TOXIC,
                new_status: PokemonStatus::POISON,
            }));
        }
    }

    pub fn gen2_remove_volatile_statuses_on_switch(
        &mut self,
        side_ref: &SideReference,
        vec_to_add_to: &mut Vec<Instruction>,
        baton_passing: bool,
    ) {
        let side = self.get_side(side_ref);
        side.volatile_statuses.retain(&mut |pkmn_volatile_status| {
            let should_retain = match pkmn_volatile_status {
                PokemonVolatileStatus::SUBSTITUTE | PokemonVolatileStatus::LEECHSEED => {
                    baton_passing
                }
                _ => false,
            };

            if !should_retain {
                vec_to_add_to.push(Instruction::RemoveVolatileStatus(
                    RemoveVolatileStatusInstruction {
                        side_ref: *side_ref,
                        volatile_status: *pkmn_volatile_status,
                    },
                ));
            }
            should_retain
        });
    }

    pub fn gen2_terrain_is_active(&self, terrain: &Terrain) -> bool {
        &self.terrain.terrain_type == terrain && self.terrain.turns_remaining > 0
    }

    pub fn gen2_weather_is_active(&self, weather: &Weather) -> bool {
        &self.weather.weather_type == weather
    }

    fn gen2_state_contains_any_move(&self, moves: &[Choices]) -> bool {
        for s in [&self.side_one, &self.side_two] {
            for pkmn in s.pokemon.into_iter() {
                for mv in pkmn.moves.into_iter() {
                    if moves.contains(&mv.id) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn gen2_set_damage_dealt_flag(&mut self) {
        if self.gen2_state_contains_any_move(&[
            Choices::COUNTER,
            Choices::MIRRORCOAT,
            Choices::METALBURST,
            Choices::COMEUPPANCE,
            Choices::FOCUSPUNCH,
        ]) {
            self.use_damage_dealt = true
        }
    }

    pub fn gen2_set_last_used_move_flag(&mut self) {
        if self.gen2_state_contains_any_move(&[
            Choices::ENCORE,
            Choices::FAKEOUT,
            Choices::FIRSTIMPRESSION,
            Choices::BLOODMOON,
            Choices::GIGATONHAMMER,
        ]) {
            self.use_last_used_move = true
        }
    }

    pub fn gen2_set_conditional_mechanics(&mut self) {
        /*
        These mechanics are not always relevant but when they are it
        is important that they are enabled. Enabling them all the time would
        suffer about a 20% performance hit.
        */
        self.gen2_set_damage_dealt_flag();
        self.gen2_set_last_used_move_flag();
    }
}
