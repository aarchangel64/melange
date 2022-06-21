# Melange


https://user-images.githubusercontent.com/13304815/174892139-d6a296af-01dd-48cf-b2f6-65e32b881e51.mp4


Dashboards. Logout Screens. Weather Widgets. App Launchers. Anything your heart so desires.
Easily create interactive, responsive, animated GUIs to customise your setup with the power of HTML, CSS, and Javascript!

Melange is program that renders a web page a using the embedded system WebView (WebkitGTK on Linux).
It uses [Wry](https://github.com/tauri-apps/wry) under the hood.

## Prerequisites

[WebkitGTK](https://webkitgtk.org/) is required for the WebView.
Install it as so:

#### Arch Linux / Manjaro:

```bash
sudo pacman -S webkit2gtk
```

#### Debian / Ubuntu:

```bash
sudo apt install libwebkit2gtk-4.0-dev
```

#### Fedora

```bash
sudo dnf install gtk3-devel webkit2gtk3-devel
```

## Usage

Melange will load `index.html` from its config directory.
Melange takes a single optional CLI argument, a config/data directory:

```bash
melange your/config/path/here
```

This allows you to make different HTML frontends for multiple different use cases.
If this is not passed, Melange will default to looking in your user config directory (`$XDG_CONFIG_HOME/melange/`, or `~/.config/melange/` on Linux).

Note that you can also pass in URLs like so:

```bash
melange http://127.0.0.1:8080
```

This can be useful if you wanted to develop your UI using a local dev server from tools such as [Webpack](https://webpack.js.org) or [Vite](https://vitejs.dev).
I **highly discourage** loading pages from the internet due to security risk, since Melange can run shell commands on your machine from JavaScript (even though the commands are only ones you manually specify in `config.toml`).

