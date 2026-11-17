use serialport;
mod data_loop;
mod input;

#[tokio::main]
async fn main() {
    let ports = serialport::available_ports().expect("no ports found");

    // check to see if the port exists among available ports if it does we can read from it

    let mut com5 = false;

    for p in ports {
        if p.port_name == "COM5" {
            com5 = true
        } else {}
    }

    if com5 != true {
        println!("Device not found");
        return()
    } else {
        println!("Device found on COM5")
    }

    // external async function for handling reads
    data_loop::data_loop_helper().await.expect("failure reading data from serialport loop");
}
