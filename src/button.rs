use std::time::Duration;

use ggez::{
    graphics::{self, Color, DrawParam, Font, Image, MeshBuilder},
    Context, GameResult,
};
use keyframe::functions::EaseInOut;

use crate::{anim, rect::Rect, settings::ButtonData};

pub struct Button {
    label: String,
    pub command: Vec<String>,
    image: Option<Image>,
    // How much of the button should the image take up, scaled to 1.0.
    image_size: f32,
    color: Color,
    draw_color: Color,
    thickness: f32,
    rect: Rect,
    pub is_hovered: bool,
}

impl Button {
    pub fn new_empty(ctx: &mut Context, data: &ButtonData, dpi_scale: f32) -> Button {
        Button::new(ctx, data, (1.0, 1.0), (1.0, 1.0), dpi_scale)
    }

    pub fn new(
        ctx: &mut Context,
        data: &ButtonData,
        (width, height): (f32, f32),
        centre: (f32, f32),
        dpi_scale: f32,
    ) -> Button {
        Button {
            label: data.label.to_owned(),
            command: data.command.to_owned(),
            // TODO: remove need for leading slash
            image: data.image.as_ref().map(|s| Image::new(ctx, &s).unwrap()),
            image_size: data.image_size,
            // Multiply thickness by scaling factor to scale for DPI
            thickness: data.thickness * dpi_scale,
            color: data.color,
            draw_color: data.color,
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

    pub fn hover(&mut self, mouse_x: f32, mouse_y: f32) {
        self.is_hovered = self.rect.inside(mouse_x, mouse_y);

        // if (self.is_hovered != self.rect.inside(mouse_x, mouse_y)) {
        //     if
        // }

        self.draw_color = if self.rect.inside(mouse_x, mouse_y) {
            Color::RED
        } else {
            self.color
        };
    }

    pub fn draw(
        &self,
        anim_time: f32,
        delay: f32,
        time: Duration,
        ctx: &mut Context,
    ) -> GameResult<&Button> {
        self.rect
            .draw_surrond(self.thickness, self.draw_color, anim_time, delay, time, ctx)
            .and(Ok(self))
    }

    pub fn draw_image(&self, ctx: &mut Context) -> GameResult<&Button> {
        // TODO: add fading in effect
        if let Some(image) = &self.image {
            let scale = if image.width() > image.height() {
                self.rect.width / image.width() as f32
            } else {
                self.rect.height / image.height() as f32
            } * self.image_size;

            let top = (self.rect.height - image.height() as f32 * scale) / 2.0;
            let left = (self.rect.width - image.width() as f32 * scale) / 2.0;

            graphics::draw(
                ctx,
                image,
                DrawParam::default()
                    .dest(glam::vec2(self.rect.left + left, self.rect.top + top))
                    .scale(glam::vec2(scale, scale)),
            )?;
        }

        Ok(self)
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
}
