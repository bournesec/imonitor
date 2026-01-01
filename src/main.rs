use std::{
    io::{self, Write},
    str,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use anyhow::{Context, Result};
use pcap::{Capture, Device};
use structopt::StructOpt;

mod stats;
use crate::stats::traffic_stats::TrafficStats;

#[derive(StructOpt, Debug)]
#[structopt(name = "imonitor", about = "Real-time network traffic monitor")]
struct Opt {
    #[structopt(short = "i", long = "interface", default_value = "eth0")]
    interface: String,

    #[structopt(short = "u", long = "update-interval", default_value = "1000")]
    update_interval: u64,

    #[structopt(short = "l", long = "list-interfaces")]
    list_interfaces: bool,

    #[structopt(short = "f", long = "filter")]
    filter: Option<String>,
}

fn list_interfaces() -> Result<()> {
    println!("Available network interfaces:");
    println!("{:<20} {:<30} {:<40}", "Name", "IP Address", "Description");
    println!("{}", "-".repeat(80));

    for device in Device::list().unwrap_or_default() {
        let ip = if let Some(addr) = device.addresses.last() {
            addr.addr.to_string()
        } else {
            "N/A".to_string()
        };

        println!(
            "{:<20} {:<30} {:<40}",
            device.name,
            ip,
            device.desc.unwrap_or_else(|| "No description".to_string())
        );
    }

    Ok(())
}

fn monitor_interface(interface: &str, update_interval: u64, filter: Option<&str>) -> Result<()> {
    let interfaces = Device::list().context("Failed to list interfaces")?;

    interfaces
        .into_iter()
        .find(|i| i.name == interface)
        .context(format!("Interface '{}' not found", interface))?;

    let mut cap = Capture::from_device(interface)
        .context(format!("Failed to create capture for '{}'", interface))?
        .promisc(true)
        .immediate_mode(true)
        .timeout(100)
        .open()
        .context(format!("Failed to open interface '{}'", interface))?;

    if let Some(filter_str) = filter {
        cap.filter(filter_str, true)
            .context(format!("Failed to set filter for '{}'", interface))?;
        println!("Filter: {}", filter_str);
    }

    println!("Monitoring interface: {}", interface);

    println!("{:-^63}", "");
    println!(
        "{:<12} {:<12} {:<12} {:<12} {:<12}",
        "Time", "Packets/s", "Bytes/s", "Total Pkts", "Total Bytes"
    );
    println!("{:-^63}", "");

    let stats = Arc::new(Mutex::new(TrafficStats::new()));
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .context("Error setting Ctrl-C handler")?;

    let stats_for_update = Arc::clone(&stats);

    let update_thread = thread::spawn({
        let running = running.clone();
        let update_interval = update_interval;

        move || {
            while running.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(update_interval));

                let (pps, bps, packets, bytes) = stats_for_update.lock().unwrap().reset();
                let time_str = chrono::Local::now().format("%H:%M:%S").to_string();

                let pps_str = if pps >= 1_000_000.0 {
                    format!("{:.2}Mpps", pps / 1_000_000.0)
                } else if pps >= 1_000.0 {
                    format!("{:.2}Kpps", pps / 1_000.0)
                } else {
                    format!("{:.2}pps", pps)
                };

                let bps_str = if bps >= 1_000_000_000.0 {
                    format!("{:.2}GB/s", bps / 1_000_000_000.0)
                } else if bps >= 1_000_000.0 {
                    format!("{:.2}MB/s", bps / 1_000_000.0)
                } else if bps >= 1_000.0 {
                    format!("{:.2}KB/s", bps / 1_000.0)
                } else {
                    format!("{:.2}B/s", bps)
                };

                println!(
                    "\r{:<12} {:<12} {:<12} {:<12} {:<12}",
                    time_str, pps_str, bps_str, packets, bytes
                );
                io::stdout().flush().unwrap();
            }
        }
    });

    while running.load(Ordering::SeqCst) {
        match cap.next_packet() {
            Ok(packet) => {
                stats
                    .lock()
                    .unwrap()
                    .add_packet(packet.header.caplen as usize);
            }
            Err(pcap::Error::TimeoutExpired) => {
                continue;
            }
            Err(e) => {
                eprintln!("Error reading packet: {}", e);
                running.store(false, Ordering::SeqCst);
            }
        }
    }

    update_thread.join().unwrap_or_default();

    let (total_packets, total_bytes) = stats.lock().unwrap().get_total();
    let elapsed = stats.lock().unwrap().start_time.elapsed().as_secs_f64();

    println!("\n\nFinal Statistics:");
    println!("Interface: {}", interface);
    println!("Monitoring duration: {:.2} seconds", elapsed);
    println!("Total packets: {}", total_packets);
    println!("Total bytes: {}", total_bytes);
    println!("Average packets/s: {:.2}", total_packets as f64 / elapsed);
    println!("Average bytes/s: {:.2}", total_bytes as f64 / elapsed);

    Ok(())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    if opt.list_interfaces {
        return list_interfaces();
    }

    monitor_interface(&opt.interface, opt.update_interval, opt.filter.as_deref())
}
