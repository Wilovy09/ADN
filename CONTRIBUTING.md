# How to contribute to the project

> [!NOTE]
> Your PR has to go to the `develop` branch.

## Add support to an LSP

> [!TIP]
> You can take inspiration from an existing LSP in the project.

- You have to create a folder with the name of the language that is supported by the LSP.
- Inside this folder you will put a `mod.rs` file with your functions.
- Then add the folder to `langauages/mod.rs` so you can use it in `src/main.rs`.
- When adding it to the user configuration, use the existing function in the `utilities/conbfig_managment.rs` file.
- Make the necessary changes to the `src/main.rs` so that when using the command it will work.
- Run `cargo fmt --all`.
- Run `cargo clippy -- -D warnigs` and fix the warnings/errors that clippy throws.

## Improve the code

- Just make the changes and in your PR explain why it is better.
- Run `cargo fmt --all`.
- Run `cargo clippy -- -D warnigs` and fix the warnings/errors that clippy throws.

## Take into account

If we make both changes, improve code and add support for an LSP.

Please do it in separate PRs explaining what you did.

