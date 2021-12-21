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

use super::WorldClock;
use bevy::prelude::*;
use std::ops::Rem;

/// Orbital Parent
pub struct OrbitalParent(pub Entity);

impl From<Entity> for OrbitalParent {
    fn from(parent: Entity) -> Self {
        OrbitalParent(parent)
    }
}

/// Orbital Children
pub struct OrbitalChildren(pub Vec<Entity>);

impl From<Vec<Entity>> for OrbitalChildren {
    fn from(children: Vec<Entity>) -> Self {
        OrbitalChildren(children)
    }
}

/// Orbital Parameters
pub struct OrbitalParameters {
    /// Distance in kilometers object orbits from it's parent.
    pub distance: f32,
    /// Duration in seconds it takes for a body to complete one full orbit (A Year)
    pub period: f32,
}

/// Orbiral Position
pub struct OrbitalPosition {
    /// Current Orbital Position
    pub rotation: f32,
}

impl Default for OrbitalPosition {
    fn default() -> OrbitalPosition {
        OrbitalPosition {
            rotation: 0.0,
        }
    }
}

pub fn orbital_update_system(
    world_clock: Res<WorldClock>,
    mut root_query: Query<
        (
            Entity,
            Option<&OrbitalChildren>,
            &OrbitalParameters,
            &mut Transform,
        ),
        Without<OrbitalParent>,
    >,
    mut transform_query: Query<
        (&OrbitalParameters, &mut OrbitalPosition, &mut Transform),
        With<OrbitalParent>,
    >,
    children_query: Query<Option<&OrbitalChildren>, (With<OrbitalParent>, With<Transform>)>,
) {
    for (entity, children, parameters, mut transform) in root_query.iter_mut() {
        if let Some(children) = children {
            for child in children.0.iter() {
                recursive_orbital_update(
                    &world_clock,
                    &transform,
                    &mut transform_query,
                    &children_query,
                    *child,
                );
            }
        }
    }
}

fn recursive_orbital_update(
    world_clock: &Res<WorldClock>,
    parent_transform: &Transform,
    transform_query: &mut Query<
        (&OrbitalParameters, &mut OrbitalPosition, &mut Transform),
        With<OrbitalParent>,
    >,
    children_query: &Query<Option<&OrbitalChildren>, (With<OrbitalParent>, With<Transform>)>,
    entity: Entity,
) {
    let global_matrix = {
        if let Ok((parameters, mut position, mut entity_transform)) =
        transform_query.get_mut(entity)
        {
            position.rotation =
                ((world_clock.seconds_since_epoch() % parameters.period as f64) * 360.0) as f32;
            let mut orbital_transform = Transform::identity();
            orbital_transform.translation = Vec3::new(0.0, parameters.distance, 0.0);
            orbital_transform.rotation =
                Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), position.rotation);
            *entity_transform = parent_transform.mul_transform(orbital_transform);
            *entity_transform
        } else {
            return;
        }
    };

    if let Ok(Some(children)) = children_query.get(entity) {
        for child in children.0.iter() {
            recursive_orbital_update(
                world_clock,
                &global_matrix,
                transform_query,
                children_query,
                *child,
            );
        }
    }
}
