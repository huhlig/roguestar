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
use bevy::render::camera::Camera;

#[derive(Bundle, Default)]
pub struct ParallaxBundle {
    pub layer: ParallaxLayer,
    pub transform: Transform,
    pub global: GlobalTransform,
    pub children: Children,
    pub material: Handle<ColorMaterial>,
    pub sprite: Sprite,
}

#[derive(Default, Debug)]
pub struct ParallaxLayer {
    pub speed_x: f32,
    pub speed_y: f32,
}

/// Matches the layer to the camera.
/// Note the layer is offset to the left by half the window to make
pub fn layer_movement_system(
    windows: Res<Windows>,
    cameras: Query<(&Transform, &Camera, &Children)>,
    mut layers: Query<(&ParallaxLayer, &Sprite, &mut Transform)>,
) -> () {
    for (transform, camera, children) in cameras.iter() {
        let window = windows.get(camera.window)
            .expect("Window attached to camera is missing");
        let window_size = Vec2::new(window.width(), window.height());
        let camera_position = transform.translation;
        for child in children.iter() {
            if let Ok((layer, sprite, mut trans)) = layers.get_mut(*child) {
                move_layer_position(&window_size, &camera_position, sprite, layer, &mut trans);
            }
        }
    }
}

/// Responsible for setting the positioning of the sprites
pub fn children_layout_system(
    layers: Query<(&Sprite, &Children), With<ParallaxLayer>>,
    mut sprites: Query<&mut Transform>,
) {
    for (sprite, children) in layers.iter() {
        for (index, child) in children.iter().enumerate() {
            if let Ok(mut transform) = sprites.get_component_mut::<Transform>(*child) {
                let scaled_sprite = scaled_sprite(sprite, &transform);
                transform.translation.x = index as f32 * scaled_sprite.x;
                transform.translation.y = index as f32 * scaled_sprite.y;
                transform.translation.z = -999.0;
            }
        }
    }
}

/// Mutates the layer based on the camera position
/// this allows us to have the parallax effect by having the layers move at different rates
/// once we move past the width of the sprite, it resets to 0
fn move_layer_position(
    window: &Vec2,
    position: &Vec3,
    sprite: &Sprite,
    layer: &ParallaxLayer,
    transform: &mut Transform,
) -> () {
    let edge_offset = camera_edge_offset(&window);
    let camera_offset = camera_sprite_offset(position, layer, sprite, transform);
    transform.translation = Vec3::from((edge_offset + camera_offset, 0.0));
}

/// Gets the 'screen' width of the sprite.
/// This takes into account the scaling
fn scaled_sprite(sprite: &Sprite, transform: &Transform) -> Vec2 {
    Vec2::new(sprite.size[0] * transform.scale.x, sprite.size[1] * transform.scale.y)
}

/// Calculate the amount of sprites we need for the effect
fn desired_child_count(window: &Vec2, sprite: &Sprite, transform: &Transform) -> UVec2 {
    let scaled_sprite = scaled_sprite(sprite, transform);
    UVec2::new(
        if scaled_sprite.x > 0.0 { window.width.div_euclid(scaled_sprite.x) + 2 } else { 0 },
        if scaled_sprite.y > 0.0 { window.width.div_euclid(scaled_sprite.y) + 2 } else { 0 },
    )
}

/// Caculates an offset to put the layer at the bottom left edge of the 'container'
/// This is because the camera seems to center on (0.0, 0.0)
fn camera_edge_offset(window: &Vec2) -> Vec2 {
    Vec2::new(0.0 - window.width as f32 / 2.0, 0.0 - window.height as f32 / 2.0)
}

/// How far to offset the layer due to the camera position
/// Will be clamped by the sprite offset
fn camera_sprite_offset(
    position: &Vec3,
    layer: &ParallaxLayer,
    sprite: &Sprite,
    transform: &Transform,
) -> Vec2 {
    let scaled_sprite = scaled_sprite(sprite, transform);
    Vec2::new(
        -(position.x() * layer.speed_x).rem_euclid(scaled_sprite.x),
        -(position.y() * layer.speed_y).rem_euclid(scaled_sprite.y),
    )
}

/// Manages the amount of child sprites we need to repeat
/// Based on the windows size
pub fn children_count_system(
    mut commands: Commands,
    cameras_query: Query<(&Camera, &Vec2, &Children)>,
    mut layer_query: Query<(
        Entity,
        &Parent,
        &Children,
        &Sprite,
        &Handle<ColorMaterial>,
        &Transform,
        With<ParallaxLayer>,
    )>,
) -> () {
    for (entity, parent, children, sprite, material, transform) in layer_query.iter_mut() {
        if let Ok(window) = cameras_query.get_component(parent.0) {
            let desired_children = desired_child_count(&window, &sprite, &transform);
            let current_children = children.len();
            let to_add = desired_children as usize - current_children;

            for _ in 0..to_add {
                commands
                    .entity(entity)
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(SpriteBundle {
                                material: material.clone(),
                                sprite: Sprite::default(),
                                ..Default::default()
                            });
                    });
            }

            //TODO: remove sprites if they aren't needed
        }
    }
}



