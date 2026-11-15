use tokio_serial::SerialPortBuilderExt;
use std::error::Error;

pub async fn data_loop_helper() -> Result<(), Box<dyn Error>> {

    let mut port = tokio_serial::new("COM5", 115200)
        .open_native_async()
        .expect("Failed to open port"); 

    // might need to increase depending on results
    let mut buffer  = [0; 76];
    
    // Continuous loop where we read from the serial port that the esp32 is connected to and monitor incoming messages
    loop {
       let count = tokio::io::AsyncReadExt::read(&mut port, &mut buffer).await?;
        
        // use 40 bytes as the middle ground seperator 
        // if the length is greater than 40 assume that its a combination of two buttons simultaneously 
        // if the length is less than 40 assume that its a single button press
        // 40 is good cause it takes into consideration time as well
        // since time makes it start at 33 and then increase incrementally 

        if count <= 40 {
            // Saving the message thats stored in memory in the serial port
            let content = &buffer[..count];

            
            // The third last element is hold or release
            // the fifth last element is the numeric value of the button e.g up is 1
            // first goal should be to pull out and compare the fifth element from the end to see what input it matches
            // within that you should see if its hold or release 
            // based on that u make a call to the xbox virtual controller which will execute the in game control 
        }


        if count > 0 {
            println!("The bytes: {:?}", &buffer[..count]);
            println!("length {}", count)
        }

    }
// [73, 32, 40, 57, 53, 49, 52, 55, 50, 49, 41, 32, 108, 117, 97, 58, 32, 91, 99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 93, 32, 50, 58, 49, 13, 10]
// [73, 32, 40, 49, 54, 54, 56, 57, 50, 56, 49, 41, 32, 108, 117, 97, 58, 32, 91, 99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 93, 32, 50, 58, 49, 13, 10]
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

/*
Single button presses seem to be of length 35 (array with 35 bytes) while simultaneous double button presses are 70
Will at most usually only have two buttons being pressed at the same time at max

So we could then make it so that we check at most a size of 70 and if the size is greater than 70 say 3 or 4 button presses at once we ignore
or actually ig we would just ignore by default and the smashes shouldnt do anything depending on how the code is written

the last byte is line feed LF and the second last is carriage return CR the input number combination is the 3 bytes after (or 1 byte after if its the home button)
*/