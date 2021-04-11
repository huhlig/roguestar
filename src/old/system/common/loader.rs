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

use bevy::asset::{AssetServer, Assets, Handle};
use bevy::ecs::{Res, ResMut};
use bevy::render::texture::Texture;
use bevy::sprite::{ColorMaterial, TextureAtlas};
use bevy::utils::HashMap;

pub struct SpriteCatalog(HashMap<String, SpriteCollection>);
pub struct SpriteCollection(HashMap<String, SpriteAnimation>);
pub struct SpriteAnimation(HashMap<String, SpriteAnimation>);
pub struct SpriteFrame {
    texture_atlas: Handle<TextureAtlas>,
    texture_sprite: Handle<Texture>,
}

fn sprite_loader_system(
    catalog: Res<SpriteCatalog>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //asset_server.load_folder("");
}
