use sysinfo::System;
mod monitor;
use colored::Colorize;

fn main() {
    select_system();
}

fn select_system() {
    let mut sys = System::new_all(); // Create instance
    sys.refresh_all(); // refreshing all

    // Inquire console
    // Struct multiple options
    let items = vec![
        "System Information",
        "Components",
        "RAM Memory",
        "SWAP Memory",
        "CPU",
        "Processes",
        "Disk",
        "Network"
    ];

    // messages
    let msg = "Please select some of these options";
    let expect = "Failed to select option".red().to_string();
    
    // create instance
    let select = inquire::MultiSelect::new(msg, items)
        .prompt()
        .expect(expect.as_str());

    // match options
    for item in select {
        match item {
            "System Information" => monitor::sys_information(),
            "Components" => monitor::sys_components(),
            "RAM Memory" => monitor::ram_memory(&sys),
            "SWAP Memory" => monitor::swap_memory(&sys),
            "CPU" => monitor::cpu(&sys),
            "Processes" => monitor::processes(&sys),
            "Disk" => monitor::disks(),
            "Network" => monitor::networks(),
            _ => println!("Unknown option selected: {}", item)
        }
    }
}