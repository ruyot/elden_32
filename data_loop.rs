use tokio_serial::SerialPortBuilderExt;
use std::{error::Error};
use winput::{Action, Button, Input};
use winapi::{shared::basetsd::INT32, um::winuser::{self, INPUT, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE}};

use crate::input::input_helper;

pub async fn data_loop_helper() -> Result<(), Box<dyn Error>> {

    let mut port = tokio_serial::new("COM5", 115200)
        .open_native_async()
        .expect("Failed to open port"); 

    // might need to increase depending on results
    let mut buffer  = [0; 76];

    // Boolean flag for changing configurations from "attacking" to "exploring" 
    let mut is_home = false;
    
    // Continuous loop where we read from the serial port that the esp32 is connected to and monitor incoming messages
    loop {
        let count = tokio::io::AsyncReadExt::read(&mut port, &mut buffer).await?;

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
            println!("user pressed home");
            if is_home == false {
                is_home = true;
                println!("Switched to configuration 2 (Exploring)")
            } else {
                is_home = false;
                println!("Switched to configuration 1 (Attacking)")
            }
        }

        // If its a single button press and were on config 1 "attacking" proceed with everything related to config 1
        if count <= 40 && is_home == false { 
            if button == 49 {
                    // W/UP was contacted 
                    // Check if it was a release or hold
  
                    if hold_or_release == 48 {
                        // Controller hold forward
                        println!("W hold");
                        let scancode = 0x11;
                        let flags = KEYEVENTF_SCANCODE; // we want a scancode and the key to be down

                        let mut input = input_helper(scancode, flags);

                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    } else {
                        // Controller release forward
                        println!("W release");
                        let scancode = 0x11;
                        let flags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP; // we want a scancode and the key to release
                        
                        let mut input = input_helper(scancode, flags);

                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    }     

            } else if button == 50 {
                    // A/LEFT was contacted
                    // Check if it was a release or hold

                    if hold_or_release == 48 {
                        // Controller hold left
                        println!("A hold");
                        let scancode = 0x1E;
                        let flags = KEYEVENTF_SCANCODE;

                        let mut input = input_helper(scancode, flags);
                        
                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    } else {
                        // Controller release left
                        println!("A release");
                        let scancode = 0x1E;
                        let flags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;

                        let mut input = input_helper(scancode, flags);

                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    }

            } else if button == 51 {
                    // S/DOWN was contacted 
                    // Check if it was a release or hold

                    if hold_or_release == 48 {
                        // Controller hold down
                        println!("S hold");
                        let scancode = 0x1F;
                        let flags = KEYEVENTF_SCANCODE;

                        let mut input = input_helper(scancode, flags);

                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    } else {
                        // Controller release down
                        println!("S release");
                        let scancode = 0x1F;
                        let flags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;

                        let mut input = input_helper(scancode, flags);
                        
                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    }

            } else if button == 52 {
                    // D/RIGHT was contacted
                    // Check if it was a release or hold

                    if hold_or_release == 48 {
                        // Controller hold right
                        println!("D hold");
                        let scancode = 0x20;
                        let flags = KEYEVENTF_SCANCODE;

                        let mut input = input_helper(scancode, flags);

                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    } else {
                        // Controller release right
                        println!("D release");
                        let scancode = 0x20;
                        let flags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;

                        let mut input = input_helper(scancode, flags);

                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    }

            } else if button == 54 {
                    // START was contacted
                    // Check if it was a release or hold

                    if hold_or_release == 48 {
                        // Controller hold start (probably can be e)
                        println!("Start hold");
                        let scancode = 0x12;
                        let flags = KEYEVENTF_SCANCODE;

                        let mut input = input_helper(scancode, flags);

                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    } else {
                        // Controller release start
                        println!("Start release");
                        let scancode = 0x12;
                        let flags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;

                        let mut input = input_helper(scancode, flags);

                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    }
            
            } else if button == 55 {
                    // B was contacted
                    // Check if it was a release or hold

                    if hold_or_release == 48 {
                        // Controller hold B
                        println!("Spacebar hold");
                        let scancode = 0x39;
                        let flags = KEYEVENTF_SCANCODE;

                        let mut input = input_helper(scancode, flags);

                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    } else {
                        // Controller release B
                        println!("Spacebar release");
                        let scancode = 0x39;
                        let flags = KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP;

                        let mut input = input_helper(scancode, flags);
                        
                        unsafe {
                            winuser::SendInput(1, input.as_mut_ptr(), size_of::<INPUT>() as i32);
                        }
                    }

            } else if button == 56 {
                    // A was contacted
                    // Check if it was a release or hold

                    if hold_or_release == 48 {
                        // Controller hold A
                        println!("Mouse left click hold");
                        let input = Input::from_button(Button::Left, Action::Press);
                        winput::send_inputs(&[input]);
                    } else {
                        // Controller release A
                        println!("Mouse left click release");
                        let input = Input::from_button(Button::Left, Action::Release);
                        winput::send_inputs(&[input]);
                    }
            }
        } 
        // If its a single button press and were on config 2 "exploring" proceed with everything related to config 2
        else if count <= 40 && is_home == true {
            // for now just print something
            println!("were on config 2!");
        }
        // will consider another branch for combination inputs (two buttons at once) later if determined necessary
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