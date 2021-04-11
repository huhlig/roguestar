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

use crate::mode::menu::MenuButtonMaterials;
use bevy::prelude::*;

pub fn asset_loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(MenuButtonMaterials {
        normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
        hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
    });
    let title_font: Handle<Font> = asset_server.load("fonts/dystopian-future.ttf");
    let alien_font_1: Handle<Font> = asset_server.load("fonts/epyval.ttf");
    let alien_font_2: Handle<Font> = asset_server.load("fonts/mercy.ttf");
    let alien_font_3: Handle<Font> = asset_server.load("fonts/novasquare.ttf");
    let alien_font_4: Handle<Font> = asset_server.load("fonts/spaceage.ttf");
    let alien_font_4: Handle<Font> = asset_server.load("fonts/spacemono.ttf");

    let title_bg_material_handle_1 =
        materials.add(asset_server.load("backgrounds/title/bg1.png").into());
    let title_bg_material_handle_2 =
        materials.add(asset_server.load("backgrounds/title/bg2.png").into());
    let title_bg_material_handle_3 =
        materials.add(asset_server.load("backgrounds/title/bg3.png").into());
    let title_bg_material_handle_4 =
        materials.add(asset_server.load("backgrounds/title/bg4.png").into());
    let title_bg_material_handle_5 =
        materials.add(asset_server.load("backgrounds/title/bg5.png").into());
    let title_bg_material_handle_6 =
        materials.add(asset_server.load("backgrounds/title/bg6.png").into());
}
