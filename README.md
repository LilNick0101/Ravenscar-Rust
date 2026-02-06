# Ravenscar Extended Application in Rust

A Rust implementation of the extended application example from [Guide for the use of the Ada Ravenscar Profile in high integrity
systems](https://www.open-std.org/jtc1/sc22/wg9/n575.pdf) made for the Real-time Kernels and Systems class at the Master Degree course in Computer Science at the University of Padua during the accademic year 2023/2024.

The Ravenscar Profile is a subset of the Ada programming language designed for high-integrity and real-time systems. It restricts certain language features to ensure predictability, reliability, and ease of analysis, making it suitable for safety-critical applications.

The extended application example demonstrates the use of the Ravenscar Profile in a simple scenario, involving multiple tasks, protected objects, and inter-task communication, the main work of this project was to translate the Ada code provided in the guide to Rust, using the [`RTIC`](https://rtic.rs/) framework to manage real-time tasks and resources and compare it's performance and behavior with the original Ada implementation, developing an understanding of how Rust can be used in high-integrity and embedded systems.

## Project Structure
```
Ravenscar-Rust/
├── Cargo.toml
├── rust-toolchain.toml
├── memory.x # Linker memory layout for the target
├── README.md
├── src/ # Main source code
│ ├── lib.rs
│ ├── activation_log.rs # activation log implementation
│ ├── activation_manager.rs # activation manager implementation
│ ├── auxiliary.rs # auxiliary utilities
│ ├── constants.rs # project constants
│ ├── production_workload.rs # production workload simulation
│ ├── system_overhead.rs # system overhead simulation
│ └── bin/ # binary entrypoints
│   └── app.rs # main application binary
├── target/ # build output
└── LICENSE-APACHE / LICENSE-MIT
```

## Building and Running
First, make sure you have Rust and the command `rustup` available.
Install your target toolchain depending on your target device, for example, for ARM Cortex-M4 devices, you can use:

```bash
rustup target add thumbv7em-none-eabihf
```
Available targets can be found in the .cargo/config.toml file in the `target` field found in the `[build]` section, make sure to install the appropriate target for your device using `rustup target add <target-name>`.

Then install the required dependencies for building and running the project:

- flip-link:
```bash
cargo install flip-link
```
- probe-rs:
```bash
cargo install probe-rs-tools --locked
```

Then, you can run the project using Cargo:

```bash
 cargo run --bin app
```

To run release build, use:

```bash
 cargo run --release --bin app
```

## Configuration
The project was developed and tested on an _STM32F303VC_ microcontroller unit, other devices and targets can be set up by changing, in the `.cargo/config.toml` file. the `runner` and `target` fields to match your specific hardware and setup.

## License
This project is licensed under the MIT License. See the [LICENSE-MIT](LICENSE-MIT) file for details.