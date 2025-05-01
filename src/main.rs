use sysinfo::System;

fn main() {
    let mut sys = System::new_all(); // Create instance
    sys.refresh_all(); // refreshing all

    println!("Global CPU usage: {}", sys.global_cpu_usage());

    println!("Total SWAP {} B", sys.total_swap());
    println!("Used SWAP {} B", sys.used_swap());
    println!("Free SWAP {} B", sys.free_swap());

    println!("Total RAM memory: {} B", sys.total_memory());
    println!("Used RAM memory: {} B", sys.used_memory());
    println!("Free RAM memory: {} B", sys.free_memory());
    println!("Number of CPUs: {}", sys.cpus().len());

    for (pid, proc) in sys.processes() {
        println!("Process: {} (PID: {}) - Use of CPU: {}% - Use of memory: {} B",
                 proc.name().to_string_lossy(),
                 pid,
                 proc.cpu_usage(),
                 proc.memory());
    }
}