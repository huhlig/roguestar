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

use bevy::app::EventReader;
use bevy::input::{
    gamepad::{GamepadEvent, GamepadEventType},
    keyboard::KeyboardInput,
    mouse::{MouseButtonInput, MouseMotion, MouseWheel},
};
use bevy::window::CursorMoved;

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
