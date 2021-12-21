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

use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::Camera;

use super::Hyperspace;
use crate::engine::PlayerAvatar;

const ZOOM_SCALE: f32 = 0.9;
const MOVE_SCALE: f32 = 1.0;

pub fn process_keyboard_input(
    mut query: Query<(&PlayerAvatar, &mut Transform), With<Hyperspace>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    for event in keyboard_input_events.iter() {
        if event.state.is_pressed() {
            match event.key_code {
                Some(KeyCode::Minus) => {
                    for (camera, mut transform) in query.iter_mut() {
                        transform.scale /= Vec3::new(ZOOM_SCALE, ZOOM_SCALE, 1.0);
                        trace!("Zoom: {:?}", transform.scale);
                    }
                }
                Some(KeyCode::Equals) => {
                    for (camera, mut transform) in query.iter_mut() {
                        transform.scale *= Vec3::new(ZOOM_SCALE, ZOOM_SCALE, 1.0);
                        trace!("Zoom: {:?}", transform.scale);
                    }
                }
                Some(KeyCode::A) | Some(KeyCode::Left) => {
                    for (camera, mut transform) in query.iter_mut() {
                        transform.translation -= Vec3::new(MOVE_SCALE, 0.0, 0.0);
                        trace!("Pos: {:?}", transform.translation);
                    }
                }
                Some(KeyCode::D) | Some(KeyCode::Right) => {
                    for (camera, mut transform) in query.iter_mut() {
                        transform.translation += Vec3::new(MOVE_SCALE, 0.0, 0.0);
                        trace!("Pos: {:?}", transform.translation);
                    }
                }
                Some(KeyCode::W) | Some(KeyCode::Up) => {
                    for (camera, mut transform) in query.iter_mut() {
                        transform.translation += Vec3::new(0.0, MOVE_SCALE, 0.0);
                        trace!("Pos: {:?}", transform.translation);
                    }
                }
                Some(KeyCode::S) | Some(KeyCode::Down) => {
                    for (camera, mut transform) in query.iter_mut() {
                        transform.translation -= Vec3::new(0.0, MOVE_SCALE, 0.0);
                        trace!("Pos: {:?}", transform.translation);
                    }
                }
                _ => {}
            }
        }
        //println!("{:?}", event);
    }
}

pub fn process_mouse_input(
    mut query: Query<&Camera, With<Hyperspace>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
    for event in mouse_button_input_events.iter() {
        //println!("{:?}", event);
    }
    for event in mouse_motion_events.iter() {
        //println!("{:?}", event);
    }
    for event in cursor_moved_events.iter() {
        //println!("{:?}", event);
    }
    for event in mouse_wheel_events.iter() {
        //println!("{:?}", event);
    }
}

pub fn process_gamepad_input(mut gamepad_event: EventReader<GamepadEvent>) {
    for event in gamepad_event.iter() {
        match &event {
            GamepadEvent(gamepad, GamepadEventType::Connected) => {
                //println!("{:?} Connected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
                //println!("{:?} Disconnected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::ButtonChanged(button_type, value)) => {
                //println!("{:?} of {:?} is changed to {}", button_type, gamepad, value);
            }
            GamepadEvent(gamepad, GamepadEventType::AxisChanged(axis_type, value)) => {
                //println!("{:?} of {:?} is changed to {}", axis_type, gamepad, value);
            }
        }
    }
}
