use winapi::um::winuser::INPUT;
use std::mem;
use winapi::um::winuser::{self, INPUT_KEYBOARD, KEYBDINPUT};

pub fn input_helper(scancode : u16, flags : u32) -> [INPUT;1] {
        let mut input_union : winuser::INPUT_u = unsafe { mem::zeroed() };
        let inner_input_union = unsafe { input_union.ki_mut() };

        *inner_input_union = KEYBDINPUT {
            wVk: 0,
            wScan: scancode,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0,
        };

        let input = 
            [INPUT {
                type_ : INPUT_KEYBOARD,
                u: input_union,
        }; 1];

        return input
}

// originally had definitions in the data loop, but i moved it to this function to make things cleaner and allow for better call and pass rather than multiple redefinitions and bloated code