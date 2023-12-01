# protonctl-rs
Proton-GE manager written in Rust and modelled after the ctl set of utilities.

# Example usage:

protonctl wine list -n 8 -p 10 -- Lists 8 wine-ge entries from the 10th page.

protonctl proton list -l -- List all local proton installs.

protonctl wine install Wine-GE-Proton8-24 -- Install wine-ge-lutris v. 8-24

protonctl remove -a -- Remove all currently installed proton versions.

protonctl remove -c -- Remove any downloaded files that weren't cleaned up at the end of an install.

# TODO
* The code needs to be cleaned up. The goal was to get something working. Now that it's working, things need to be better organized and the structs used/means of passing data around needs to be looked at again.
* Better error handling. I'm converting reqwest errors to anyhow errors and I've not been using anyhows Context. This means the returning of errors is verbose and likely incorrect/inefficient. This needs a second look.
* Take a look at async. It's been stripped out because it wasn't needed at the time and I've since moved to reqwests blocking api to avoid needing to pull in tokio. Determine if we want to use async or if the blocking api is sufficient.
* We want positional args that don't require a flag: i.e. protonctl install GE-Proton8-20. Right now a flag is required.
* Add Wine-GE support. I initially wrote this to handle proton installs since I used steam primarily. I've since moved to GoG/Lutris and would like the manager to support Wine-Ge installs.
* A long term goal for the project is to also have an interactive prompt, similar to protonup-rs, for managing installs. I would like to use ratatui for this. Once the api has stabilized and the code has been cleaned up, I would like to start work on a TUI.
* Split the library code into its own workspace.
* I would like to add colors, progressbars, etc to the cli output.
