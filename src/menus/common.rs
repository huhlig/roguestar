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

use bevy::prelude::*;

/// MainMenu Tag Component
pub struct MainMenuTag;

/// Pause Menu Tag
pub struct PauseMenuTag;

/// Actions in Main Menu
pub enum MainMenuAction {
    NewGame,
    LoadGame,
    Settings,
    Credits,
    QuitGame,
}

/// Actions in Pause Menu
pub enum PauseMenuAction {
    Resume,
    LoadGame,
    SaveGame,
    Settings,
    MainMenu,
    QuitGame,
}

/// Button Materials
pub struct MenuButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}
