use sysinfo::{System, Components, Disks, Networks};
pub fn sys_information(){
    println!("=> Global system information:");
    println!("Operating System: {}", System::name().unwrap_or_else(|| "Desconocido".to_string()));
    println!("OS Version: {}", System::os_version().unwrap_or_else(|| "Desconocido".to_string()));
    println!("Kernel Version: {}", System::kernel_version().unwrap_or_else(|| "Desconocido".to_string()));
    println!("Host Name: {}", System::host_name().unwrap_or_else(|| "Desconocido".to_string()));
}

pub fn sys_components() {
    let components = Components::new_with_refreshed_list();
    for component in &components {
        println!("{component:?}");
    }
}

pub fn ram_memory(sys: &System){
    println!("=> RAM memory information");
    println!("Total RAM: {} B", sys.total_memory());
    println!("Used RAM: {} B", sys.used_memory());
    println!("Free RAM: {} B", sys.free_memory());
    println!("Free RAM: {} B", sys.available_memory());
}

pub fn swap_memory(sys: &System){
    println!("=> SWAP memory information");
    println!("Total SWAP {} B", sys.total_swap());
    println!("Used SWAP {} B", sys.used_swap());
    println!("Free SWAP {} B", sys.free_swap());
}

pub fn cpu(sys: &System){
    println!("=> CPUs information");
    println!("Global CPU usage: {}", sys.global_cpu_usage());
    println!("Number CPUs: {}", sys.cpus().len());
    for cpu in sys.cpus() {
        println!("CPU Name: {}", cpu.name());
        println!("CPU usage: {}%", cpu.cpu_usage());
        println!("CPU frequency: {} MHz", cpu.frequency());
    }
}

pub fn processes(sys: &System){
    for (pid, process) in sys.processes() {
        println!("=> Processes information");
        println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
    }
}

pub fn disks(){
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {

        let disk_name = match disk.name().to_str() {
            Some(name) if !name.is_empty() => name,
            _ => "Desconocido",
        };
        println!("=> Disks information");
        println!("Disk Name: {}", disk_name);
        println!("Type:{}", disk.kind());
        println!("File System: {}", disk.file_system().to_string_lossy());
        println!("Removable?: {}", disk.is_removable());
        println!("ReadOnly?: {}", disk.is_read_only());
        println!("Total size: {} B", disk.total_space());
        println!("Available size: {} B", disk.available_space());
    }
}

pub fn networks() {
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        println!("=> Networks information");
        println!(
            "{interface_name}: {} B (down) / {} B (up)",
            data.total_received(),
            data.total_transmitted(),
        );
    }
}

// colores
// en los loops process() y cpu() poner un sistema donde guarde esa info aparte
// seguir pasos en readme.md