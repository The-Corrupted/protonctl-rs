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
### Todo:
Code cleanup - The code is still a mess. The run function should be broken down so it's not responsible for doing everything.

Profiling - I've done some basic profiling and while certain things are about as fast and use as much memory as I would expect them to, certain things are still quite slow compared to other, similar managers. Checking the hash takes far longer than I would expect and is worth looking into. Additionally, we sit at around 14-25mb of memory usage whereas other tools like protonup sit at around 8-15mb. I would like to investigate what's eating up so much memory, especially early on.

xz2 alternative - It's not clear to me if xz decompressing is slow and memory hungry by nature but decompressing using XzDecode takes ages and uses a ton of memory whereas GzDecode is relatively quick and only adds an extra megabyte to our memory usage vs the 8+ xz adds. It may be worth looking into some of the other crates to see if this is "a problem" across the board.

