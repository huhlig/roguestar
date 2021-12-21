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

use super::GenerationConfiguration;
use super::GenerationContext;

#[derive(Debug)]
pub struct ProtoFaction {
    name: String,
}

/// Generate Faction
pub fn generate_government_faction(mut context: &GenerationContext) {}

const ADJECTIVES: [&str; 2] = ["Holy", "Inscrutable"];

const TYPES: [&str; 7] = [
    "Cabal",
    "Confederation",
    "Consortium",
    "Federation",
    "Syndicate",
    "Empire",
    "Kingdom",
];
