use sysinfo::System;
mod monitor;

fn main() {
    let mut sys = System::new_all(); // Create instance
    sys.refresh_all(); // refreshing all

    monitor::sys_information();
    monitor::sys_components();
    monitor::ram_memory(&sys);
    monitor::swap_memory(&sys);
    monitor::cpu(&sys);
    monitor::processes(&sys);
    monitor::disks();
    monitor::networks();
}