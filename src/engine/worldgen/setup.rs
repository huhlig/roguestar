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

use super::super::common::PlayerAvatar;
use super::super::hyperspace::Hyperspace;
use super::super::GameState;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use crate::engine::worldgen::generate_galaxy;
use crate::engine::worldgen::params::Parameters;

pub fn setup_worldgen(
    mut commands: Commands,
    windows: Res<Windows>,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    trace!("Setup WorldGen");
    let parameters = Parameters {
        seed: 0,
        sector_density: 1.0,
        resource_density: 1.0
    };

    generate_galaxy(&mut commands, &parameters);
}

pub fn update_worldgen(mut commands: Commands, mut state: ResMut<State<GameState>>) {
    state.set(GameState::HyperspaceMode).unwrap();
}

pub fn cleanup_worldgen(mut commands: Commands) {
    trace!("Cleanup WorldGen");
}
