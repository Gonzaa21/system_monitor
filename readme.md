# System Monitor
With this project you can collect information about RAM, SWAP and more CPU specs. The program pick the data and creates a single snapshot to save it and send to sysEye discord bot.

mod.rs - for each component have the print function and value function
main.rs - main code, console UI and calls from funcions of mod.rs and snapshot.rs
snapshot.rs - create a csv file with previously collected information
Cargo.toml - Dependencies and crates
> sysEye bot repository [here](https://github.com/Gonzaa21/sysEye-Bot)

# How it works
1. In `system_monitor` project send:
   ```rs
   cargo run
   ```
2. Select the components to collect information
3. In terminal, shows you the selected data
4. Create a snapshot and send it to `sysEye` project
5. In the project:
   ```py
   py main.py
   ```
6. Create visual graphs and send the dashboard to channel