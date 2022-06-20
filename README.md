# Informant

Dashboards. Logout Screens. Weather Widgets. (Anything you want, really!)
Easily create custom GUIs to rice your setup with the power of HTML, CSS, and Javascript!

Informant is program that renders a web page a using the embedded system WebView (WebkitGTK on Linux).
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

Informant takes a single optional CLI argument: a config/data directory:

```bash
informant your/html/path/here
```

This allows you to make different HTML frontends for multiple different use cases.
If this is not passed, Informant will default to looking in your user config directory (`$XDG_CONFIG_HOME/informant/`, or `~/.config/informant/` on Linux).
