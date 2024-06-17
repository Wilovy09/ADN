<div align="center">
<a href="https://github.com/Wilovy09/ADN">
<img src="./static/images/readme.png">
</a>
<h1>Facilitate the installation of LSPs in <a href="https://helix-editor.com/" target="_blank">Helix editor</a>.</h1>

<a target="_blank" href="https://crates.io/crates/adn">
<img alt="crates.io" src="https://img.shields.io/crates/v/adn.svg?style=for-the-badge&color=28153f&logo=rust" height="20">
</a>
</div>

> [!WARNING]
> This project seeks the contribution of more people in order to have a lot of support for different languages with working configurations.

## Installation

It is very simple, you just need to have `cargo` installed on your computer.

```sh
cargo install adn
```

> [!WARNING]
> Make sure you have `~/.cargo/bin` in your PATH to be able to use the `adn` command.

## How to use

```sh
adn ACTION LANGUAGE
```

| Arg      | Description                                                                                  |
|----------|----------------------------------------------------------------------------------------------|
| ACTION   | In this part we will put the action that we want to do `i` to install or (`r` to remove WIP) |
| LANGUAGE | Here we will put the name of the language we want the LSP to be installed in.                |

> [!NOTE]
> If the `path` does not end in `langugages.toml` it will throw an error.

### Example

You can run the command from helix using `:run-command-shell` or if you prefer in your trusted terminal.

```sh
adn i rust
```

### Config

| Flag                | Descriptión                       |
|---------------------|-----------------------------------|
| `-c` `--config`     | To set the `langauges.toml` path. |
| `-n` `--npm-folder` | To ser the `node_modules/` path.  |

## Features

* [ ] Ability to uninstall LSP from languages
