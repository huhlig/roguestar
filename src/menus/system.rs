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

use super::{MainMenuAction, MainMenuTag, MenuButtonMaterials, PauseMenuAction, PauseMenuTag};
use crate::GameState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::prelude::*;

/// Gets called whenever mode started
pub fn setup_pausemenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<MenuButtonMaterials>,
) {
    // Pause Menu UI camera
    commands
        .spawn()
        .insert(PauseMenuTag)
        .insert_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .insert(PauseMenuTag)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .insert(PauseMenuTag)
                .insert(PauseMenuAction::Resume)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Resume Game",
                                TextStyle {
                                    font: asset_server.load("fonts/epyval.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(PauseMenuTag);
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .insert(PauseMenuTag)
                .insert(PauseMenuAction::SaveGame)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Save Game",
                                TextStyle {
                                    font: asset_server.load("fonts/epyval.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(PauseMenuTag);
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .insert(PauseMenuTag)
                .insert(PauseMenuAction::LoadGame)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Load Game",
                                TextStyle {
                                    font: asset_server.load("fonts/epyval.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(PauseMenuTag);
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .insert(PauseMenuTag)
                .insert(PauseMenuAction::Settings)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Settings",
                                TextStyle {
                                    font: asset_server.load("fonts/epyval.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(PauseMenuTag);
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .insert(PauseMenuTag)
                .insert(PauseMenuAction::MainMenu)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Main Menu",
                                TextStyle {
                                    font: asset_server.load("fonts/epyval.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(PauseMenuTag);
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .insert(PauseMenuTag)
                .insert(PauseMenuAction::QuitGame)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Quit Game",
                                TextStyle {
                                    font: asset_server.load("fonts/epyval.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(PauseMenuTag);
                });
        });
}

pub fn update_pausemenu(
    button_materials: Res<MenuButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &PauseMenuAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material, action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                match action {
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

/// Cleanup PauseMenu after closing
pub fn cleanup_pausemenu(mut commands: Commands, mut query: Query<&Entity, With<PauseMenuTag>>) {
    for entity in query.iter() {
        commands.entity(*entity).despawn_recursive();
    }
}

/// Called Each time the main menu state is entered.
pub fn setup_mainmenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<MenuButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    trace!("Setup MainMenu");
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        // Spawn Whole Screen Node
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            material: materials.add(asset_server.load("backgrounds/BaseBlue.png").into()),
            ..Default::default()
        })
        .insert(MainMenuTag)
        .with_children(|parent| {
            parent
                // Center Area with Logo and area for buttons
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Percent(60.0)),
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgba_u8(255, 0, 0, 100).into()),
                    ..Default::default()
                })
                .insert(MainMenuTag)
                .with_children(|parent| {
                    parent
                        // Spawn Logo at Top of Buttons
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "ROGUE STAR",
                                TextStyle {
                                    font: asset_server.load("fonts/dystopian-future.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(MainMenuTag);
                    parent
                        // Spawn Container for Buttons
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                align_content: AlignContent::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::ColumnReverse,
                                ..Default::default()
                            },
                            material: materials.add(Color::rgba_u8(66, 80, 80, 200).into()),
                            ..Default::default()
                        })
                        .insert(MainMenuTag)
                        .with_children(|parent| {
                            parent
                                // Spawn first Button
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                        align_content: AlignContent::Center,
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    material: materials.add(Color::rgba_u8(66, 80, 80, 200).into()),
                                    ..Default::default()
                                })
                                .insert(MainMenuTag)
                                .insert(MainMenuAction::NewGame)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "New Game",
                                                TextStyle {
                                                    font: asset_server.load("fonts/epyval.ttf"),
                                                    font_size: 40.0,
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        })
                                        .insert(MainMenuTag);
                                });
                            parent
                                // Spawn second Button
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                        align_content: AlignContent::Center,
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    material: materials.add(Color::rgba_u8(66, 80, 80, 200).into()),
                                    ..Default::default()
                                })
                                .insert(MainMenuTag)
                                .insert(MainMenuAction::LoadGame)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "Load Game",
                                                TextStyle {
                                                    font: asset_server.load("fonts/epyval.ttf"),
                                                    font_size: 40.0,
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        })
                                        .insert(MainMenuTag);
                                });
                            parent
                                // Spawn second Button
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                        align_content: AlignContent::Center,
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    material: materials.add(Color::rgba_u8(66, 80, 80, 200).into()),
                                    ..Default::default()
                                })
                                .insert(MainMenuTag)
                                .insert(MainMenuAction::Settings)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "Settings",
                                                TextStyle {
                                                    font: asset_server.load("fonts/epyval.ttf"),
                                                    font_size: 40.0,
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        })
                                        .insert(MainMenuTag);
                                });
                            parent
                                // Spawn second Button
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                        align_content: AlignContent::Center,
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    material: materials.add(Color::rgba_u8(66, 80, 80, 200).into()),
                                    ..Default::default()
                                })
                                .insert(MainMenuTag)
                                .insert(MainMenuAction::Credits)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "Credits",
                                                TextStyle {
                                                    font: asset_server.load("fonts/epyval.ttf"),
                                                    font_size: 40.0,
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        })
                                        .insert(MainMenuTag);
                                });
                            parent
                                // Spawn second Button
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                                        align_content: AlignContent::Center,
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    material: materials.add(Color::rgba_u8(66, 80, 80, 200).into()),
                                    ..Default::default()
                                })
                                .insert(MainMenuTag)
                                .insert(MainMenuAction::QuitGame)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                "Quit Game",
                                                TextStyle {
                                                    font: asset_server.load("fonts/epyval.ttf"),
                                                    font_size: 40.0,
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        })
                                        .insert(MainMenuTag);
                                });
                        });
                });
        });
}

pub fn cleanup_mainmenu(mut commands: Commands, mut query: Query<Entity, With<MainMenuTag>>) {
    trace!("Cleanup MainMenu");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_mainmenu(
    mut state: ResMut<State<GameState>>,
    button_materials: Res<MenuButtonMaterials>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Handle<ColorMaterial>,
            &Children,
            &MainMenuAction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material, children, action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                match action {
                    MainMenuAction::NewGame => state.set(GameState::WorldGeneration).unwrap(),
                    MainMenuAction::LoadGame => {}
                    MainMenuAction::Settings => {}
                    MainMenuAction::Credits => {}
                    MainMenuAction::QuitGame => {}
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

/// Process Gamepad Input
pub fn process_gamepad_input(mut gamepad_event: EventReader<GamepadEvent>) {
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
}

pub fn process_keyboard_input(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        println!("{:?}", event);
    }
}

pub fn process_mouse_input(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
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
