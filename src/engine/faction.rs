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
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

/// Faction Reference Table
///
#[derive(Serialize, Deserialize, TypeUuid)]
#[uuid = "8830af02-da6c-4a26-8308-8cac59594a95"]
pub struct Factions(HashMap<String, FactionId>, Vec<Faction>);

impl Factions {
    /// Add new Faction
    pub fn create_faction(
        &mut self,
        name: &str,
        kind: FactionKind,
        politics: PoliticalAxis,
    ) -> FactionId {
        let id = FactionId(self.1.len());
        let relations = HashMap::default();
        self.1.push(Faction {
            id,
            name: name.to_owned(),
            kind,
            relations,
            politics,
        });
        self.0.insert(name.to_owned(), id);
        id
    }
    /// Get Faction by FactionId
    pub fn get_by_id(&self, id: FactionId) -> Option<&Faction> {
        self.1.get(id.0)
    }
    /// Get Faction by Faction Label
    pub fn get_by_label(&self, label: &str) -> Option<&Faction> {
        self.1.get(self.0.get(label)?.0)
    }
}

impl Default for Factions {
    fn default() -> Factions {
        let mut factions = Factions(HashMap::default(), Vec::default());
        factions.create_faction(
            "Player",
            FactionKind::Player,
            PoliticalAxis {
                economic: 0.0,
                civil: 0.0,
                diplomatic: 0.0,
                social: 0.0,
            },
        );
        factions
    }
}

/// Faction Id
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct FactionId(usize);

/// Faction Data
#[derive(Debug, Serialize, Deserialize)]
pub struct Faction {
    /// Faction Id
    pub id: FactionId,
    /// Faction Name
    pub name: String,
    /// Type of Faction
    pub kind: FactionKind,
    /// Relationships with other Factions
    pub relations: HashMap<FactionId, f32>,
    /// Faction Political Axis
    pub politics: PoliticalAxis,
}

/// Kind of Faction
#[derive(Debug, Serialize, Deserialize)]
pub enum FactionKind {
    /// Corporate Factions focus on achieving specific goals.
    Corporate,
    /// Criminal Factions operate like Corporate factions but ignore laws.
    Criminal,
    /// Governments focus on population management and providing civil services.
    Government,
    /// Noble Houses are hereditary lineages.
    House,
    /// Guilds are cross cutting groups focused on achieving specific goals.
    Guild,
    /// The Player Faction.
    Player,
    /// Rebellions are organized bodies working against the Governmental faction.
    Rebellion,
    /// Pirates are chaotic Outlaws with no allegiance to anyone but themselves.
    Pirate,
    /// Religious Factions govern a body of people with spirituality and often operate as a shadow government.
    Religious,
}

/// Faction Axis
///
/// https://8values.github.io/
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PoliticalAxis {
    /// Economic Axis
    ///
    /// Is the economy driven by needss or profit.
    ///
    /// * -1.0 - Socialism
    /// *  0.0 - Economic Centrism
    /// * +1.0 - Capitalism
    economic: f32,
    /// Civil Axis
    ///
    /// How free are people to do as they wish.
    ///
    /// * -1.0 - Libertarianism
    /// *  0.0 - Civil Centrism
    /// * +1.0 - Authoritarianism
    civil: f32,
    /// Social Axis
    ///
    /// * -1.0 - Traditionalism
    /// *  0.0 - Social Centrism
    /// * +1.0 - Progressivism
    ///
    social: f32,
    /// Diplomatic Axis
    ///
    /// * -1.0 - Globalism
    /// *  0.0 - Diplomatic Centrism
    /// * +1.0 - Nationalism
    diplomatic: f32,
}