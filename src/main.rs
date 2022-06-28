use melange::Melange;

use std::env;
use wry::application::event_loop::EventLoop;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_dir = if let Some(path) = args.get(1) {
        path.to_owned()
    } else {
        format!(
            "{}/melange",
            env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!(
            "{}/.config",
            env::var("HOME").expect(
                "Your $HOME variable isn't set, I think you have bigger problems than this error."
            )
        ))
        )
    };

    let mut melange = Melange::new(config_dir);
    let event_loop = EventLoop::new();
    let window = melange.make_window(&event_loop);
    let webview = melange.make_webview(window);
    Melange::run_loop(event_loop);
}
