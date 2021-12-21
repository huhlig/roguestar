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

/// SectorSpace Tag with Sector Hex Location
pub struct SectorSpace(pub Coordinate<i32>);

/// Star Information
pub struct StarInfo {
    /// Name of Star
    pub name: String,
    /// Mass of Primary
    pub mass: f32,
    /// How Bright is the Primary
    pub luminosity: f32,
    /// Primary Radius
    pub radius: f32,
    /// Temperature in Kelvin
    pub temp: f32,
    /// Primary Color
    pub color: Color,
}

/// Planet Information
pub struct PlanetInfo {
    /// Name of Planet
    pub name: String,
    /// Radius of Planet
    pub radius: f32,
    /// Temperature of Planet
    pub temperature: f32,
    /// Foliage on planet
    pub foliage: f32,
    /// Minerals on Planet
    pub minerals: f32,
    /// Water On Planet
    pub water: f32,
    /// Gasses on planet
    pub gases: f32,
    /// Population on planet
    pub population: f32,
    /// Does planet have rings
    pub ring: bool,
}

/// Asteroid Information
pub struct AsteroidInfo {
    /// Radius of Asteroid
    pub radius: f32,
}

/// Space Station Information
pub struct StationInfo {
    /// Name of Station
    pub name: String,
}

/// StarShip Information
pub struct ShipInfo {
    /// Name of Ship
    pub name: String,
}
