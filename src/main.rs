extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
// use glfw_window::GlfwWindow as Window;
use keyframe::functions::Linear;
use keyframe::{ease, functions::EaseIn};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use winit::dpi::{LogicalSize, PhysicalPosition};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    time: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 0.6];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let size = ease(Linear, 0.0, 1.0, (self.time / 1.0).clamp(0.0, 1.0));

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

            // Draw a box rotating around the middle of the screen.

            Rectangle::new_border(WHITE, 1.0).draw(square, &c.draw_state, transform, gl);
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

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Logout-HUD", [100, 100])
        .graphics_api(opengl)
        .fullscreen(false)
        .resizable(true)
        .decorated(false)
        .transparent(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let gwindow = window.ctx.window();
    let monitor = gwindow.current_monitor().unwrap();
    let monitor_width = (monitor.size().width as f64 / monitor.scale_factor()) as i32;
    let monitor_height = (monitor.size().height as f64 / monitor.scale_factor()) as i32;
    let mut pos = monitor.position();
    println!("{}", pos.x);
    // pos.x += 500;
    // pos.y += 50;
    gwindow.set_always_on_top(true);
    // gwindow.set_outer_position(PhysicalPosition::new(0, 0));
    gwindow.set_outer_position(pos);
    gwindow.set_inner_size(LogicalSize::new(monitor_width, monitor_height));

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        time: 0.0,
    };

    // let mut sequence = keyframes![
    // (0.0, 0.0),
    // (1.)]

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
