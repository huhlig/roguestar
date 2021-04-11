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

use bevy::asset::Handle;
use bevy::audio::Audio;
use bevy::core::{Time, Timer};
use bevy::ecs::{Command, Res};
use bevy::reflect::TypeUuid;
use bevy::sprite::TextureAtlas;
use bevy::utils::Duration;

/// Stategraph
/// A Stateless representation of an Entities possible states.
#[derive(TypeUuid)]
#[uuid = "06ec0d9f-2762-45b9-a393-1aad38334993"]
pub struct StateGraph {
    groups: Vec<StateGroup>,
    states: Vec<State>,
}

impl StateGraph {
    /// Create a new State, This should be wrapped in friendly APIs
    pub fn add_state(
        &mut self,
        name: &str,
        sprite: Option<(Handle<TextureAtlas>, u32)>,
    ) -> StateId {
        let state_id = StateId(self.states.len());
        self.states.push(EntityState {
            id: state_id,
            name: name.to_string(),
            commands,
            exits: vec![],
        });
        state_id
    }
    /// Get current State,
    pub fn get_state(&self, id: StateId) -> Option<&EntityState> {
        self.states.get(id.0)
    }
    pub fn get_state_mut(&mut self, id: StateId) -> Option<&mut EntityState> {
        self.states.get_mut(id.0)
    }
    pub fn to_dot<T: std::io::Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(writer, "digraph state_graph {{");
        for state in &self.states {
            writeln!(writer, "    {} [label=\"{}\"]", state.id.0, state.name);
            for transition in &state.exits {
                writeln!(
                    writer,
                    "    {}->{} [label=\"{}\"]",
                    state.id.0, transition.target.0, state.name
                );
            }
        }
        writeln!(writer, "}}")
    }
}

impl Default for StateGraph {
    fn default() -> Self {
        StateGraph {
            groups: Vec::new(),
            states: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct StateId(usize);

/// Animation State
pub struct State {
    pub(crate) id: StateNodeId,
    pub(crate) name: String,
    pub(crate) group: Option<StateGroupId>,
    pub(crate) texture: Option<(Handle<TextureAtlas>, u32)>,
    pub(crate) sound: Option<Handle<Audio>>,
    pub(crate) transitions: Vec<StateTransition>,
}

/// State Transition
pub struct StateTransition {
    pub(crate) condition: Condition,
    pub(crate) destination: StateId,
}

pub enum Condition {
    Timer(Duration),
}
