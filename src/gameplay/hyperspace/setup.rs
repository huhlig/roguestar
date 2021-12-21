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

use super::Hyperspace;
use crate::engine::PlayerAvatar;
//use crate::plugin::parallax::ParallaxBackgroundBundle;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::texture::AddressMode;

pub fn initialize_hyperspace(
    mut commands: Commands,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("sprites/ships/Ship-001.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 6, 6);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Create Camera, Ship and background
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(PlayerAvatar)
        .insert(Hyperspace)
        .with_children(|parent| {
            let mut camera_bundle = OrthographicCameraBundle::new_2d();
            let window = windows.get(camera_bundle.camera.window).unwrap();
            camera_bundle.transform = Transform::from_xyz(0.0, 0.0, 10.0);
            parent.spawn_bundle(camera_bundle).insert(Hyperspace);

            let bg1: Handle<Texture> = asset_server.load("backgrounds/title/bg1.png");
            textures
                .get_mut(bg1.clone())
                .unwrap() // We got none here... why
                .sampler
                .set_address_mode(AddressMode::Repeat);
            let bg2: Handle<Texture> = asset_server.load("backgrounds/title/bg2.png");
            textures
                .get_mut(bg2.clone())
                .unwrap()
                .sampler
                .set_address_mode(AddressMode::Repeat);
            let bg3: Handle<Texture> = asset_server.load("backgrounds/title/bg3.png");
            textures
                .get_mut(bg3.clone())
                .unwrap()
                .sampler
                .set_address_mode(AddressMode::Repeat);
            let bg4: Handle<Texture> = asset_server.load("backgrounds/title/bg4.png");
            textures
                .get_mut(bg4.clone())
                .unwrap()
                .sampler
                .set_address_mode(AddressMode::Repeat);
            let bg5: Handle<Texture> = asset_server.load("backgrounds/title/bg5.png");
            textures
                .get_mut(bg5.clone())
                .unwrap()
                .sampler
                .set_address_mode(AddressMode::Repeat);
            let bg6: Handle<Texture> = asset_server.load("backgrounds/title/bg6.png");
            textures
                .get_mut(bg6.clone())
                .unwrap()
                .sampler
                .set_address_mode(AddressMode::Repeat);
            // let hyperspace_background_texture = crate::utility::texgen::CloudTextureGenerator::new(0)
            // .texture(window.width() as usize, window.height() as usize, 0);

            // Spawn Hyperspace Background
            parent
                .spawn_bundle(ParallaxBackgroundBundle {
                    material: materials.add(ColorMaterial::texture(bg1)),
                    transform: Transform::from_xyz(0.0, 0.0, -1000.0),
                    visible: Visible {
                        is_visible: false,
                        is_transparent: false,
                    },
                    ..Default::default()
                })
                .insert(Hyperspace);
            parent
                .spawn_bundle(ParallaxBackgroundBundle {
                    material: materials.add(ColorMaterial::texture(bg2)),
                    transform: Transform::from_xyz(0.0, 0.0, -800.0),
                    visible: Visible {
                        is_visible: false,
                        is_transparent: false,
                    },
                    ..Default::default()
                })
                .insert(Hyperspace);
            parent
                .spawn_bundle(ParallaxBackgroundBundle {
                    material: materials.add(ColorMaterial::texture(bg3)),
                    transform: Transform::from_xyz(0.0, 0.0, -600.0),
                    visible: Visible {
                        is_visible: false,
                        is_transparent: false,
                    },
                    ..Default::default()
                })
                .insert(Hyperspace);
            parent
                .spawn_bundle(ParallaxBackgroundBundle {
                    material: materials.add(ColorMaterial::texture(bg4)),
                    transform: Transform::from_xyz(0.0, 0.0, -400.0),
                    visible: Visible {
                        is_visible: false,
                        is_transparent: false,
                    },
                    ..Default::default()
                })
                .insert(Hyperspace);
            parent
                .spawn_bundle(ParallaxBackgroundBundle {
                    material: materials.add(ColorMaterial::texture(bg5)),
                    transform: Transform::from_xyz(0.0, 0.0, -200.0),
                    visible: Visible {
                        is_visible: false,
                        is_transparent: false,
                    },
                    ..Default::default()
                })
                .insert(Hyperspace);
            parent
                .spawn_bundle(ParallaxBackgroundBundle {
                    material: materials.add(ColorMaterial::texture(bg6)),
                    transform: Transform::from_xyz(0.0, 0.0, -100.0),
                    visible: Visible {
                        is_visible: false,
                        is_transparent: false,
                    },
                    ..Default::default()
                })
                .insert(Hyperspace);
        });
}

pub fn setup_hyperspace(
    mut commands: Commands,
    mut hyperspace_query: Query<&mut Visible, With<Hyperspace>>,
    mut camera_query: Query<(Entity, &Camera, &mut Transform), With<Hyperspace>>,
) {
    trace!("Setup Hyperspace");

    trace!("Making Hyperspace Objects Visible");
    for mut visible in hyperspace_query.iter_mut() {
        visible.is_visible = true;
    }
}

pub fn update_hyperspace(mut commands: Commands) {}

pub fn cleanup_hyperspace(
    mut commands: Commands,
    mut hyperspace_query: Query<&mut Visible, With<Hyperspace>>,
) {
    trace!("Cleanup Hyperspace");
    for mut visible in hyperspace_query.iter_mut() {
        visible.is_visible = false;
    }
}
