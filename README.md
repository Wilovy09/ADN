<div align="center">
<a href="https://github.com/Wilovy09/ADN">
<img src="./static/images/readme.png">
</a>
<h1>Facilitate the installation of LSPs in <a href="https://helix-editor.com/" target="_blank">Helix editor</a>.</h1>
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
adn PATH ACTION LANGUAGE
```

| Arg      | Description                                                                           |
|----------|---------------------------------------------------------------------------------------|
| PATH     | We will replace this part with the location of our `languages.toml` file.             |
| ACTION   | In this part we will put the action that we want to do `install` or (`uninstall` WIP) |
| LANGUAGE | Here we will put the name of the language we want the LSP to be installed in.         |

> [!NOTE]
> If the `path` does not end in `langugages.toml` it will throw an error.

### Example

You can run the command from helix using `:run-command-shell` or if you prefer in your trusted terminal.

```sh
adn ~/.config/helix/languges.toml install rust
```

## Features

* [ ] Ability to uninstall LSP from languages
