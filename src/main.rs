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

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::{LogPlugin, LogSettings};
use bevy::prelude::*;

mod common;
mod mode;
mod plugin;
mod utility;

#[bevy_main]
fn main() {
    println!(
        "Roguestar v{} Copyright (C) 2021 Hans W. Uhlig",
        env!("CARGO_PKG_VERSION")
    );
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .insert_resource(WindowDescriptor {
            title: format!("Roguestar v{}", env!("CARGO_PKG_VERSION")),
            width: 800.,
            height: 600.,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        // Load the Assets
        .add_startup_system(common::asset_loader.system())
        .add_startup_system(common::create_cameras.system())
        .add_startup_system(common::music_player.system())
        // Then go to the main menu
        .add_state(mode::GameState::MainMenu)
        // Main Menu States
        .add_system_set(
            SystemSet::on_enter(mode::GameState::MainMenu)
                .with_system(mode::menu::setup_mainmenu.system()),
        )
        .add_system_set(
            SystemSet::on_update(mode::GameState::MainMenu)
                .with_system(mode::menu::update_mainmenu.system())
                //.with_system(mode::menu::process_gamepad_input.system())
                //.with_system(mode::menu::process_keyboard_input.system())
                //.with_system(mode::menu::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(mode::GameState::MainMenu)
                .with_system(mode::menu::cleanup_mainmenu.system()),
        )
        // WorldGen States
        .add_system_set(
            SystemSet::on_enter(mode::GameState::WorldGen)
                .with_system(mode::worldgen::setup_worldgen.system()),
        )
        .add_system_set(
            SystemSet::on_update(mode::GameState::WorldGen)
                .with_system(mode::worldgen::update_worldgen.system()),
        )
        .add_system_set(
            SystemSet::on_exit(mode::GameState::WorldGen)
                .with_system(mode::worldgen::cleanup_worldgen.system()),
        )
        // Ground Travel State
        .add_system_set(
            SystemSet::on_enter(mode::GameState::TilespaceMode)
                .with_system(mode::tilespace::setup_tilespace.system()),
        )
        .add_system_set(
            SystemSet::on_update(mode::GameState::TilespaceMode)
                .with_system(mode::tilespace::process_gamepad_input.system())
                .with_system(mode::tilespace::process_keyboard_input.system())
                .with_system(mode::tilespace::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(mode::GameState::TilespaceMode)
                .with_system(mode::tilespace::cleanup_tilespace.system()),
        )
        // Sector Flight Travel States
        .add_system_set(
            SystemSet::on_enter(mode::GameState::SectorspaceMode)
                .with_system(mode::sectorspace::setup_sectorspace.system()),
        )
        .add_system_set(
            SystemSet::on_update(mode::GameState::SectorspaceMode)
                .with_system(mode::sectorspace::process_gamepad_input.system())
                .with_system(mode::sectorspace::process_keyboard_input.system())
                .with_system(mode::sectorspace::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(mode::GameState::SectorspaceMode)
                .with_system(mode::sectorspace::setup_sectorspace.system()),
        )
        // Hyperspace Travel State
        .add_system_set(
            SystemSet::on_enter(mode::GameState::HyperspaceMode)
                .with_system(mode::hyperspace::setup_hyperspace.system()),
        )
        .add_system_set(
            SystemSet::on_update(mode::GameState::HyperspaceMode)
                .with_system(mode::hyperspace::process_gamepad_input.system())
                .with_system(mode::hyperspace::process_keyboard_input.system())
                .with_system(mode::hyperspace::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(mode::GameState::HyperspaceMode)
                .with_system(mode::hyperspace::cleanup_hyperspace.system()),
        )
        // Cyberspace Travel State
        .add_system_set(
            SystemSet::on_enter(mode::GameState::CyberspaceMode)
                .with_system(mode::cyberspace::setup_cyberspace.system()),
        )
        .add_system_set(
            SystemSet::on_update(mode::GameState::CyberspaceMode)
                .with_system(mode::cyberspace::process_gamepad_input.system())
                .with_system(mode::cyberspace::process_keyboard_input.system())
                .with_system(mode::cyberspace::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(mode::GameState::CyberspaceMode)
                .with_system(mode::cyberspace::cleanup_cyberspace.system()),
        )
        // Pause Menu States
        .add_system_set(
            SystemSet::on_enter(mode::GameState::PauseMenu)
                .with_system(mode::menu::setup_mainmenu.system()),
        )
        .add_system_set(
            SystemSet::on_update(mode::GameState::PauseMenu)
                .with_system(mode::menu::process_gamepad_input.system())
                .with_system(mode::menu::process_keyboard_input.system())
                .with_system(mode::menu::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(mode::GameState::PauseMenu)
                .with_system(mode::menu::setup_pausemenu.system()),
        )
        .run();
}
