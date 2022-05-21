use std::arch::x86_64::_MM_FROUND_CUR_DIRECTION;
use std::borrow::Borrow;

use ggez::conf;
use ggez::conf::FullscreenType;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::event::EventLoop;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::winit::dpi::LogicalSize;
use ggez::{Context, GameResult};

use glam::Vec2;

use keyframe::functions::EaseInOut;

use fontconfig::Fontconfig;
use keyframe::{ease, EasingFunction};

use crate::button::Button;

mod button;

const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 0.6];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

struct UI {
    logout: Button,
    sleep: Button,
    power: Button,
}

pub struct MainState {
    time: f64,
    pos: [f64; 2],
    ui: UI,
    font: graphics::Font,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let fc = Fontconfig::new().unwrap();
        let font = fc.find("iosevka", Some("italic")).unwrap();
        println!("{}", font.path.to_str().unwrap());

        let bytes = std::fs::read(font.path).unwrap();
        let font = graphics::Font::new_glyph_font_bytes(ctx, &bytes).unwrap();

        let state = MainState {
            time: 0.0,
            pos: [0.0, 0.0],
            ui: UI {
                logout: Button::new_empty(WHITE, 0.5),
                sleep: Button::new_empty(WHITE, 0.5),
                power: Button::new_empty(WHITE, 0.5),
            },
            font,
        };

        Ok(state)
    }
}

#[inline]
fn anim<F: EasingFunction>(function: impl Borrow<F>, seconds: f64, offset: f64, time: f64) -> f64 {
    return ease(
        function,
        0.0,
        1.0,
        ((time - offset) / seconds).clamp(0.0, 1.0),
    );
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear the screen.
        graphics::clear(ctx, BACKGROUND.into());

        // let transform = ctx.transform;
        // let anim_time = 0.7;

        // self.ui
        //     .logout
        //     .anim_rect(anim(EaseInOut, anim_time, 0.0, self.time), ctx, gl)
        //     .draw_label("test", glyph, ctx, gl);
        // self.ui
        //     .sleep
        //     .anim_rect(anim(EaseInOut, anim_time, 0.3, self.time), ctx, gl);
        // self.ui
        //     .power
        //     .anim_rect(anim(EaseInOut, anim_time, 0.6, self.time), ctx, gl);

        let text = graphics::Text::new((
            format!("fps: {}", ggez::timer::fps(ctx).round()),
            self.font,
            48.0,
        ));
        let test = glam::vec2(100.0, 100.0);
        graphics::draw(ctx, &text, (test,))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // self.ui.logout.set_size(self.ui.logout.rect.width * 1.001, self.ui.logout.rect.height * 1.001);
        Ok(())
    }
}

fn main() -> GameResult {
    // Create an eventloop to get the monitor's size, in case some WMs don't respect set_inner_size
    let size = EventLoop::new().primary_monitor().unwrap().size();
    // TODO: Make this a part of the config
    const FULLSCREEN: FullscreenType = FullscreenType::True;

    let cb = ggez::ContextBuilder::new("informant", "cosmicdoge").window_mode(
        conf::WindowMode::default()
            .dimensions(size.width as f32, size.height as f32)
            .fullscreen_type(FULLSCREEN)
            .transparent(true),
    );
    let (mut ctx, event_loop) = cb.build()?;

    if FULLSCREEN != FullscreenType::True {
        let window = graphics::window(&ctx);
        let monitor = window.current_monitor().unwrap();
        let monitor_width = (monitor.size().width as f64 / monitor.scale_factor()) as i32;
        let monitor_height = (monitor.size().height as f64 / monitor.scale_factor()) as i32;
        let pos = monitor.position();
        window.set_always_on_top(true);
        window.set_decorations(false);
        window.set_resizable(false);
        window.set_outer_position(pos);
        window.set_inner_size(LogicalSize::new(monitor_width, monitor_height));
    }

    let game = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
    // Create an eventloop to get the monitor's size, in case some WMs don't respect set_inner_size
    // let size = EventLoop::new().primary_monitor().unwrap().size();

    // let settings = WindowSettings::new("Logout-HUD", [size.width, size.height])
    //     .graphics_api(opengl)
    //     .vsync(true)
    //     .fullscreen(true)
    //     .resizable(false)
    //     .decorated(false)
    //     .transparent(true)
    //     .exit_on_esc(true);

    // let mut window: GlutinWindow = settings.build().unwrap();

    // let gwindow = window.ctx.window();

    // let mut sequence = keyframes![
    // (0.0, 0.0),
    // (1.)]

    // let mut events = Events::new(EventSettings::new());

    // while let Some(e) = events.next(&mut window) {
    //     if let Some(pos) = e.mouse_cursor_args() {
    //         app.pos = pos;
    //     }

    //     if let Some(args) = e.render_args() {
    //         app.render(&args, &mut glyph_cache, &mut gl);
    //         app.fps = fps.tick();
    //     }

    //     if let Some(args) = e.update_args() {
    //         app.update(&args);
    //         app.ups = ups.tick();
    //     }

    //     if let Some(args) = e.resize_args() {
    //         let button_size = args.window_size[0] / 6.0;
    //         let grid_width = args.window_size[0] / 6.0;
    //         let grid_height = args.window_size[1] / 2.0;

    //         app.ui.logout.set_size(button_size, button_size);
    //         app.ui.logout.set_pos((1.5 * grid_width, grid_height));

    //         app.ui.sleep.set_size(button_size, button_size);
    //         app.ui.sleep.set_pos((3.0 * grid_width, grid_height));

    //         app.ui.power.set_size(button_size, button_size);
    //         app.ui.power.set_pos((4.5 * grid_width, grid_height));
    //     }
    // }
}
