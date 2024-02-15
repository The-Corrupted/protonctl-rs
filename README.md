# protonctl-rs
![ProtonCTL_multi_ani](https://github.com/The-Corrupted/protonctl-rs/assets/27307991/80eb7cd8-900a-4e07-8856-9ac8fec1c163)

A proton and wine manager written in Rust.
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
protonctl list -t wine
```
Remove local wine-ge install:
```
protonctl remove -t wine lutris-GE-Proton8-25-x86_64
```
Install wine-ge version:
```
protonctl install -t wine GE-Proton8-25
```
## Todo:
Man pages - While this isn't a particularly complicated tool to use, it would be nice to have man pages for the users that want or need that.

Proper output formatting/handling - console does a pretty good job of detecting if we're piping the output or running through a terminal that doesn't support colored output but there are instances where we seem to break things. One such instance is if we try to run install and pipe to a file. stderr is still written but we end up with the progress bar ending up in the file as a string of escape codes

Flatpack support - There is no flatpack support at the moment.

Custom installs - There is no support for custom installs at the moment. Currently only wine-ge-custom and proton-ge-custom are supported.

User configuration - There is no support for user configuration of colors or custom install locations at the moment. Wine and proton builds will be installed into the lutris and proton compatibility/runner directories respectively.

