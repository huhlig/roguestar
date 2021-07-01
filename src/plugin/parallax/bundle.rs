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

use super::render::PARALLAX_BACKGROUND_PIPELINE_HANDLE;
use bevy::asset::Handle;
use bevy::ecs::bundle::Bundle;
use bevy::render::draw::{Draw, Visible};
use bevy::render::mesh::Mesh;
use bevy::render::pipeline::{RenderPipeline, RenderPipelines};
use bevy::render::render_graph::base::MainPass;
use bevy::sprite::{ColorMaterial, QUAD_HANDLE};
use bevy::transform::components::{GlobalTransform, Transform};

#[derive(Bundle, Clone)]
pub struct ParallaxBackgroundBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for ParallaxBackgroundBundle {
    fn default() -> Self {
        Self {
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                PARALLAX_BACKGROUND_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            main_pass: MainPass,
            draw: Default::default(),
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
