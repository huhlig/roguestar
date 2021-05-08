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

/// Sector Radius
pub const SECTOR_RADIUS: f32 = 20.0;

/// User Configurable Generation Parameters
pub struct Parameters {
    /// Master Seed used by all generation
    pub seed: u64,
    /// Likelyhood of a primary existing in a sector.
    pub sector_density: f32,
    /// Average Density of Resources in the Galaxy.
    pub resource_density: f32,
}
