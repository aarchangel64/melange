extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use keyframe::{ease, functions::EaseIn};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event_loop::EventLoop;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    size: f64,
    time: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 0.6];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let (x, y) = (
            (args.window_size[0] - self.size) / 2.0,
            (args.window_size[1] - self.size) / 2.0,
        );
        let square = rectangle::square(x, y, self.size);
        let rotation = self.rotation;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND, gl);

            let transform = c.transform;
            // .trans(x, y)
            // .rot_rad(rotation)
            // .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.

            Rectangle::new_border(WHITE, 1.0).draw(square, &c.draw_state, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        // self.size += 50.0 * args.dt;
        self.time += args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Use winit's EventLoop to get the available monitors
    let event = EventLoop::new();
    let mon = event.available_monitors().next().unwrap();
    let mon_width = (mon.size().width as f64 / mon.scale_factor()) as i32 + 5;
    let mon_height = (mon.size().height as f64 / mon.scale_factor()) as i32 + 5;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Logout-HUD", [10, 10])
        .graphics_api(opengl)
        .fullscreen(false)
        .resizable(false)
        .decorated(false)
        .transparent(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let gwindow = window.ctx.window();
    let pos = gwindow.primary_monitor().unwrap().position();
    gwindow.set_always_on_top(true);
    gwindow.set_outer_position(pos);
    gwindow.set_inner_size(LogicalSize::new(mon_width, mon_height));

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        size: 50.0,
        time: 0.0,
    };

    // let mut sequence = keyframes![
    // (0.0, 0.0),
    // (1.)]

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            let width = ease(EaseIn, 1.0, mon_width as f64, app.time / 5.0);
            let height = ease(EaseIn, 1.0, mon_width as f64, app.time / 5.0);
            let pos_x = pos.x + (mon_width - width as i32) / 2;
            let pos_y = pos.y + (mon_height - height as i32) / 2;
            window
                .ctx
                .window()
                .set_outer_position(PhysicalPosition::new(pos_x, pos_y));
            window
                .ctx
                .window()
                .set_inner_size(LogicalSize::new(width, height));
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
