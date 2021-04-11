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

use super::MenuButtonMaterials;
use crate::mode::GameState;
use bevy::prelude::*;

/// MainMenu Tag Component
pub struct MainMenu;

pub enum MainMenuAction {
    NewGame,
    LoadGame,
    Settings,
    Credits,
    QuitGame,
}

pub fn setup_mainmenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<MenuButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    trace!("Setup MainMenu");
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
        .insert(MainMenu)
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
                .insert(MainMenu)
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
                        .insert(MainMenu);
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
                        .insert(MainMenu)
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
                                .insert(MainMenu)
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
                                        .insert(MainMenu);
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
                                .insert(MainMenu)
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
                                        .insert(MainMenu);
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
                                .insert(MainMenu)
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
                                        .insert(MainMenu);
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
                                .insert(MainMenu)
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
                                        .insert(MainMenu);
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
                                .insert(MainMenu)
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
                                        .insert(MainMenu);
                                });
                        });
                });
        });
}

pub fn cleanup_mainmenu(mut commands: Commands, mut query: Query<Entity, With<MainMenu>>) {
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
                    MainMenuAction::NewGame => state.set(GameState::WorldGen).unwrap(),
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
