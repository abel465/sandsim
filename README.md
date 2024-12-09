# SandSim

Uses
[rust-gpu](https://github.com/Rust-GPU/rust-gpu),
[wgpu](https://github.com/gfx-rs/wgpu), and
[egui](https://github.com/emilk/egui)

## Usage

```bash
nix run github:abel465/sandsim
```

## Development
Shader hot reloading is enabled
```bash
git clone https://github.com/abel465/sandsim.git
cd sandsim
nix develop
cargo run --release
```
