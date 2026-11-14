use tokio_serial::SerialPortBuilderExt;
use std::error::Error;

pub async fn data_loop_helper() -> Result<(), Box<dyn Error>> {

    let mut port = tokio_serial::new("COM5", 115200)
        .open_native_async()
        .expect("Failed to open port"); 

    let mut buffer  = [0; 50];
    
    // Continuous loop where we read from the serial port that the esp32 is connected to and monitor incoming messages
    loop {
        let count = tokio::io::AsyncReadExt::read(&mut port, &mut buffer).await?;
        
        if count > 0 {
            println!("The bytes: {:?}", &buffer[..count]);
            println!("length {}", count)
        }

    }
    
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