use snapshot::snapshot_system;
use sysinfo::System;
use colored::Colorize;
use chrono::Local;
mod monitor;
mod snapshot;

fn main() {
    let mut sys = System::new_all(); // Create instance
    sys.refresh_all(); // refreshing all

    // create and formating date to string
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();

    let selected = select_system(&mut sys);

    snapshot_system(&mut sys, &timestamp, &selected);
}

fn select_system(sys: &mut System) -> Vec<String> {
    // Inquire console
    // Struct multiple options
    let items = vec![
        "System Information",
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
    for item in &select {
        match *item {
            "System Information" => monitor::print_sys_information(),
            "RAM Memory" => monitor::print_ram_memory(sys),
            "SWAP Memory" => monitor::print_swap_memory(sys),
            "CPU" => monitor::print_cpu(sys),
            "Processes" => monitor::print_processes(sys),
            "Disk" => monitor::print_disks(),
            "Network" => monitor::print_networks(),
            _ => println!("Unknown option selected: {}", item)
        }
    }
    select.into_iter().map(|s| s.to_string()).collect()
}