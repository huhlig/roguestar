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

use bevy::asset::{Assets, HandleUntyped};
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::{
    BlendComponent, BlendFactor, BlendOperation, BlendState, ColorTargetState, ColorWrite,
    CompareFunction, DepthBiasState, DepthStencilState, FrontFace, PipelineDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, StencilFaceState, StencilState,
};
use bevy::render::render_graph::RenderGraph;
use bevy::render::shader::{Shader, ShaderStage, ShaderStages};
use bevy::render::texture::TextureFormat;

pub const PARALLAX_BACKGROUND_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 4389734874387384874);

pub(crate) fn build_parallax_background_pipeline(
    shaders: &mut Assets<Shader>,
) -> PipelineDescriptor {
    PipelineDescriptor {
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilState {
                front: StencilFaceState::IGNORE,
                back: StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: DepthBiasState {
                constant: 0,
                slope_scale: 0.0,
                clamp: 0.0,
            },
        }),
        color_target_states: vec![ColorTargetState {
            format: TextureFormat::default(),
            blend: Some(BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
                alpha: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
            }),
            write_mask: ColorWrite::ALL,
        }],
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: PolygonMode::Fill,
            clamp_depth: false,
            conservative: false,
        },
        ..PipelineDescriptor::new(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("parallax.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("parallax.frag"),
            ))),
        })
    }
}
