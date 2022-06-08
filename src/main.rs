use std::process::Command;
use std::time::Duration;

use ggez::conf::{self, FullscreenType};
use ggez::event::{self, EventLoop, KeyCode, KeyMods};
use ggez::graphics::{self, Color, Font};
use ggez::winit::dpi::LogicalSize;
use ggez::{timer, Context, GameError, GameResult};

use fontconfig::Fontconfig;
use settings::{ConfigData, Settings};

use crate::button::Button;

mod anim;
mod button;
mod settings;

const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 0.6];

struct UI {
    buttons: Vec<Button>,
}

pub struct MainState {
    dt: Duration,
    time: Duration,
    pos: (f32, f32),
    ui: UI,
    scale_factor: f32,
    font: Font,
    config: Settings,
}

impl MainState {
    fn new(
        ctx: &mut Context,
        scale_factor: f32,
        buttons: Vec<Button>,
        settings: Settings,
    ) -> GameResult<MainState> {
        let fc = Fontconfig::new().unwrap();
        let font = fc
            .find(&settings.font.family, Some(&settings.font.style))
            .unwrap();
        println!("{}", font.path.to_str().unwrap());

        let bytes = std::fs::read(font.path).unwrap();
        let font = Font::new_glyph_font_bytes(ctx, &bytes).unwrap();

        let state = MainState {
            dt: Duration::new(0, 0),
            time: Duration::new(0, 0),
            pos: (0.0, 0.0),
            ui: UI { buttons },
            scale_factor,
            font,
            config: settings,
        };

        Ok(state)
    }
}

impl event::EventHandler<GameError> for MainState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear the screen.
        graphics::clear(ctx, BACKGROUND.into());

        let font_size = self.config.font.size * self.scale_factor;

        for (i, button) in self.ui.buttons.iter().enumerate() {
            button
                .draw(
                    self.config.anim.duration,
                    self.config.anim.delay * i as f32,
                    self.time,
                    ctx,
                )?
                .draw_label(self.font, font_size, ctx)?;
        }

        let text = graphics::Text::new((
            format!(
                "fps: {}, mouse: {} {}",
                ggez::timer::fps(ctx).round(),
                self.pos.0,
                self.pos.1
            ),
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

        self.dt = timer::delta(ctx);
        self.time = timer::time_since_start(ctx);
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        // Button sizes get set here, since a resize event is fired on first draw (I think)
        let button_size = width / 6.0;
        let grid_width = width / 6.0;
        let grid_height = height / 2.0;

        for (i, button) in self.ui.buttons.iter_mut().enumerate() {
            button.set_size(button_size, button_size);
            button.set_pos((i + 1) as f32 * 1.5 * grid_width, grid_height);
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _repeat: bool) {
        match key {
            // TODO: Make a config for keymap and shell
            KeyCode::L => {
                let c = Command::new("sh")
                    .arg("-c")
                    .arg("echo hello")
                    .output()
                    .expect("failed to execute process");
                println!("{}", String::from_utf8(c.stdout).unwrap());
            }
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        };
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        // Mouse coordinates are PHYSICAL coordinates, but here we want logical coordinates.

        // If you simply use the initial coordinate system, then physical and logical
        // coordinates are identical.
        self.pos.0 = x;
        self.pos.1 = y;

        for button in &mut self.ui.buttons {
            button.hover(x, y);
        }
    }
}

fn main() -> GameResult {
    // TODO: Handle invalid config error
    let settings = ConfigData::new();

    // Deserialize (and thus freeze) the entire configuration
    let (settings, buttons) = match settings {
        Ok(s) => (
            Settings {
                fullscreen: s.fullscreen,
                shell: s.shell,
                anim: s.anim,
                font: s.font,
            },
            s.buttons,
        ),
        // TODO: Handle error better (maybe an error popup?)
        Err(error) => panic!("{}", error),
    };

    // Create an eventloop to get the monitor's size, in case some WMs don't respect set_inner_size
    let size = EventLoop::new().primary_monitor().unwrap().size();

    let fullscreen = if settings.fullscreen {
        FullscreenType::True
    } else {
        FullscreenType::Desktop
    };

    let cb = ggez::ContextBuilder::new("informant", "cosmicdoge")
        .with_conf_file(false)
        .window_mode(
            conf::WindowMode::default()
                .dimensions(size.width as f32, size.height as f32)
                .fullscreen_type(fullscreen)
                .transparent(true),
        );
    let (mut ctx, event_loop) = cb.build()?;

    let window = graphics::window(&ctx);
    let scale = window.scale_factor() as f32;

    if fullscreen != FullscreenType::True {
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

    let buttons = buttons
        .iter()
        // Multiply thickness by scaling factor to scale for DPI
        .map(|b| Button::new_empty(b.label.to_owned(), Color::WHITE, b.thickness * scale))
        .collect();

    let game = MainState::new(&mut ctx, scale, buttons, settings)?;
    event::run(ctx, event_loop, game)

    // let mut sequence = keyframes![
    // (0.0, 0.0),
    // (1.)]
}
