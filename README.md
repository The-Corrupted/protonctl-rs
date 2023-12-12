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
protonctl wine list
```
Remove local wine-ge install:
```
protonctl wine remove lutris-GE-Proton8-25-x86_64
```
Install wine-ge version:
```
protonctl wine install GE-Proton8-25
```
## Todo:
Code cleanup - The code is still a mess. The run function should be broken up so it's not responsible for doing everything.

Profiling - I've done some basic profiling and while certain things are about as fast and use as much memory as I would expect them to, others are still quite slow compared to similar managers. Checking the file integrity takes far longer than I would expect and is worth looking into. Additionally, we sit at around 14-25mb of memory usage whereas other tools like protonup sit at around 8-15mb. I would like to investigate what's eating up so much memory, especially early on.

xz2 alternative - It's not clear to me if xz decompressing is slow and memory hungry by nature but decompressing using XzDecode takes ages and uses a ton of memory whereas GzDecode is relatively quick and only adds an extra megabyte to our memory usage vs the 8+ xz adds. It may be worth looking into some of the other crates to see if this is "a problem" across the board.

Autocomplete - Most cli applications have autocomplete. This does not at the moment. It would be great to get this added. It looks like clap_complete might be a good place to start.

Man pages - While this isn't a particularly complicated tool to use, it would be nice to have man pages for the users that want or need that.

Proper output formatting/handling - console does a pretty good job of detecting if we're piping the output or running through a terminal that doesn't support colored output but there are instances where we seem to break things. One such instance is if we try to run install and pipe to a file. stderr is still written to but the formatting is all screwed up. Additionally, the output for list local is quite bad ( only list 3 items per row and the columns aren't aligned ). It would be nice to fix this so it looks a bit more like ls output.

Flatpack support - There is no flatpack support at the moment.

Custom installs - There is no support for custom installs at the moment. Currently only wine-ge-custom and proton-ge-custom are supported.

User configuration - There is no support for user configuration of colors or custom install locations at the moment. Wine and proton builds will be installed into the lutris and proton compatibility/runner directories respectively.

