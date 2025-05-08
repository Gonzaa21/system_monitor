use sysinfo::{System, Components, Disks, Networks};
use colored::Colorize;

pub fn sys_information(){
    println!("{}","=> Global system information".bold().underline());
    println!("{} {}","Operating System:".truecolor(64, 64, 64), System::name().unwrap_or_else(|| "Desconocido".to_string()));
    println!("{} {}","OS Version:".truecolor(64, 64, 64), System::os_version().unwrap_or_else(|| "Desconocido".to_string()));
    println!("{} {}","Kernel Version:".truecolor(64, 64, 64), System::kernel_version().unwrap_or_else(|| "Desconocido".to_string()));
    println!("{} {}","Host Name:".truecolor(64, 64, 64), System::host_name().unwrap_or_else(|| "Desconocido".to_string()));
}

pub fn sys_components() {
    let components = Components::new_with_refreshed_list();
    for component in &components {
        println!("{component:?}");
    }
}

pub fn ram_memory(sys: &System){
    println!("{}","=> RAM memory information".bold().underline());
    println!("{}", format!("Total RAM: {} B", sys.total_memory()).truecolor(64, 64, 64));
    println!("{}", format!("Used RAM: {} B", sys.used_memory()).truecolor(64, 64, 64));
    println!("{}", format!("Free RAM: {} B", sys.free_memory()).truecolor(64, 64, 64));
    println!("{}", format!("Available RAM: {} B", sys.available_memory()).truecolor(64, 64, 64));
}

pub fn swap_memory(sys: &System){
    println!("{}","=> SWAP memory information".bold().underline());
    println!("{}", format!("Total SWAP {} B", sys.total_swap()).truecolor(64, 64, 64));
    println!("{}", format!("Used SWAP {} B", sys.used_swap()).truecolor(64, 64, 64));
    println!("{}", format!("Free SWAP {} B", sys.free_swap()).truecolor(64, 64, 64));
}

pub fn cpu(sys: &System){
    let cpu_count = sys.cpus().len(); 

    println!("{}","=> CPUs information".bold().underline());
    println!("{} {}", "Global CPU usage:".truecolor(64, 64, 64), format!("{:.2}%", sys.global_cpu_usage()).truecolor(64, 64, 64));
    println!("{} {}", "Number of CPUs:".truecolor(64, 64, 64), cpu_count.to_string().truecolor(64, 64, 64));

    for cpu in sys.cpus() {
        println!("{} {}","CPU Name:".truecolor(64, 64, 64), cpu.name());
        println!("{}", format!("CPU usage: {}%", cpu.cpu_usage()).truecolor(64, 64, 64));
        println!("{}", format!("CPU frequency: {} MHz", cpu.frequency()).truecolor(64, 64, 64));
    }
}

pub fn processes(sys: &System){
    for (pid, process) in sys.processes() {
        println!("{}","=> Processes information".bold().underline());
        println!("{}", format!("[{pid}] {:?} {:?}", process.name(), process.disk_usage()).truecolor(64, 64, 64));
    }
}

pub fn disks(){
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {

        let disk_name = match disk.name().to_str() {
            Some(name) if !name.is_empty() => name,
            _ => "Desconocido",
        };
        println!("{}","=> Disks information".bold().underline());
        println!("{} {}","Disk Name:".truecolor(64, 64, 64), disk_name);
        println!("{} {}","Type:".truecolor(64, 64, 64), disk.kind());
        println!("{} {}","File System:".truecolor(64, 64, 64), disk.file_system().to_string_lossy());
        println!("{} {}","Removable?:".truecolor(64, 64, 64), disk.is_removable());
        println!("{} {}","ReadOnly?:".truecolor(64, 64, 64), disk.is_read_only());
        println!("{}", format!("Total size: {} B", disk.total_space()).truecolor(64, 64, 64));
        println!("{}", format!("Available size: {} B", disk.available_space()).truecolor(64, 64, 64));
    }
}

pub fn networks() {
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        println!("{}","=> Networks information".bold().underline());
        println!("{}", format!("{interface_name}: {} B (down) / {} B (up)", data.total_received(), data.total_transmitted()).truecolor(64, 64, 64));
    }
}

// en los loops process() y cpu() poner un sistema donde guarde esa info aparte
// seguir pasos en readme.md