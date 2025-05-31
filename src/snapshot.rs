use crate::monitor;
use sysinfo::System;
use std::fs::OpenOptions;
use std::fs::File;
use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs;

pub fn snapshot_system(sys: &mut System, timestamp: &str, selected: &Vec<String>) {
    sys.refresh_all();

    // create csv file
    std::fs::create_dir_all("snapshots").unwrap();
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
                let disks_json = monitor::disks_value();
                let parsed_disks: Vec<serde_json::Value> = serde_json::from_str(&disks_json).unwrap_or(vec![]);

                // For each disk, add columns
                for (_i, disk) in parsed_disks.iter().enumerate() {
                    let prefix = format!("Disk");
                    headers.extend([
                        format!("{} Name", prefix),
                        format!("{} Type", prefix),
                        format!("{} FileSystem", prefix),
                        format!("{} Removable", prefix),
                        format!("{} ReadOnly", prefix),
                        format!("{} Total Space", prefix),
                        format!("{} Available Space", prefix),
                    ]);
                    
                    values.extend([
                        disk["name"].to_string().trim_matches('"').to_string(),
                        disk["type"].to_string().trim_matches('"').to_string(),
                        disk["filesystem"].to_string().trim_matches('"').to_string(),
                        disk["removable"].to_string(),
                        disk["readonly"].to_string(),
                        disk["total"].to_string(),
                        disk["available"].to_string(),
                    ]);
                }
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
                let network_json = monitor::networks_value();
                let parsed_networks: Vec<serde_json::Value> = serde_json::from_str(&network_json).unwrap_or(vec![]);

                for (_i, net) in parsed_networks.iter().enumerate() {
                    let prefix = format!("Net");
                    headers.extend([
                        format!("{} Name", prefix),
                        format!("{} Down", prefix),
                        format!("{} Up", prefix),
                    ]);
                    
                    values.extend([
                        net["name"].to_string().trim_matches('"').to_string(),
                        net["down"].to_string(),
                        net["up"].to_string(),
                    ]);
                }
            }
            _ => {}
        }
    }

    // add writer and headers in csv file
    writeln!(writer, "{}", headers.join(","))        .unwrap();
    writeln!(writer, "{}", values.join(","))         .unwrap();
    writer.flush().unwrap();

    // discord bot paths
    let dest_dir = "C:/Users/Usuario/Downloads/sysEye/snapshots";
    std::fs::create_dir_all(dest_dir).unwrap();
    let dest_path = format!("{}/snapshot_{}.csv", dest_dir, timestamp);
    let latest_path = format!("{}/latest.txt", dest_dir);

    // verify if file exists
    if Path::new(&dest_path).exists() {
        fs::remove_file(&dest_path).unwrap();
    }

    fs::copy(&filename, &dest_path).unwrap(); // copy to discord bot
    // send snapshot name to latest.txt
    let mut latest_file = OpenOptions::new().write(true).create(true).truncate(true).open(&latest_path).unwrap();
    writeln!(latest_file, "snapshot_{}.csv", timestamp).unwrap();
}
