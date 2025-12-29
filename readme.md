<div align="center">
    <img src="snowstrike.gif">
</div>

## credits

- [newt](https://newty.dev) - programming
- [sam](https://github.com/samuelhorobin) - concept, artwork

## development

### windows

for improved compilation times, we use the [lld](https://lld.llvm.org/) linker. you can install it by running:

```bash
cargo install -f cargo-binutils
rustup component add llvm-tools
```

### macos

for improved compilation times, we use the [lld](https://lld.llvm.org/) linker. you can install it by running:

```bash
brew install llvm
```

### linux

for improved compilation times, we use the [mold](https://github.com/rui314/mold) linker. depending on your distribution, you can install it by running:

```bash
sudo apt install mold clang # debian/ubuntu
sudo dnf install mold clang # fedora
sudo pacman -S mold clang   # arch
```