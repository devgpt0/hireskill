# Tauri + React + Typescript

This template should help get you started developing with Tauri 2.0, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Pre-requisite

Before working on this project you need to check the pre-requiste in your system

Rust toolchain (rustup) and a JS package manager (npm / pnpm / yarn).

OS deps:

Windows: MSVC Build Tools + WebView2 Runtime

macOS: Xcode Command Line Tools

Linux: webkit2gtk & friends
Official checklist here.

Quick sanity check:
```bash
rustup --version
cargo --version
node -v
npm -v
```
## To Run and Build the application

Command to build the application:
```bash
npm run tauri build
```

Command to run the application:
```bash
npm run tauri dev
```