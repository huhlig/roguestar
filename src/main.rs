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

use crate::plugin::parallax::ParallaxBackgroundPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::{LogPlugin, LogSettings};
use bevy::prelude::*;

mod common;
mod engine;
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
        .add_plugin(ParallaxBackgroundPlugin)
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
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        // Load the Assets
        .add_startup_system(engine::mainmenu::initialize_mainmenu.system())
        .add_startup_system(engine::hyperspace::initialize_hyperspace.system())
        .add_startup_system(engine::sectorspace::initialize_sectorspace.system())
        .add_startup_system(engine::tilespace::initialize_tilespace.system())
        .add_startup_system(engine::cyberspace::initialize_cyberspace.system())
        .add_startup_system(engine::pausemenu::initialize_pausemenu.system())
        // Then go to the main mainmenu
        .add_state(engine::GameState::MainMenu)
        // Main Menu States
        .add_system_set(
            SystemSet::on_enter(engine::GameState::MainMenu)
                .with_system(engine::mainmenu::setup_mainmenu.system()),
        )
        .add_system_set(
            SystemSet::on_update(engine::GameState::MainMenu)
                .with_system(engine::mainmenu::update_mainmenu.system())
                //.with_system(engine::mainmenu::process_gamepad_input.system())
                //.with_system(engine::mainmenu::process_keyboard_input.system())
                //.with_system(engine::mainmenu::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(engine::GameState::MainMenu)
                .with_system(engine::mainmenu::cleanup_mainmenu.system()),
        )
        // WorldGen States
        .add_system_set(
            SystemSet::on_enter(engine::GameState::WorldGen)
                .with_system(engine::worldgen::setup_worldgen.system()),
        )
        .add_system_set(
            SystemSet::on_update(engine::GameState::WorldGen)
                .with_system(engine::worldgen::update_worldgen.system()),
        )
        .add_system_set(
            SystemSet::on_exit(engine::GameState::WorldGen)
                .with_system(engine::worldgen::cleanup_worldgen.system()),
        )
        // Ground Travel State
        .add_system_set(
            SystemSet::on_enter(engine::GameState::TilespaceMode)
                .with_system(engine::tilespace::setup_tilespace.system()),
        )
        .add_system_set(
            SystemSet::on_update(engine::GameState::TilespaceMode)
                .with_system(engine::tilespace::process_gamepad_input.system())
                .with_system(engine::tilespace::process_keyboard_input.system())
                .with_system(engine::tilespace::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(engine::GameState::TilespaceMode)
                .with_system(engine::tilespace::cleanup_tilespace.system()),
        )
        // Sector Flight Travel States
        .add_system_set(
            SystemSet::on_enter(engine::GameState::SectorspaceMode)
                .with_system(engine::sectorspace::setup_sectorspace.system()),
        )
        .add_system_set(
            SystemSet::on_update(engine::GameState::SectorspaceMode)
                .with_system(engine::sectorspace::process_gamepad_input.system())
                .with_system(engine::sectorspace::process_keyboard_input.system())
                .with_system(engine::sectorspace::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(engine::GameState::SectorspaceMode)
                .with_system(engine::sectorspace::setup_sectorspace.system()),
        )
        // Hyperspace Travel State
        .add_system_set(
            SystemSet::on_enter(engine::GameState::HyperspaceMode)
                .with_system(engine::hyperspace::setup_hyperspace.system()),
        )
        .add_system_set(
            SystemSet::on_update(engine::GameState::HyperspaceMode)
                .with_system(engine::hyperspace::process_gamepad_input.system())
                .with_system(engine::hyperspace::process_keyboard_input.system())
                .with_system(engine::hyperspace::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(engine::GameState::HyperspaceMode)
                .with_system(engine::hyperspace::cleanup_hyperspace.system()),
        )
        // Cyberspace Travel State
        .add_system_set(
            SystemSet::on_enter(engine::GameState::CyberspaceMode)
                .with_system(engine::cyberspace::setup_cyberspace.system()),
        )
        .add_system_set(
            SystemSet::on_update(engine::GameState::CyberspaceMode)
                .with_system(engine::cyberspace::process_gamepad_input.system())
                .with_system(engine::cyberspace::process_keyboard_input.system())
                .with_system(engine::cyberspace::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(engine::GameState::CyberspaceMode)
                .with_system(engine::cyberspace::cleanup_cyberspace.system()),
        )
        // Pause Menu States
        .add_system_set(
            SystemSet::on_enter(engine::GameState::PauseMenu)
                .with_system(engine::pausemenu::setup_pausemenu.system()),
        )
        .add_system_set(
            SystemSet::on_update(engine::GameState::PauseMenu)
                .with_system(engine::pausemenu::process_gamepad_input.system())
                .with_system(engine::pausemenu::process_keyboard_input.system())
                .with_system(engine::pausemenu::process_mouse_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(engine::GameState::PauseMenu)
                .with_system(engine::pausemenu::setup_pausemenu.system()),
        )
        .run();
}
