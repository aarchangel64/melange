use std::time::Duration;

use ggez::{
    graphics::{self, Color, DrawParam, MeshBuilder},
    Context, GameResult,
};
use keyframe::functions::EaseInOut;

use crate::anim;

pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub centre: (f32, f32),
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Rect {
    pub fn new(width: f32, height: f32, centre: (f32, f32)) -> Rect {
        Rect {
            width,
            height,
            centre,
            left: centre.0 - width / 2.0,
            right: centre.0 + width / 2.0,
            // Subtract height because screen origin is at the top left
            top: centre.1 - height / 2.0,
            bottom: centre.1 + height / 2.0,
        }
    }

    pub fn inside(&self, x: f32, y: f32) -> bool {
        x < self.right && x > self.left && y > self.top && y < self.bottom
    }

    pub fn draw_surrond(
        &self,
        thickness: f32,
        draw_color: Color,
        anim_time: f32,
        delay: f32,
        time: Duration,
        ctx: &mut Context,
    ) -> GameResult<&Rect> {
        let progress = anim::run(EaseInOut, anim_time, delay, time);

        let mut mesh_builder = MeshBuilder::new();
        // Closure so that we don't have to specify thickness and colour everytime
        let mut draw_line = |from: glam::Vec2, to: glam::Vec2| -> GameResult<()> {
            mesh_builder
                .line(&[from, to], thickness, draw_color)
                .and(Ok(()))
        };

        // self.draw_colour = anim::map(progress, self.colour.to_rgba(), Color::RED.to_rgba());

        // Offset since lines would otherwise draw to the middle point of corners
        let offset = thickness / 2.0;

        // Skip drawing to avoid an error (and optimise) if there's degenerate geometry
        // Requires a small tolerance greater than just 0
        if progress >= 0.001 {
            // Bottom
            draw_line(
                glam::vec2(self.left, self.bottom),
                glam::vec2(
                    self.left + (self.width + offset) * anim::map(progress, 0.0, 0.25),
                    self.bottom,
                ),
            )?;

            // Right
            draw_line(
                glam::vec2(self.right, self.bottom),
                glam::vec2(
                    self.right,
                    self.bottom - (self.height + offset) * anim::map(progress, 0.25, 0.50),
                ),
            )?;

            // Top
            draw_line(
                glam::vec2(self.right, self.top),
                glam::vec2(
                    self.right - (self.width + offset) * anim::map(progress, 0.5, 0.75),
                    self.top,
                ),
            )?;

            // Left
            draw_line(
                glam::vec2(self.left, self.top),
                glam::vec2(
                    self.left,
                    self.top + self.height * anim::map(progress, 0.75, 1.0),
                ),
            )?;

            let mesh = mesh_builder.build(ctx).unwrap();
            graphics::draw(ctx, &mesh, DrawParam::new())?;
        }

        Ok(self)
    }

    pub fn draw_expand(
        &self,
        thickness: f32,
        draw_color: Color,
        anim_time: f32,
        delay: f32,
        time: Duration,
        ctx: &mut Context,
    ) -> GameResult<&Rect> {
        let progress = anim::run(EaseInOut, anim_time, delay, time);

        let mut mesh_builder = MeshBuilder::new();
        // Closure so that we don't have to specify thickness and colour everytime
        let mut draw_line = |from: glam::Vec2, to: glam::Vec2| -> GameResult<()> {
            mesh_builder
                .line(&[from, to], thickness, draw_color)
                .and(Ok(()))
        };

        // self.draw_colour = anim::map(progress, self.colour.to_rgba(), Color::RED.to_rgba());

        // Offset since lines would otherwise draw to the middle point of corners
        let offset = thickness / 2.0;

        // Skip drawing to avoid an error (and optimise) if there's degenerate geometry
        // Requires a small tolerance greater than just 0
        if progress >= 0.001 {
            // Bottom
            draw_line(
                glam::vec2(self.left, self.bottom),
                glam::vec2(
                    self.left + (self.width + offset) * anim::map(progress, 0.0, 0.25),
                    self.bottom,
                ),
            )?;

            // Right
            draw_line(
                glam::vec2(self.right, self.bottom),
                glam::vec2(
                    self.right,
                    self.bottom - (self.height + offset) * anim::map(progress, 0.25, 0.50),
                ),
            )?;

            // Top
            draw_line(
                glam::vec2(self.right, self.top),
                glam::vec2(
                    self.right - (self.width + offset) * anim::map(progress, 0.5, 0.75),
                    self.top,
                ),
            )?;

            // Left
            draw_line(
                glam::vec2(self.left, self.top),
                glam::vec2(
                    self.left,
                    self.top + self.height * anim::map(progress, 0.75, 1.0),
                ),
            )?;

            let mesh = mesh_builder.build(ctx).unwrap();
            graphics::draw(ctx, &mesh, DrawParam::new())?;
        }

        Ok(self)
    }
}
}
