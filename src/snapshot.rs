use crate::monitor;
use sysinfo::System;
use std::fs::File;
use std::io::{Write, BufWriter};

pub fn snapshot_system(sys: &mut System, timestamp: &str, selected: &Vec<String>) {
    sys.refresh_all();

    // create csv file
    let filename = format!("snapshots/snapshot_{}.csv", timestamp);
    let file = File::create(&filename).unwrap();
    let mut writer = BufWriter::new(file);

    // dinamic header
    let mut headers = vec!["timestamp".to_string()];
    let mut values = vec![timestamp.to_string()];

    // compare options selected
    for opt in selected {
        match opt.as_str() {
            "System Information" => {
                let (os, version) = monitor::sys_info_values();
                headers.push("Operating System".to_string());
                headers.push("OS Version".to_string());
                values.push(os);
                values.push(version);
            }
            "RAM Memory" => {
                let (total, used, free, available) = monitor::ram_memory_values(sys);
                headers.extend(["Total RAM", "Used RAM", "Free RAM", "Available RAM"].iter().map(|s| s.to_string()));
                values.extend([total, used, free, available].iter().map(|v| v.to_string()));
            }
            "SWAP Memory" => {
                let (total, used, free) = monitor::swap_memory_values(sys);
                headers.extend(["Total SWAP", "Used SWAP", "Free SWAP"].iter().map(|s| s.to_string()));
                values.extend([total, used, free].iter().map(|v| v.to_string()));
            }
            "Disk" => {
                let disks = monitor::disks_value();
                headers.push("Disks".to_string());
                values.push(disks);
            }
            "CPU" => {
                let (usage, cores) = monitor::cpu_values(sys);
                headers.extend(["Global CPU Usage", "Number of CPUs"].iter().map(|s| s.to_string()));
                values.extend([format!("{:.2}", usage), cores.to_string()]);
            }
            "Processes" => {
                let count = monitor::process_count(sys);
                headers.push("Number of Processes".to_string());
                values.push(count.to_string());
            }
            "Network" => {
                let data = monitor::networks_value();
                headers.push("Network Usage".to_string());
                values.push(data);
            }
            _ => {}
        }
    }

    // add writer and headers in csv file
    writeln!(writer, "{}", headers.join(","))        .unwrap();
    writeln!(writer, "{}", values.join(","))         .unwrap();
}
