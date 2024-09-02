use serialport::{available_ports, SerialPortType};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // /// Name of the person to greet
    // #[arg(short, long)]
    // name: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,

    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    GetPath {
        #[arg(short, long)]
        sn: String
    },
    ListAll
}

fn get_port_by_sn(sn: String) {
    let ports = available_ports().unwrap();
    for port in ports {
        match port.port_type {
            SerialPortType::UsbPort(info) => {
                if info.serial_number == Some(sn.clone()) {
                    #[cfg(target_os = "macos")]
                    if port.port_name.starts_with("/dev/cu") {
                        println!("{}", port.port_name);
                    }
                    #[cfg(target_os = "linux")]
                    println!("{}", port.port_name);
                    #[cfg(not(target_os = "linux"))]
                    println!("Unsupported operating system");
                    return;
                }
            }
            _ => {}
        }
    }
    println!("No port found with serial number: {}", sn);
}

fn list_all() {
    match available_ports() {
        Ok(ports) => {
            match ports.len() {
                0 => println!("No ports found."),
                1 => println!("Found 1 port:"),
                n => println!("Found {} ports:", n),
            };
            for p in ports {
                println!("  {}", p.port_name);
                match p.port_type {
                    SerialPortType::UsbPort(info) => {
                        println!("    Type: USB");
                        println!("    VID:{:04x} PID:{:04x}", info.vid, info.pid);
                        println!(
                            "     Serial Number: {}",
                            info.serial_number.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "      Manufacturer: {}",
                            info.manufacturer.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "           Product: {}",
                            info.product.as_ref().map_or("", String::as_str)
                        );
                        // #[cfg(feature = "usbportinfo-interface")]
                        // println!(
                        //     "         Interface: {}",
                        //     info.interface
                        //         .as_ref()
                        //         .map_or("".to_string(), |x| format!("{:02x}", *x))
                        // );
                    }
                    SerialPortType::BluetoothPort => {
                        println!("    Type: Bluetooth");
                    }
                    SerialPortType::PciPort => {
                        println!("    Type: PCI");
                    }
                    SerialPortType::Unknown => {
                        println!("    Type: Unknown");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
            eprintln!("Error listing serial ports");
        }
    }
}


fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::GetPath{sn} => get_port_by_sn(sn),
        Commands::ListAll => list_all()
        
    }
    
    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }
}

// fn main() {
//     match available_ports() {
//         Ok(ports) => {
//             match ports.len() {
//                 0 => println!("No ports found."),
//                 1 => println!("Found 1 port:"),
//                 n => println!("Found {} ports:", n),
//             };
//             for p in ports {
//                 println!("  {}", p.port_name);
//                 match p.port_type {
//                     SerialPortType::UsbPort(info) => {
//                         println!("    Type: USB");
//                         println!("    VID:{:04x} PID:{:04x}", info.vid, info.pid);
//                         println!(
//                             "     Serial Number: {}",
//                             info.serial_number.as_ref().map_or("", String::as_str)
//                         );
//                         println!(
//                             "      Manufacturer: {}",
//                             info.manufacturer.as_ref().map_or("", String::as_str)
//                         );
//                         println!(
//                             "           Product: {}",
//                             info.product.as_ref().map_or("", String::as_str)
//                         );
//                         #[cfg(feature = "usbportinfo-interface")]
//                         println!(
//                             "         Interface: {}",
//                             info.interface
//                                 .as_ref()
//                                 .map_or("".to_string(), |x| format!("{:02x}", *x))
//                         );
//                     }
//                     SerialPortType::BluetoothPort => {
//                         println!("    Type: Bluetooth");
//                     }
//                     SerialPortType::PciPort => {
//                         println!("    Type: PCI");
//                     }
//                     SerialPortType::Unknown => {
//                         println!("    Type: Unknown");
//                     }
//                 }
//             }
//         }
//         Err(e) => {
//             eprintln!("{:?}", e);
//             eprintln!("Error listing serial ports");
//         }
//     }
// }
