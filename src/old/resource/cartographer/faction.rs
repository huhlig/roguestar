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

use bevy::render::color::Color;

///
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FactionId(pub(crate) usize);

#[derive(Copy, Clone, Debug)]
pub enum FactionKind {
    Corporate,
    Criminal,
    Governmental,
    Religious,
}

pub enum FactionReach {
    Sector,
    Cluster,
    Empire,
}

///
#[derive(Debug)]
pub struct Faction {
    pub id: FactionId,
    /// Shortened Code for Faction
    pub code: String,
    /// Name
    pub name: String,
    /// Description of the Faction & Lore
    pub description: String,
    /// Color used to distinguish Faction
    pub color: Color,
    /// Kind of Faction
    pub kind: FactionKind,
    /// Faction Relations with those around it.
    pub relations: Vec<Relation>,
}

#[derive(Debug)]
pub struct Relation(FactionId, f32);

const ADJECTIVES: [&str; 1] = ["Holy"];

const TYPES: [&str; 7] = [
    "Cabal",
    "Confederation",
    "Consortium",
    "Federation",
    "Syndicate",
    "Empire",
    "Kingdom",
];
