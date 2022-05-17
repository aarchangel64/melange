use fontconfig::Fontconfig;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use winit::dpi::LogicalSize;

use keyframe::functions::{EaseInOutCubic, EaseInQuint, Linear};
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
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 0.6];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let size = ease(EaseInOutCubic, 0.0, 1.0, (self.time / 1.5).clamp(0.0, 1.0));

        let (x, y) = (
            args.window_size[0] * (1.0 - size) / 2.0,
            args.window_size[1] * (1.0 - size) / 2.0,
        );

        let square = rectangle::rectangle_by_corners(
            x,
            y,
            x + size * args.window_size[0],
            y + size * args.window_size[1],
        );
        let rotation = self.rotation;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);

            let transform = c.transform;
            // .trans(x, y)
            // .rot_rad(rotation)
            // .trans(-25.0, -25.0);

            Rectangle::new_border(WHITE, 1.0).draw(square, &c.draw_state, transform, gl);
            Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
                .draw(
                    format!("fps: {}, tps: {}", self.fps, self.ups).as_str(),
                    &mut self.glyph,
                    &c.draw_state,
                    transform.trans(10.0, 30.0).zoom(0.5),
                    gl,
                )
                .unwrap();
        });
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
