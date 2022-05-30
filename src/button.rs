use ggez::{
    graphics::{self, Color, DrawParam, Font, MeshBuilder},
    Context, GameResult,
};

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
}

pub struct Button {
    label: String,
    colour: Color,
    thickness: f32,
    rect: Rect,
}

impl Button {
    pub fn new_empty(label: String, colour: Color, thickness: f32) -> Button {
        Button {
            label,
            colour,
            thickness,
            rect: Rect::new(1.0, 1.0, (1.0, 1.0)),
        }
    }

    pub fn new(
        label: String,
        colour: Color,
        thickness: f32,
        (width, height): (f32, f32),
        centre: (f32, f32),
    ) -> Button {
        Button {
            label,
            colour,
            thickness,
            rect: Rect::new(width, height, centre),
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

    pub fn anim_rect(&self, progress: f32, ctx: &mut Context) -> GameResult<&Button> {
        let mut mesh = MeshBuilder::new();

        let mut draw_line = |from: glam::Vec2, to: glam::Vec2| {
            mesh.line(&[from, to], self.thickness, self.colour).unwrap();
        };
        let map = |val: f32, start, end| (val.clamp(start, end) - start) / (end - start);

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
