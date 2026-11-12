use serialport;
use tokio_serial;

mod data_loop;

fn main() {
    let ports = serialport::available_ports().expect("no ports found");

    // check to see if the port exists among available ports if it does we can read from it

    let mut com5 = false;

    for p in ports {
        if p.port_name == "COM5" {
            com5 = true
        } else {}
    }

    if com5 != true {
        return()
    } else {
        println!("Device found on COM5")
    }


    let mut port = tokio_serial::new("COM5", 115200)
        .open()
        .expect("Failed to open port");

    let mut serial_buf : Vec<u8> = vec![0; 32];

    // external async function for handling reads
    data_loop::data_loop_helper(port, serial_buf);
    
    /*
    match read {
        Ok(msg) {
            // need to destructure the msg here 
            // then need to match it against all of its possible combinations that we established earlier
            // so 1:0, 1:1, 2:0, 2:1, 3:0, 3:1, 4:0, 4:1, 5, 6:0, 6:1, 7:0, 7:1, 8:0, 8:1
            match msg {

            }




        }
        Err(_) {
            return("Found no data when reading")
        }
    };
    */

}
