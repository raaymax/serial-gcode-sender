pub fn available_ports() {
    println!("Available ports:");
    for ports in serialport::available_ports().into_iter() {
        for port in ports {
            println!(" - {}", port.port_name);
            if let serialport::SerialPortType::UsbPort(info) = port.port_type {
                println!("\tproduct: {}", info.product.unwrap_or("None".to_string()));
                println!("\tmanufacturer: {}", info.manufacturer.unwrap_or("None".to_string()));
                println!("\tserial: {}", info.serial_number.unwrap_or("None".to_string()));
            }

        }
    }
}
