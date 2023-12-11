# protonctl-rs
![ProtonCTL_multi_ani](https://github.com/The-Corrupted/protonctl-rs/assets/27307991/80eb7cd8-900a-4e07-8856-9ac8fec1c163)

A proton-ge and wine-ge manager written in Rust and modelled after the ctl set of tools.
## Build Instructions
Clone the repository:
```
git clone https://github.com/The-Corrupted/protonctl-rs.git
```
From the root of the project run:
```
cargo build --release && sudo mv ./target/release/protonctl-rs /usr/bin/protonctl
```
This will compile the project and move the executable into your path.
## Usage
List remote wine-ge releases:
```
protonctl list wine
```
Remove local wine-ge install:
```
protonctl remove lutris-GE-Proton8-25-x86_64 wine
```
Install wine-ge version:
```
protonctl install GE-Proton8-25 wine
```
