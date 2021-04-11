//
// Copyright 2021 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use crate::resource::cartographer::faction::FactionKind;
use crate::resource::cartographer::{Faction, FactionId, Orbital, OrbitalId, Sector, TileMap};

pub struct CartographyBuilder {
    factions: Vec<Faction>,
    sectors: Vec<Sector>,
    orbitals: Vec<Orbital>,
    tilemaps: Vec<TileMap>,
}

impl Default for CartographyBuilder {
    fn default() -> Self {
        CartographyBuilder {
            factions: Vec::new(),
            sectors: Vec::new(),
            orbitals: Vec::new(),
            tilemaps: Vec::new(),
        }
    }
}

impl CartographyBuilder {
    pub fn generate_tutorial_sector(&mut self) -> &mut Self {
        let faction_id = FactionId(self.factions.len());
        let faction = Faction {
            id: faction_id,
            code: "EPA".to_string(),
            name: "Earth Port Authority".to_string(),
            description: "".to_string(),
            color: Default::default(),
            kind: FactionKind::Governmental,
            relations: vec![],
        };
        let primary_id = OrbitalId(self.orbitals.len());
        let primary = Orbital {
            id: primary_id,
            name: String::from("Sol"),
            mass: 1.989e30,
            radius: 6.96347e5,
            rotational_period: 0.0,
            orbital_distance: 0.0,
            orbital_period: 0.0,
            orbital_parent: None,
        };
        let planet1_id = OrbitalId(self.orbitals.len());
        let planet1 = Orbital {
            id: planet1_id,
            name: String::from("Mercury"),
            mass: 3.285e23,
            radius: 2.439e3,
            rotational_period: 1407.0,
            orbital_distance: 5.8e7,
            orbital_period: 88.0,
            orbital_parent: Some(primary_id),
        };
        let planet2_id = OrbitalId(self.orbitals.len());
        let planet2 = Orbital {
            id: planet2_id,
            name: String::from("Venus"),
            mass: 4.867e24,
            radius: 6.052e3,
            rotational_period: 2802.0,
            orbital_distance: 1.08e8,
            orbital_period: 5400.0,
            orbital_parent: Some(primary_id),
        };
        let planet3_id = OrbitalId(self.orbitals.len());
        let planet3 = Orbital {
            id: planet3_id,
            name: String::from("Earth"),
            mass: 5.972e24,
            radius: 6.371e3,
            rotational_period: 2802.0,
            orbital_distance: 1.08e8,
            orbital_period: 5400.0,
            orbital_parent: Some(primary_id),
        };
        let planet3a_id = OrbitalId(self.orbitals.len());
        let planet3a = Orbital {
            id: planet3a_id,
            name: String::from("Luna"),
            mass: 5.972e24,
            radius: 6.371e3,
            rotational_period: 2802.0,
            orbital_distance: 1.08e8,
            orbital_period: 5400.0,
            orbital_parent: Some(primary_id),
        };
        self
    }
}
