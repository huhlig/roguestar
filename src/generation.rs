//
//  Roguestar - An experimental Roguelike Adventure across the stars.
//  Copyright (C) 2021 Hans W. Uhlig
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

//! World Generation is responsible for loading or creating a new Universe to play in.
mod apartment;
mod context;
mod faction;
mod sector;

pub use self::context::{GenerationConfiguration, GenerationContext, GenerationState};
pub use self::faction::{generate_government_faction, ProtoFaction};
pub use self::sector::{ProtoCluster, ProtoOrbital, ProtoSector};
use super::GameState;
use bevy::ecs::schedule::StateError;
use bevy::prelude::*;

pub fn bootstrap_worldgeneration(app: &mut AppBuilder) {
    app.add_system_set(
        SystemSet::on_enter(GameState::WorldGeneration).with_system(setup_worldgen.system()),
    )
    .add_system_set(
        SystemSet::on_update(GameState::WorldGeneration).with_system(update_worldgen.system()),
    )
    .add_system_set(
        SystemSet::on_exit(GameState::WorldGeneration).with_system(cleanup_worldgen.system()),
    );
}

pub fn setup_worldgen(mut commands: Commands) {
    commands.insert_resource(GenerationContext::default());
}

pub fn update_worldgen(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut context: ResMut<GenerationContext>,
) {
    if context.state == GenerationState::Complete {
        state
            .set(GameState::GameplayMode)
            .expect("Error Starting Game Engine");
    } else {
        context.step(commands);
    }
}

pub fn cleanup_worldgen(mut commands: Commands) {
    trace!("Cleanup WorldGen");
    commands.remove_resource::<GenerationContext>()
}
