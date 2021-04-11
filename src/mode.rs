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
pub mod cyberspace;
pub mod hyperspace;
pub mod menu;
pub mod sectorspace;
pub mod tilespace;
pub mod worldgen;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    WorldGen,
    TilespaceMode,
    SectorspaceMode,
    HyperspaceMode,
    CyberspaceMode,
    PauseMenu,
}
