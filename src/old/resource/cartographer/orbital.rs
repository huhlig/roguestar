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

/// Identifier for an Astronomical Object
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OrbitalId(pub(crate) usize);

/// Astronomical Object in Realspace
#[derive(Debug)]
pub struct Orbital {
    pub id: OrbitalId,
    /// Name of Orbital Body
    pub name: String,
    /// Mass of Orbital Body in kg
    pub mass: f32,
    /// Radius of Orbital Body in km
    pub radius: f32,
    /// Rotational Period of Orbital Body in hours
    pub rotational_period: f32,
    /// Orbital Distance from Parent
    pub orbital_distance: f32,
    /// Orbital Period in hours
    pub orbital_period: f32,
    /// Parent Body Orbits
    pub orbital_parent: Option<OrbitalId>,
}
