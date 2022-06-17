#[macro_use]
extern crate smart_default;

use std::env;
use std::fs::{canonicalize, read};
use std::process::Command;

use wry::application::event::KeyEvent;
use wry::application::keyboard::Key;
use wry::application::window::Fullscreen;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    http::{method::Method, ResponseBuilder},
    webview::WebViewBuilder,
};

// use settings::{Input, Settings};

// mod settings;

fn execute(inputs: &Vec<String>) {
    let output = Command::new(&inputs[0])
        .args(&inputs[1..])
        .output()
        .expect("failed to execute process");
    print!("{}", String::from_utf8(output.stdout).unwrap());
}

// if let Some(command) = self.config.keymap.get(&Input { key, mods }) {
//     MainState::execute(command);
// }

// if key == KeyCode::Escape {
//     event::quit(ctx)
// }

fn main() -> wry::Result<()> {
    let config_dir = &format!(
        "{}/informant",
        env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!(
            "{}/.config",
            env::var("HOME").expect(
                "Your $HOME variable isn't set, I think you have bigger problems than this error."
            )
        ))
    );

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Informant")
        .with_decorations(false)
        .with_always_on_top(true)
        .with_transparent(true)
        // .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .unwrap();

    let monitor = window.primary_monitor().unwrap();
    window.set_inner_size(monitor.size());
    window.set_outer_position(monitor.position());

    let webview = WebViewBuilder::new(window)
        .unwrap()
        .with_transparent(true)
        .with_custom_protocol("wry".into(), move |request| {
            // TODO: Add check to make sure only files in the config directory can be accessed (with an option, maybe?)
            // Remove url scheme
            let path = request.uri().replace("wry://", "");
            ResponseBuilder::new()
                .mimetype("text/html")
                .body(read(canonicalize(&path)?)?)
        })
        // tell the webview to load the custom protocol
        // .with_url(&format!("wry://{}/index.html", config_dir))?
        .with_url("http://127.0.0.1:8080")?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Escape,
                                ..
                            },
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

// Build the loop.
// TODO: Handle errors
// let (mut ctx, event_loop) = cb.build()?;

// let (settings, buttons) = Settings::new(&ctx);

// let window = graphics::window(&ctx);
// let scale = window.scale_factor() as f32;
// let monitor = window.current_monitor().unwrap();

// let monitor_width = monitor.size().width as f32;
// let monitor_height = monitor.size().height as f32;

// if let Err(err) = graphics::set_fullscreen(&mut ctx, settings.fullscreen) {
//     eprintln!("{err}")
// }

// // Set window size to cover entire screen.
// if settings.fullscreen != FullscreenType::Windowed {
//     if let Err(err) = graphics::set_drawable_size(&mut ctx, monitor_width, monitor_height) {
//         eprintln!("{err}")
//     }

//     if let Err(err) = graphics::set_screen_coordinates(
//         &mut ctx,
//         Rect {
//             x: 0.,
//             y: 0.,
//             w: monitor_width,
//             h: monitor_height,
//         },
//     ) {
//         eprintln!("{err}")
//     }
// }

// if settings.fullscreen == FullscreenType::Desktop {
//     let window = graphics::window(&ctx);
//     let pos = monitor.position();
//     window.set_always_on_top(true);
//     window.set_decorations(false);
//     window.set_resizable(false);
//     window.set_outer_position(pos);
// }

// // TODO: Handle setting fullscreen result?

// // Convert button data from config file into button structs
// let buttons = buttons
//     .iter()
//     .map(|b| Button::new_empty(&mut ctx, b, scale))
//     .collect();

// let game = MainState::new(&mut ctx, scale, buttons, settings)?;
// event::run(ctx, event_loop, game);
