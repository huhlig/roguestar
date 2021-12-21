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

mod common;
mod input;
mod system;

pub use self::common::{
    MainMenuAction, MainMenuTag, MenuButtonMaterials, PauseMenuAction, PauseMenuTag,
};
pub use self::system::{cleanup_mainmenu, setup_mainmenu, update_mainmenu};
pub use self::system::{cleanup_pausemenu, setup_pausemenu, update_pausemenu};

use crate::GameState;
use bevy::app::{AppBuilder, EventReader};
use bevy::input::{
    gamepad::{GamepadEvent, GamepadEventType},
    keyboard::KeyboardInput,
    mouse::{MouseButtonInput, MouseMotion, MouseWheel},
};
use bevy::prelude::*;
use bevy::window::CursorMoved;

pub fn bootstrap_menus(app: &mut AppBuilder) {
    app.add_startup_system(self::initialize_menus.system());
    app.add_system_set(
        SystemSet::on_enter(GameState::PauseMenu).with_system(self::setup_pausemenu.system()),
    )
    .add_system_set(
        SystemSet::on_update(GameState::PauseMenu).with_system(self::process_input.system()),
    )
    .add_system_set(
        SystemSet::on_exit(GameState::PauseMenu).with_system(self::setup_pausemenu.system()),
    )
    .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_mainmenu.system()))
    .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(update_mainmenu.system()))
    .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(cleanup_mainmenu.system()));
}

/// Gets Called once at initialization.
pub fn initialize_menus(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(MenuButtonMaterials {
        normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
        hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
    });
}

/// Process Gamepad Input
pub fn process_input(
    mut gamepad_event: EventReader<GamepadEvent>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
    for event in gamepad_event.iter() {
        match &event {
            GamepadEvent(gamepad, GamepadEventType::Connected) => {
                println!("{:?} Connected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
                println!("{:?} Disconnected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::ButtonChanged(button_type, value)) => {
                println!("{:?} of {:?} is changed to {}", button_type, gamepad, value);
            }
            GamepadEvent(gamepad, GamepadEventType::AxisChanged(axis_type, value)) => {
                println!("{:?} of {:?} is changed to {}", axis_type, gamepad, value);
            }
        }
    }
    for event in keyboard_input_events.iter() {
        println!("{:?}", event);
    }
    for event in mouse_button_input_events.iter() {
        println!("{:?}", event);
    }
    for event in mouse_motion_events.iter() {
        println!("{:?}", event);
    }
    for event in cursor_moved_events.iter() {
        println!("{:?}", event);
    }
    for event in mouse_wheel_events.iter() {
        println!("{:?}", event);
    }
}
