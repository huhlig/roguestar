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
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn setup_hyperspace(
    mut commands: Commands,
    windows: Res<Windows>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut camera_query: Query<(Entity, &Camera, &mut Transform), With<WorldCamera>>,
    mut hyperspace_mode: Query<()>
) {
    trace!("Setup Hyperspace");
    // Set Camera to right Scale and create background
    for (entity, camera, mut transform) in camera_query.iter_mut() {
        trace!("Configuring Camera({:?})", camera.name);
        let window = windows.get(camera.window).unwrap();
        let background_texture = crate::utility::texgen::hyperspace_texture(
            window.width() as usize,
            window.height() as usize,
            0,
            0,
        );
        transform.scale = Vec3::new(0.01, 0.01, 1.0);
        commands.entity(entity).with_children(|camera| {
            camera.spawn_bundle(SpriteBundle {
                material: materials.add(ColorMaterial::texture(textures.add(background_texture))),
                transform: Transform::from_xyz(0.0, 0.0, -999.0),
                ..Default::default()
            });
        });
    }
    for visible
}

pub fn update_hyperspace(mut commands: Commands) {}

pub fn cleanup_hyperspace(mut commands: Commands) {
    trace!("Cleanup MainMenu");
}
