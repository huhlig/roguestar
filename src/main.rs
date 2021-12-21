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

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
// TODO: Remove These
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

//use crate::plugin::parallax::ParallaxBackgroundPlugin;
use self::generation::GenerationContext;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::{LogPlugin, LogSettings};
use bevy::prelude::*;
use bevy_tilemap::prelude::*;
use rand::Rng;

mod cartographer;
mod gameplay;
mod generation;
mod menus;
mod plugin;
mod utility;

/// State of Game Engine
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    WorldGeneration,
    GameplayMode,
    PauseMenu,
}

#[bevy_main]
fn main() {
    println!(
        "Roguestar v{} Copyright (C) 2021 Hans W. Uhlig",
        env!("CARGO_PKG_VERSION")
    );
    let mut builder = App::build();
    builder
        .add_plugins(DefaultPlugins)
        //.add_plugin(ParallaxBackgroundPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugins(TilemapDefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: format!("Roguestar v{}", env!("CARGO_PKG_VERSION")),
            width: 800.,
            height: 600.,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_state(GameState::MainMenu);

    // Pause Menu States
    self::menus::bootstrap_menus(&mut builder);
    self::generation::bootstrap_worldgeneration(&mut builder);

    builder.run();
}
