use graphics::{Context, Line, Text, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};

pub struct Rect {
    width: f64,
    height: f64,
    centre: (f64, f64),
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

impl Rect {
    pub fn new(width: f64, height: f64, centre: (f64, f64)) -> Rect {
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
    colour: [f32; 4],
    thickness: f64,
    rect: Rect,
}

impl Button {

    pub fn new_empty(
        colour: [f32; 4],
        thickness: f64,
    ) -> Button {
        Button {
            colour,
            thickness,
            rect: Rect::new(0.0, 0.0, (0.0, 0.0)),
        }
    }

    pub fn new(
        colour: [f32; 4],
        thickness: f64,
        (width, height): (f64, f64),
        centre: (f64, f64),
    ) -> Button {
        Button {
            colour,
            thickness,
            rect: Rect::new(width, height, centre),
        }
    }

    pub fn set_size(&mut self, width: f64, height: f64) {
        self.rect = Rect::new(width, height, self.rect.centre);
    }

    pub fn set_pos(&mut self, centre: (f64, f64)) {
        self.rect = Rect::new(self.rect.width, self.rect.height, centre)
    }

    pub fn draw_label(&self, text: &str, glyph: &mut GlyphCache, ctx: Context, gl: &mut GlGraphics) {
        Text::new_color(self.colour, 32)
            .draw(
                text,
                glyph,
                &ctx.draw_state,
                ctx.transform.trans(self.rect.centre.0, self.rect.bottom + self.rect.height * 0.1).zoom(0.5),
                gl,
            )
            .unwrap();

    }

    pub fn anim_rect(&self, progress: f64, ctx: Context, gl: &mut GlGraphics) -> &Button {
        let mut draw_line = |points: [f64; 4]| {
            Line::new_round(self.colour, self.thickness).draw(
                points,
                &ctx.draw_state,
                ctx.transform,
                gl,
            )
        };

        let map = |val: f64, start, end| (val.clamp(start, end) - start) / (end - start);

        // Bottom
        draw_line([
            self.rect.left,
            self.rect.bottom,
            self.rect.left + self.rect.width * map(progress, 0.0, 0.25),
            self.rect.bottom,
        ]);
        // Left
        draw_line([
            self.rect.right,
            self.rect.bottom,
            self.rect.right,
            self.rect.bottom - self.rect.height * map(progress, 0.25, 0.50),
        ]);
        // Top
        draw_line([
            self.rect.right,
            self.rect.top,
            self.rect.right - self.rect.width * map(progress, 0.50, 0.75),
            self.rect.top,
        ]);
        // Right
        draw_line([
            self.rect.left,
            self.rect.top,
            self.rect.left,
            self.rect.top + self.rect.height * map(progress, 0.75, 1.0),
        ]);

        self
    }
}
