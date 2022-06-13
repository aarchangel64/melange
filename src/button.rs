use std::time::Duration;

use ggez::{
    graphics::{self, Color, DrawParam, Font, Image, MeshBuilder},
    Context, GameResult,
};
use keyframe::functions::EaseInOut;

use crate::{anim, settings::ButtonData};

pub struct Rect {
    width: f32,
    height: f32,
    centre: (f32, f32),
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
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
}

pub struct Button {
    label: String,
    pub command: Vec<String>,
    image: Option<Image>,
    // How much of the button should the image take up, in percentage.
    image_size: f32,
    colour: Color,
    draw_colour: Color,
    thickness: f32,
    rect: Rect,
    pub is_hovered: bool,
}

impl Button {
    pub fn new_empty(ctx: &mut Context, data: &ButtonData, colour: Color, scale: f32) -> Button {
        Button::new(ctx, data, colour, (1.0, 1.0), (1.0, 1.0), scale)
    }

    pub fn new(
        ctx: &mut Context,
        data: &ButtonData,
        colour: Color,
        (width, height): (f32, f32),
        centre: (f32, f32),
        scale: f32,
    ) -> Button {
        Button {
            label: data.label.to_owned(),
            command: data.command.to_owned(),
            image: data.image.as_ref().map(|s| Image::new(ctx, &s).unwrap()),
            image_size: data.image_size,
            // Multiply thickness by scaling factor to scale for DPI
            thickness: data.thickness * scale,
            colour,
            draw_colour: colour,
            rect: Rect::new(width, height, centre),
            is_hovered: false,
        }
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.rect = Rect::new(width, height, self.rect.centre);
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.rect = Rect::new(self.rect.width, self.rect.height, (x, y))
    }

    pub fn draw_label(&self, font: Font, size: f32, ctx: &mut Context) -> GameResult<&Button> {
        let mut text = graphics::Text::new((self.label.as_str(), font, size));
        text.set_bounds(
            glam::vec2(self.rect.width, f32::INFINITY),
            graphics::Align::Center,
        );

        graphics::draw(
            ctx,
            &text,
            (glam::vec2(
                self.rect.left,
                self.rect.bottom + self.rect.height * 0.05,
            ),),
        )?;

        Ok(self)
    }

    pub fn hover(&mut self, mouse_x: f32, mouse_y: f32) {
        self.is_hovered = self.rect.inside(mouse_x, mouse_y);

        // if (self.is_hovered != self.rect.inside(mouse_x, mouse_y)) {
        //     if
        // }

        self.draw_colour = if self.rect.inside(mouse_x, mouse_y) {
            Color::RED
        } else {
            self.colour
        };
    }

    pub fn draw(
        &self,
        anim_time: f32,
        delay: f32,
        time: Duration,
        ctx: &mut Context,
    ) -> GameResult<&Button> {
        if let Some(image) = &self.image {
            let scale = if image.width() > image.height() {
                self.rect.width / image.width() as f32
            } else {
                self.rect.height / image.height() as f32
            };

            graphics::draw(
                ctx,
                image,
                DrawParam::default()
                    .dest(glam::vec2(self.rect.left, self.rect.top))
                    .scale(glam::vec2(scale, scale)),
            );
        }

        let progress = anim::run(EaseInOut, anim_time, delay, time);

        let map = |val: f32, start, end| (val.clamp(start, end) - start) / (end - start);
        let mut mesh = MeshBuilder::new();
        let mut draw_line = |from: glam::Vec2, to: glam::Vec2| {
            mesh.line(&[from, to], self.thickness, self.draw_colour)
                .unwrap();
        };

        // self.draw_colour = map(progress, self.colour.to_rgba(), Color::RED.to_rgba());

        // Offset since lines would otherwise draw to the middle point of corners
        let offset = self.thickness / 2.0;

        // Skip drawing to avoid an error (and optimise) if there's degenerate geometry
        // Requires a small tolerance greater than just 0
        if progress >= 0.001 {
            // Bottom
            draw_line(
                glam::vec2(self.rect.left, self.rect.bottom),
                glam::vec2(
                    self.rect.left + (self.rect.width + offset) * map(progress, 0.0, 0.25),
                    self.rect.bottom,
                ),
            );

            // Right
            draw_line(
                glam::vec2(self.rect.right, self.rect.bottom),
                glam::vec2(
                    self.rect.right,
                    self.rect.bottom - (self.rect.height + offset) * map(progress, 0.25, 0.50),
                ),
            );

            // Top
            draw_line(
                glam::vec2(self.rect.right, self.rect.top),
                glam::vec2(
                    self.rect.right - (self.rect.width + offset) * map(progress, 0.5, 0.75),
                    self.rect.top,
                ),
            );

            // Left
            draw_line(
                glam::vec2(self.rect.left, self.rect.top),
                glam::vec2(
                    self.rect.left,
                    self.rect.top + self.rect.height * map(progress, 0.75, 1.0),
                ),
            );

            let test = mesh.build(ctx).unwrap();
            // graphics::draw(ctx, &test, (glam::vec2(1.0, 1.0), 1.0, graphics::Color::GREEN)).unwrap();
            graphics::draw(ctx, &test, DrawParam::new()).unwrap();
        }

        Ok(self)
    }
}
