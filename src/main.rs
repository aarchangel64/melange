use std::borrow::Borrow;

use fontconfig::Fontconfig;
use glutin_window::GlutinWindow;
use graphics::{line, Context, DrawState, Graphics, Line};
use keyframe::EasingFunction;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use winit::dpi::LogicalSize;

use keyframe::functions::{
    EaseInCubic, EaseInOutCubic, EaseInQuint, EaseOut, EaseOutCubic, Linear,
};
use keyframe::{ease, functions::EaseIn};

use fps_counter::*;

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    glyph: GlyphCache<'a>,
    rotation: f64, // Rotation for the square.
    time: f64,
    fps: usize,
    ups: usize,
}

impl App<'_> {
    #[inline]
    fn anim<F: EasingFunction>(
        function: impl Borrow<F>,
        seconds: f64,
        offset: f64,
        time: f64,
    ) -> f64 {
        return ease(
            function,
            0.0,
            1.0,
            ((time - offset) / seconds).clamp(0.0, 1.0),
        );
    }

    fn anim_rect<F: EasingFunction>(
        &mut self,
        colour: [f32; 4],
        radius: f64,
        (mut width, mut height): (f64, f64),
        (x, y): (f64, f64),
        func: impl Borrow<F>,
        mut seconds: f64,
        ctx: Context,
    ) {
        width /= 2.0;
        height /= 2.0;
        seconds /= 4.0;

        let mut draw_line =
            |points: [f64; 4]| line(colour, radius, points, ctx.transform, &mut self.gl);

        // Bottom
        draw_line([
            x - width,
            y + height,
            x + width * (2.0 * App::anim(EaseOut, seconds, 0.0, self.time) - 1.0),
            y + height,
        ]);
        // Left
        draw_line([
            x + width,
            y + height,
            x + width,
            y - height * (2.0 * App::anim(EaseOut, seconds, seconds, self.time) - 1.0),
        ]);
        // Top
        draw_line([
            x + width,
            y - height,
            x - width * (2.0 * App::anim(EaseOut, seconds, 2.0 * seconds, self.time) - 1.0),
            y - height,
        ]);
        // Right
        draw_line([
            x - width,
            y - height,
            x - width,
            y + height * (2.0 * App::anim(EaseOut, seconds, 3.0 * seconds, self.time) - 1.0),
        ]);
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 0.6];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let ctx = self.gl.draw_begin(args.viewport());
        // Clear the screen.
        clear(BACKGROUND, &mut self.gl);

        let transform = ctx.transform;
        // .trans(x, y)
        // .rot_rad(rotation)
        // .trans(-25.0, -25.0);

        self.anim_rect(
            WHITE,
            1.0,
            (500.0, 500.0),
            (1920.0 / 2.0, 1080.0 / 2.0),
            EaseOut,
            3.0,
            ctx,
        );

        Text::new_color(WHITE, 32)
            .draw(
                format!("fps: {}, tps: {}", self.fps, self.ups).as_str(),
                &mut self.glyph,
                &ctx.draw_state,
                transform.trans(10.0, 30.0).zoom(0.5),
                &mut self.gl,
            )
            .unwrap();

        self.gl.draw_end();
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.time += args.dt;

        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let settings = WindowSettings::new("Logout-HUD", [200, 200])
        .graphics_api(opengl)
        .vsync(true)
        .fullscreen(false)
        .resizable(false)
        .decorated(false)
        .transparent(true)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build().unwrap();

    let gwindow = window.ctx.window();
    let monitor = gwindow.current_monitor().unwrap();
    let monitor_width = (monitor.size().width as f64 / monitor.scale_factor()) as i32;
    let monitor_height = (monitor.size().height as f64 / monitor.scale_factor()) as i32;
    let pos = monitor.position();
    gwindow.set_always_on_top(true);
    gwindow.set_outer_position(pos);
    gwindow.set_inner_size(LogicalSize::new(monitor_width, monitor_height));

    let fc = Fontconfig::new().unwrap();
    let font = fc.find("iosevka cosmic", Some("italic")).unwrap();
    println!("{}", font.path.to_str().unwrap());

    let glyph_cache = GlyphCache::new(font.path.as_path(), (), TextureSettings::new()).unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        glyph: glyph_cache,
        rotation: 0.0,
        time: 0.0,
        fps: 0,
        ups: 0,
    };

    let mut ups = FPSCounter::default();
    let mut fps = FPSCounter::default();
    // let mut sequence = keyframes![
    // (0.0, 0.0),
    // (1.)]

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
            app.fps = fps.tick();
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
            app.ups = ups.tick();
        }
    }
}
