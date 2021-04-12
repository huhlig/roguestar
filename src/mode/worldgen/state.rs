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

use crate::common::WorldCamera;
use crate::mode::hyperspace::Hyperspace;
use crate::mode::GameState;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use tracing::instrument;

#[instrument(skip(
    commands,
    windows,
    state,
    asset_server,
    textures,
    materials,
    camera_query
))]
pub fn setup_worldgen(
    mut commands: Commands,
    windows: Res<Windows>,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut camera_query: Query<(Entity, &Camera), With<WorldCamera>>,
) {
    trace!("Setup WorldGen");
    for (entity, camera) in camera_query.iter_mut() {
        commands.entity(entity).with_children(|parent| {
            trace!("Adding Hyperspace Background to Camera({:?})", camera.name);
            let window = windows.get(camera.window).unwrap();
            let hyperspace_background_texture = crate::utility::texgen::hyperspace_texture(
                window.width() as usize,
                window.height() as usize,
                0,
                0,
            );
            parent
                // Spawn Hyperspace Background
                .spawn_bundle(SpriteBundle {
                    material: materials.add(ColorMaterial::texture(
                        textures.add(hyperspace_background_texture),
                    )),
                    visible: Visible {
                        is_visible: false,
                        is_transparent: false,
                    },
                    transform: Transform::from_xyz(0.0, 0.0, -999.0),
                    ..Default::default()
                })
                .insert(Hyperspace);
        });
    }
    super::generate_galaxy(commands, asset_server, textures, materials);
}

pub fn update_worldgen(mut commands: Commands, mut state: ResMut<State<GameState>>) {
    state.set(GameState::HyperspaceMode).unwrap();
}

pub fn cleanup_worldgen(mut commands: Commands) {
    trace!("Cleanup WorldGen");
}
