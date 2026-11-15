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
            
            // The third last element is the number indicating whether the input is a hold or a release
            let hold_or_release_i = count - 3;           
            
            // checking whether or not theres a semi colon allows us to differentiate the input from home (which is 5) or the other 7 buttons which are 1:0, 2:1, 2:0 etc
            let semi_i = count - 4;

            // The fifth last element is the number indicating what button the input message is from
            let button_i = count - 5;

            let hold_or_release = content[hold_or_release_i];
            let semi = content[semi_i];
            let button = content[button_i];

            if semi != 58 {
                // HOME was contacted
                
                println!("Home")

            } else if button == 49 {
                    // W/UP was contacted 
                    // Check if it was a release or hold
  
                    if hold_or_release == 48 {
                        // Controller hold forward
                        println!("W hold")
                    } else {
                        // Controller release forward
                        println!("W release")
                    }

            } else if button == 50 {
                    // A/LEFT was contacted
                    // Check if it was a release or hold

                    if hold_or_release == 48 {
                        // Controller hold left
                        println!("A hold")
                    } else {
                        // Controller release left
                        println!("A release")
                    }

            } else if button == 51 {
                    // S/DOWN was contacted 
                    // Check if it was a release or hold

                    if hold_or_release == 48 {
                        // Controller hold down
                        println!("S hold")
                    } else {
                        // Controller release down
                        println!("S release")
                    }

            } else if button == 52 {
                    // D/RIGHT was contacted
                    // Check if it was a release or hold

                    if hold_or_release == 48 {
                        // Controller hold right
                        println!("D hold")
                    } else {
                        // Controller release right
                        println!("D release")
                    }

            } else if button == 54 {
                    // START was contacted
                    // Check if it was a release or hold

                    if hold_or_release == 48 {
                        // Controller hold start? (gonna figure out what this should do later)
                        println!("Start hold")
                    } else {
                        // Controller release start
                        println!("Start release")
                    }
            
            } else if button == 55 {
                    // A was contacted
                    // Check if it was a release or hold

                    if hold_or_release == 48 {
                        // Controller hold A
                        println!("A hold")
                    } else {
                        // Controller release A
                        println!("A release")
                    }

            } else if button == 56 {
                    // B was contacted
                    // Controller hold B 

                    if hold_or_release == 48 {
                        // Controller hold B
                        println!("B hold")
                    } else {
                        // Controller release B
                        println!("B release")
                    }

            }
        } 
        // will consider another branch for combination inputs (two buttons at once) later if determined necessary
        
            // the fifth last element is the numeric value of the button e.g up is 1
            // first goal should be to pull out and compare the fifth element from the end to see what input it matches
            // within that you should see if its hold or release 
            // based on that u make a call to the xbox virtual controller which will execute the in game control 

        /*
        if count > 0 {
            println!("The bytes: {:?}", &buffer[..count]);
            println!("length {}", count)
        }
        */
    };
}

/*
Single button presses seem to be of length 35 (array with 35 bytes) while simultaneous double button presses are 70
Will at most usually only have two buttons being pressed at the same time at max

So we could then make it so that we check at most a size of 70 and if the size is greater than 70 say 3 or 4 button presses at once we ignore
or actually ig we would just ignore by default and the smashes shouldnt do anything depending on how the code is written

length also increases as time elapsed increases since the array has a timestamp embedded within it which is why the buffer size is larger than just 70 to account for fluctuation

the last byte is line feed LF and the second last is carriage return CR the input number combination is the 3 bytes after (or 1 byte after if its the home button)
*/