//
// Copyright 2021 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

mod seed;
mod starfield;

use bevy::app::EventReader;
use bevy::asset::{AssetServer, Assets};
use bevy::ecs::{Commands, Res, ResMut};
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use bevy::input::ElementState;
use bevy::math::Vec2;
use bevy::render::texture::Texture;
use bevy::sprite::entity::SpriteSheetBundle;
use bevy::sprite::{ColorMaterial, TextureAtlas, TextureAtlasBuilder};
use nominae::Totro;
use rand::{Rng, RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn generation(
    commands: &mut Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
}

pub fn generate_sector(commands: &mut Commands, seed: u32, x: i16, y: i16) {
    let sector_seed = ((seed as u64) << 32) | ((x as u64) << 16) | (y as u64);
    let rng = Xoshiro256Plus::seed_from_u64(sector_seed);
    /*
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        texture_atlas_builder.add_texture(asset_server.load("planets/Static/Suns/1.png"), texture);
        let texture_handle = asset_server.load("planets/Static/Suns/1.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let primary_entity = commands
            .spawn(SpriteSheetBundle {
                texture_atlas: (),
                sprite: Default::default(),
                draw: false,
                visible: Default::default(),
                render_pipelines: Default::default(),
                main_pass: Default::default(),
                mesh: (),
                transform: Default::default(),
                global_transform: Default::default(),
            })
            .current_entity()
            .unwrap();
    */
}
