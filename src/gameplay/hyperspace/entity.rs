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
use hexgrid::Coordinate;

/// Tag Denoting Hyperspace Object
pub struct Hyperspace;

/// Info about Hyperspace Gravity Well
pub struct HyperspaceSectorInfo {
    /// Integer Sector Location
    pub location: Coordinate<i32>,
    /// Cartesian Position
    pub position: Vec2,
    /// Primary Radius
    pub primary_radius: f32,
    /// Primary Color
    pub primary_color: Color,
}
