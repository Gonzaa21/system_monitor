use sysinfo::{System, Disks, Networks};
use colored::Colorize;
use serde_json::json;

// for each component, use the print_function (print in console) 
// and the value_function (returns values to generate snapshots)

// information
pub fn sys_information() -> String {
    format!(
        "{}\n{} {}\n{} {}\n{} {}\n{} {}",
        "=> Global system information".bold().underline(),
        "Operating System:".truecolor(64, 64, 64),
        System::name().unwrap_or_else(|| "Desconocido".to_string()),
        "OS Version:".truecolor(64, 64, 64),
        System::os_version().unwrap_or_else(|| "Desconocido".to_string()),
        "Kernel Version:".truecolor(64, 64, 64),
        System::kernel_version().unwrap_or_else(|| "Desconocido".to_string()),
        "Host Name:".truecolor(64, 64, 64),
        System::host_name().unwrap_or_else(|| "Desconocido".to_string()),
    )
}

// ram
pub fn ram_memory(sys: &System) -> String {
    format!(
        "{}\n{}{}{}\n{}{}{}\n{}{}{}\n{}{}{}",
        "=> RAM memory information".bold().underline(),
        "Total RAM: ".truecolor(64, 64, 64), sys.total_memory(), " B".truecolor(64, 64, 64),
        "Used RAM: ".truecolor(64, 64, 64), sys.used_memory(), " B".truecolor(64, 64, 64),
        "Free RAM: ".truecolor(64, 64, 64), sys.free_memory(), " B".truecolor(64, 64, 64),
        "Available RAM: ".truecolor(64, 64, 64), sys.available_memory(), " B".truecolor(64, 64, 64),
    )
}

// swap
pub fn swap_memory(sys: &System) -> String {
    format!(
        "{}\n{}{}{}\n{}{}{}\n{}{}{}",
        "=> SWAP memory information".bold().underline(),
        "Total SWAP: ".truecolor(64, 64, 64), sys.total_swap(), " B".truecolor(64, 64, 64),
        "Used SWAP: ".truecolor(64, 64, 64), sys.used_swap(), " B".truecolor(64, 64, 64),
        "Free SWAP: ".truecolor(64, 64, 64), sys.free_swap(), " B".truecolor(64, 64, 64),
    )
}

// cpu
pub fn cpu(sys: &System) -> String {
    let mut info = format!(
        "{}\n{}{:.2}{}\n{} {}\n",
        "=> CPUs information".bold().underline(),
        "Global CPU usage:".truecolor(64, 64, 64), sys.global_cpu_usage(), " %".truecolor(64, 64, 64),
        "Number of CPUs:".truecolor(64, 64, 64), sys.cpus().len()
    );

    for cpu in sys.cpus() {
        info.push_str(&format!(
            "{} {}\n{}{}{}\n{}{}{}\n",
            "CPU Name:".truecolor(64, 64, 64), cpu.name(),
            "CPU usage: ".truecolor(64, 64, 64), cpu.cpu_usage(), " %".truecolor(64, 64, 64),
            "CPU frequency: ".truecolor(64, 64, 64), cpu.frequency(), " MHz".truecolor(64, 64, 64),
        ));
    }
    info
}

// process
pub fn processes(sys: &System) -> String {
    let mut output = format!("{}\n", "=> Processes information".bold().underline());
    for (pid, process) in sys.processes() {
        output.push_str(&format!(
            "{}\n",
            format!("[{pid}] {:?} {:?}", process.name(), process.disk_usage()).truecolor(64, 64, 64)
        ));
    }
    output
}

// disk
pub fn disks() -> String {
    let disks = Disks::new_with_refreshed_list();
    let mut output = format!("{}\n", "=> Disks information".bold().underline());

    for disk in disks.list() {
        let name = match disk.name().to_str() {
            Some(name) if !name.is_empty() => name,
            _ => "Desconocido",
        };

        output.push_str(&format!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{}{}{}\n{}{}{}\n",
            "Disk Name:".truecolor(64, 64, 64), name,
            "Type:".truecolor(64, 64, 64), disk.kind(),
            "File System:".truecolor(64, 64, 64), disk.file_system().to_string_lossy(),
            "Removable?:".truecolor(64, 64, 64), disk.is_removable(),
            "ReadOnly?:".truecolor(64, 64, 64), disk.is_read_only(),
            "Total size: ".truecolor(64, 64, 64), disk.total_space(), " B".truecolor(64, 64, 64),
            "Available size: ".truecolor(64, 64, 64), disk.available_space(), " B".truecolor(64, 64, 64),
        ));
    }
    output
}

// network
pub fn networks() -> String {
    let networks = Networks::new_with_refreshed_list();
    let mut output = format!("{}\n", "=> Networks information".bold().underline());

    for (name, data) in &networks {
        output.push_str(&format!(
            "{} {} B {} {} B {}\n",
            format!("{name}:").truecolor(64, 64, 64),
            data.total_received(), "(down)".truecolor(64, 64, 64),
            data.total_transmitted(), "(up)".truecolor(64, 64, 64)
        ));
    }
    output
}

// PRINT_FUNCTIONS
pub fn print_sys_information() {
    println!("{}", sys_information());
}
pub fn print_ram_memory(sys: &System) {
    println!("{}", ram_memory(sys));
}
pub fn print_swap_memory(sys: &System) {
    println!("{}", swap_memory(sys));
}
pub fn print_cpu(sys: &System) {
    println!("{}", cpu(sys));
}
pub fn print_processes(sys: &System) {
    println!("{}", processes(sys));
}
pub fn print_disks() {
    println!("{}", disks());
}
pub fn print_networks() {
    println!("{}", networks());
}

// VALUE_FUNCTIONS
pub fn sys_info_values() -> (String, String) {
    let system = System::name().unwrap_or_else(|| "Desconocido".to_string());
    let version = System::os_version().unwrap_or_else(|| "Desconocido".to_string());
    (system, version)
}
pub fn cpu_values(sys: &mut System) -> (f32, usize) {
    sys.refresh_cpu_all();
    let usage = sys.global_cpu_usage();
    let cores = sys.cpus().len();
    (usage, cores)
}
pub fn ram_memory_values(sys: &System) -> (u64, u64, u64, u64) {
    (
        sys.total_memory(),
        sys.used_memory(),
        sys.free_memory(),
        sys.available_memory(),
    )
}
pub fn swap_memory_values(sys: &System) -> (u64, u64, u64) {
    (
        sys.total_swap(),
        sys.used_swap(),
        sys.free_swap(),
    )
}
pub fn disks_value() -> String {
    let disks = Disks::new_with_refreshed_list();
    let info: Vec<_> = disks
        .list()
        .iter()
        .map(|d| {
            json!({
                "name": d.name().to_string_lossy(),
                "type": format!("{:?}", d.kind()),
                "filesystem": d.file_system().to_string_lossy(),
                "removable": d.is_removable(),
                "readonly": d.is_read_only(),
                "total": d.total_space(),
                "available": d.available_space()
            })
        })
        .collect();

    serde_json::to_string(&info).unwrap_or_else(|_| "[]".to_string())
}
pub fn process_count(sys: &System) -> usize {
    sys.processes().len()
}
pub fn networks_value() -> String {
    let networks = Networks::new_with_refreshed_list();
    let network: Vec<_> = networks
        .iter()
        .map(|(name, data)| {
            json!({
                "name": name,
                "down": data.total_received(),
                "up": data.total_transmitted()
            })
        })
        .collect();

    serde_json::to_string(&network).unwrap_or_else(|_| "[]".to_string())
}