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

use super::render::{build_parallax_background_pipeline, PARALLAX_BACKGROUND_PIPELINE_HANDLE};
use bevy::app::{AppBuilder, Plugin};
use bevy::asset::Assets;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::prelude::RenderPipelines;
use bevy::render::render_graph::RenderGraph;
use bevy::render::shader::Shader;
use bevy::sprite::QUAD_HANDLE;
use bevy::transform::components::{GlobalTransform, Transform};

#[derive(Default)]
pub struct ParallaxBackgroundPlugin;

impl Plugin for ParallaxBackgroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let world = app.world_mut();
        let world_cell = world.cell();
        let mut pipelines = world_cell
            .get_resource_mut::<Assets<PipelineDescriptor>>()
            .unwrap();
        let mut shaders = world_cell.get_resource_mut::<Assets<Shader>>().unwrap();
        let pipeline_handle = pipelines.set_untracked(
            PARALLAX_BACKGROUND_PIPELINE_HANDLE,
            build_parallax_background_pipeline(&mut shaders),
        );
    }
}
