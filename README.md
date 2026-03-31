# Embassy STM32 Quickstart Template

A automated, [Cargo Generate](https://cargo-generate.github.io/cargo-generate/)  template for bootstrapping embedded Rust projects using [Embassy](https://embassy.dev/) on STM32 microcontrollers.

Instead of manually configuring your `.cargo/config.toml` runners, looking up memory addresses, or juggling Cargo features, this template asks you a few simple questions and sets everything up for you instantly.

## ✨ Features

* **Instant `probe-rs` Setup:** Automatically wires up `.cargo/config.toml` with the correct target architecture and `probe-rs` chip string using a unified `cfg` block. Just hit `cargo run` to flash and run.
* **Smart `memory.x` Handling:** Choose between letting `embassy-stm32` automatically handle your memory layout via the `memory-x` feature, or automatically generate a custom `memory.x` file based on your Flash and RAM sizes.
* **Pre-configured Logging:** Defmt logging and RTT are wired up and ready to go out of the box.
* **Linker Scripts:** Automatically passes the correct linker arguments for your target.

## 🛠️ Prerequisites

Before generating a project, make sure you have the required embedded tooling installed:

1. **cargo-generate:**

   ```bash
   cargo install cargo-generate
   ```

2. Probe-rs

```bash
cargo install probe-rs-tools
```

1. Rust Target: Make sure you have the target architecture for your STM32 installed (e.g., thumbv7em-none-eabihf for Cortex-M4F/M7F).

```bash
rustup target add <your-target-architecture>
```

## Usage

To create a new project run:

```bash
cargo generate --git [https://github.com/YOUR_USERNAME/YOUR_REPO_NAME](https://github.com/YOUR_USERNAME/YOUR_REPO_NAME)
```

## The Prompts

The generator will ask you a few questions to perfectly tailor the project to your hardware:

1. Project Name: The name of your new crate.

2. Embassy MCU Feature: The exact lowercase family name for embassy-stm32 (e.g., stm32f413zh).

3. Probe-rs Chip Name: The exact, case-sensitive chip name for the probe-rs run command (e.g., STM32F413ZHTx).

4. Target Architecture: Your Rust compilation target (e.g., thumbv7em-none-eabihf).

5. Custom memory.x?: * Select false to automatically use Embassy's built-in memory mapping.

* Select true to generate a custom `memory.x` file. You will be prompted for your `Flash` and `RAM` sizes (e.g., 1536K and 320K).

## What gets generated?

Your new project will look like this

my-stm32-project/
├── .cargo/
│   └── config.toml      # Configured with your target and probe-rs chip
├── src/
│   └── main.rs          # Basic async Embassy blinky/logging boilerplate
├── build.rs             # Linker script wiring
├── Cargo.toml           # Configured with your specific STM32 feature flags
├── memory.x             # (Optional) Generated if you opted for custom memory
