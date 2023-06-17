# CONFU3U

`version: 0.1.0`

An app to install, uninstall, and configure _Ubuntu Desktop_ software

## Development

### Project Directories:

| Directory | Type      | Description                                      |
| --------- | --------- | ------------------------------------------------ |
| rs_dev    | Rust      | Main development crate used to test other crates |
| rs_tauri  | Rust      | Main _Tauri_ crate                               |
| src       | SvelteKit | Main _SvelteKit_ frontend code                   |

### Tauri and SvelteKit Development:

Make sure all the [Tauri](https://tauri.app) and [SvelteKit](https://kit.svelte.dev) dependencies are met. Additionally, [yarn](https://classic.yarnpkg.com/lang/en/) is the _Node_ dependency manager that the project is configured to use.

The following script commands are available:

| Shell Command      | Description                                                                     |
| ------------------ | ------------------------------------------------------------------------------- |
| `yarn dev`         | Starts the **_SvelteKit_** frontend on localhost port `5173`                    |
| `yarn tauri dev`   | Starts the **_Tauri_** backend and **_SvelteKit_** frontend in development mode |
| `yarn tauri build` | Compiles the application for distribution                                       |

### Rust Crate Development:

A `justfile` is included to simplify launching Rust development scripts. In order to make use of the **_just_** scripts, the following crates should be installed globally:

| Crate                                                   | Installation                  |
| ------------------------------------------------------- | ----------------------------- |
| [just](https://crates.io/crates/just)                   | `cargo install just`          |
| [cargo-watch](https://crates.io/crates/cargo-watch)     | `cargo install cargo-watch`   |
| [cargo-expand](https://crates.io/crates/cargo-expand)   | `cargo install cargo-expand`  |
| [cargo-modules](https://crates.io/crates/cargo-modules) | `cargo install cargo-modules` |

With the prerequisites being met, the following most common `just` script commands are availble:

| Shell Command | Description                                        |
| ------------- | -------------------------------------------------- |
| `just build`  | Build all crates                                   |
| `just dev`    | Run the `rs_dev` crate and watch for changes       |
| `just mods`   | Generate a module tree of all the different crates |
| `just expand` | Generate an expanded macro output from all modules |

In addition, there are derivatives of the above commands for different crates. To see a full list of available _just_ scripts, run `just --list` in the terminal.
