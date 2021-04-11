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

use super::{Faction, FactionId, Orbital, OrbitalId, Sector, SectorId, TileMap, TileMapId};
use bevy::math::Vec2;
use rstar::primitives::PointWithData;
use rstar::{RTree, AABB};

/// Resource Container for Stellar geography information
pub struct Cartographer {
    factions: Vec<Faction>,
    hypermap: RTree<PointWithData<SectorId, [f32; 2]>>,
    sectors: Vec<Sector>,
    orbitals: Vec<Orbital>,
    tilemaps: Vec<TileMap>,
}

impl Cartographer {
    pub fn new(
        factions: Vec<Faction>,
        sectors: Vec<Sector>,
        orbitals: Vec<Orbital>,
        tilemaps: Vec<TileMap>,
    ) -> Cartographer {
        let hypermap = RTree::bulk_load(
            sectors
                .iter()
                .enumerate()
                .map(|data| {
                    PointWithData::new(SectorId(data.0), [data.1.coords.x, data.1.coords.y])
                })
                .collect(),
        );
        Cartographer {
            factions,
            hypermap,
            sectors,
            orbitals,
            tilemaps,
        }
    }
    pub fn get_faction_by_id(&self, id: &FactionId) -> Option<&Faction> {
        self.factions.get(id.0)
    }
    pub fn get_nearby_sectors(&self, point: Vec2, radius: f32) {
        let iter = self
            .hypermap
            .locate_in_envelope(&AABB::from_corners(
                [point.x - radius, point.y - radius],
                [point.x + radius, point.y + radius],
            ))
            .map(|a| self.sectors.get(a.data.0));
    }
    pub fn get_sector_by_id(&self, id: &SectorId) -> Option<&Sector> {
        self.sectors.get(id.0)
    }
    pub fn get_orbital_by_id(&self, id: &OrbitalId) -> Option<&Orbital> {
        self.orbitals.get(id.0)
    }
    pub fn get_orbital_position(&self, id: OrbitalId, game_time: f32) -> Option<Vec2> {
        let orbital = self.orbitals.get(id.0)?;
        let parent_position = if let Some(parent_id) = orbital.orbital_parent {
            self.get_orbital_position(parent_id, game_time)?
        } else {
            Vec2::new(0.0, 0.0)
        };
        let start = parent_position + Vec2::new(orbital.orbital_distance, 0.0);
        let theta = game_time / orbital.orbital_period;
        let position = Vec2::new(
            start.x * theta.cos() - start.y * theta.sin(),
            start.x * theta.sin() + start.y * theta.cos(),
        );
        Some(position)
    }
}

impl Default for Cartographer {
    fn default() -> Cartographer {
        Cartographer {
            factions: Vec::new(),
            hypermap: RTree::default(),
            sectors: Vec::new(),
            orbitals: Vec::new(),
            tilemaps: Vec::new(),
        }
    }
}
