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
    println!("{}{}{}", "Total RAM: ".truecolor(64, 64, 64), sys.total_memory(), " B".truecolor(64, 64, 64));
    println!("{}{}{}", "Used RAM: ".truecolor(64, 64, 64), sys.used_memory(), " B".truecolor(64, 64, 64));
    println!("{}{}{}", "Free RAM: ".truecolor(64, 64, 64), sys.free_memory(), " B".truecolor(64, 64, 64));
    println!("{}{}{}", "Available RAM: ".truecolor(64, 64, 64), sys.available_memory(), " B".truecolor(64, 64, 64));
}

pub fn swap_memory(sys: &System){
    println!("{}","=> SWAP memory information".bold().underline());
    println!("{}{}{}", "Total SWAP: ".truecolor(64, 64, 64), sys.total_swap(), " B".truecolor(64, 64, 64));
    println!("{}{}{}", "Used SWAP: ".truecolor(64, 64, 64), sys.used_swap(), " B".truecolor(64, 64, 64));
    println!("{}{}{}", "Free SWAP: ".truecolor(64, 64, 64), sys.free_swap(), " B".truecolor(64, 64, 64));
}

pub fn cpu(sys: &System){
    let cpu_count = sys.cpus().len(); 

    println!("{}","=> CPUs information".bold().underline());
    println!("{}{:.2}{}", "Global CPU usage:".truecolor(64, 64, 64), sys.global_cpu_usage(), " %".truecolor(64, 64, 64));
    println!("{} {}", "Number of CPUs:".truecolor(64, 64, 64), cpu_count.to_string().truecolor(64, 64, 64));

    for cpu in sys.cpus() {
        println!("{} {}","CPU Name:".truecolor(64, 64, 64), cpu.name());
        println!("{}{}{}", "CPU usage: ".truecolor(64, 64, 64), cpu.cpu_usage(), " %".truecolor(64, 64, 64));
        println!("{}{}{}", "CPU frequency: ".truecolor(64, 64, 64), cpu.frequency(), " MHz".truecolor(64, 64, 64));
    }
}

pub fn processes(sys: &System){
    println!("{}","=> Processes information".bold().underline());
    for (pid, process) in sys.processes() {
        println!("{}", format!("[{pid}] {:?} {:?}", process.name(), process.disk_usage()).truecolor(64, 64, 64));
    }
}

pub fn disks(){
    let disks = Disks::new_with_refreshed_list();
    println!("{}","=> Disks information".bold().underline());

    for disk in disks.list() {
        let disk_name = match disk.name().to_str() {
            Some(name) if !name.is_empty() => name,
            _ => "Desconocido",
        };
        println!("{} {}","Disk Name:".truecolor(64, 64, 64), disk_name);
        println!("{} {}","Type:".truecolor(64, 64, 64), disk.kind());
        println!("{} {}","File System:".truecolor(64, 64, 64), disk.file_system().to_string_lossy());
        println!("{} {}","Removable?:".truecolor(64, 64, 64), disk.is_removable());
        println!("{} {}","ReadOnly?:".truecolor(64, 64, 64), disk.is_read_only());
        println!("{}{}{}", "Total size: ".truecolor(64, 64, 64), disk.total_space(), " B".truecolor(64, 64, 64));
        println!("{}{}{}", "Available size: ".truecolor(64, 64, 64), disk.available_space(), " B".truecolor(64, 64, 64));
    }
}

pub fn networks() {
    let networks = Networks::new_with_refreshed_list();
    println!("{}","=> Networks information".bold().underline());
    for (interface_name, data) in &networks {
        println!(
            "{} {} B {} {} B {}", 
            format!("{interface_name}:").truecolor(64, 64, 64),
            data.total_received(),"(down)".truecolor(64, 64, 64),
            data.total_transmitted(),"(up)".truecolor(64, 64, 64));
    }
}