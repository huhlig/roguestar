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
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
/*
/// Generate a star system using Gurps Space Rules
pub fn generate_sector(mut commands: Commands, sector_coordinates: IVec2, seed: [u8; 32]) {
    let mut rng = Xoshiro256PlusPlus::from_seed(seed);

    // Sector is the root of a system.

    match rng.roll(3, 6) {
        1...7 => {
            let star0 = generate_primary_star(rng);
        }
        8...15 => {
            let star0 = generate_primary_star(rng);
            let star1 = generate_companion_star(rng);
        }
        16...18 => {
            let star0 = generate_star(rng);
            let star1 = generate_star(rng);
            let star2 = generate_star(rng);
        }
    };

    for orbit in 0..8 {
        let planet = world.create_entity().build();
        for so in 0..8 {
            let moon = world.create_entity().build();
        }
    }
}

/// Taken from Gurps 4E: Space Pg 101
/// TODO: Redo this to include > 2.0 stellar masses
fn generate_primary_stellar_mass(rng: &mut XorShiftRng) -> f32 {
    match (rng.roll(3, 6), rng.roll(3, 6)) {
        (03, 03...10) => 2.00,
        (03, 11...18) => 1.90,
        (04, 03...08) => 1.80,
        (04, 09...11) => 1.70,
        (04, 12...18) => 1.60,
        (05, 03...07) => 1.50,
        (05, 08...10) => 1.45,
        (05, 11...12) => 1.40,
        (05, 13...18) => 1.35,
        (06, 03...07) => 1.30,
        (06, 08...09) => 1.25,
        (06, 10...10) => 1.20,
        (06, 11...12) => 1.15,
        (06, 13...18) => 1.10,
        (07, 03...07) => 1.05,
        (07, 08...09) => 1.00,
        (07, 10...10) => 0.95,
        (07, 11...12) => 0.90,
        (07, 13...18) => 0.85,
        (08, 03...07) => 0.80,
        (08, 08...09) => 0.75,
        (08, 10...10) => 0.70,
        (08, 11...12) => 0.65,
        (08, 13...18) => 0.60,
        (09, 03...08) => 0.55,
        (09, 09...11) => 0.50,
        (09, 12...18) => 0.45,
        (10, 03...08) => 0.40,
        (10, 09...11) => 0.35,
        (10, 12...18) => 0.30,
        (11, 03...18) => 0.25,
        (12, 03...18) => 0.20,
        (13, 03...18) => 0.15,
        (14...18, 03...18) => 0.10,
    }
}

fn generate_companion_stellar_mass(rng: &mut XorShiftRng, primary_mass: f32) -> f32 {
    match rng.roll(1, 6) {
        x => ::std::cmp::max(primary_mass - (rng.roll(x, 6) * 0.1), 0.1),
        0 => primary_mass,
    }
}
*/
