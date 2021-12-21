//
// Copyright 2019-2020 Hans W. Uhlig. All Rights Reserved.
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

use crate::builder::cluster::ProtoCluster;
use draw::{Canvas, Color, Drawing, Point, Shape, Style, SvgRenderer};
use hex2d::{Coordinate, Spacing};
use std::error::Error;
use std::ops::Range;
use std::path::Path;
use draw::shape::LinePoint;

/// Plotter Struct
pub struct Plotter {
    radius: f32,
    scaling: f32,
    canvas: Canvas,
}

impl Plotter {
    /// Create a new Plotter
    ///
    /// Arguments
    /// * `radius`: Distance from center of galaxy.
    /// * `scale`: Scale of the Galaxy
    ///
    pub fn new(radius: f32, scaling: f32) -> Plotter {
        let width = (radius * scaling * 2.0) as u32;
        let canvas = Canvas::new(width, width);
        Plotter {
            radius,
            scaling,
            canvas,
        }
    }
    pub fn width(&self) -> u32 {
        self.canvas.width
    }
    pub fn height(&self) -> u32 {
        self.canvas.height
    }
    fn translate( &self, x: f32, y: f32) -> Point {
        Point {
            x: x*self.scaling + self.radius,
            y: y*self.scaling + self.radius,
        }
    }
    pub fn draw_grid(&mut self) {
        self.canvas.display_list.add(
            Drawing::new()
                .with_shape(Shape::Line {
                    start:  self.translate(0.0, self.radius),
                    points: vec![LinePoint::Straight {point: self.translate(0.0, -self.radius)}],
                })
                .with_style(Style::stroked(2, Color::gray(1))),
        );
        self.canvas.display_list.add(
            Drawing::new()
                .with_shape(Shape::Line {
                    start:  self.translate(self.radius, 0.0),
                    points: vec![LinePoint::Straight {point: self.translate(-self.radius, 0.0)}],
                })
                .with_style(Style::stroked(2, Color::gray(1))),
        );
    }
    pub fn draw_cluster(&mut self, cluster: &ProtoCluster) {
        self.canvas.display_list.add(
            Drawing::new()
                .with_xy(
                    cluster.center.x * self.scaling,
                    cluster.center.y * self.scaling,
                )
                .with_shape(Shape::Circle {
                    radius: cluster.radius as u32,
                })
                .with_style(Style::stroked(2, Color::())),
        );
    }
    pub fn draw_hex(&mut self, hex: &Coordinate) {
        let center = hex.to_pixel(Spacing::PointyTop(self.scaling));

        self.canvas.display_list.add(
            Drawing::new()
                .with_shape(Shape::Line {
                    start: ,
                    points: vec![] })
                .with_style(Style::stroked(2, Color::black())),
        );
    }
    /// Save current plotter
    pub fn save(&self, path: &str, width: u32, height: u32) -> std::io::Result<()> {
        let mut canvas = Canvas::new(width, height);
        self.drawings.into_iter().for_each(|drawing| {
            canvas.display_list.add(drawing);
        });
        draw::render::save(&canvas, path, SvgRenderer::new())
    }
}

impl Default for Plotter {
    fn default() -> Self {
        Plotter::new(1000.0, 32.0)
    }
}
