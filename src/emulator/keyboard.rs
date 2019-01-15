//! This module contains all the code dealing with the user's inputs on the keyboard.
use std::io::{self, Read};
use std::sync::mpsc;

/// A single keystroke as read directly from a keyboard.
pub type Key = String;

/// Takes a byte and returns the String representation of it as follows (stolen from https://github.com/indragiek/Chip8, which
/// in turn stole it from somewhere else...):
///
/// ```
/// Keypad                   Keyboard
/// +-+-+-+-+                +-+-+-+-+
/// |1|2|3|C|                |1|2|3|4|
/// +-+-+-+-+                +-+-+-+-+
/// |4|5|6|D|                |Q|W|E|R|
/// +-+-+-+-+       =>       +-+-+-+-+
/// |7|8|9|E|                |A|S|D|F|
/// +-+-+-+-+                +-+-+-+-+
/// |A|0|B|F|                |Z|X|C|V|
/// +-+-+-+-+                +-+-+-+-+
/// ```
pub fn map(byte: u8) -> Result<Key, String> {
    Ok(match byte {
        0x00 => "X",
        0x01 => "1",
        0x02 => "2",
        0x03 => "3",
        0x04 => "Q",
        0x05 => "W",
        0x06 => "E",
        0x07 => "A",
        0x08 => "S",
        0x09 => "D",
        0x0A => "Z",
        0x0B => "C",
        0x0C => "4",
        0x0D => "R",
        0x0E => "F",
        0x0F => "V",
        _ => {return Err(format!("Cannot convert byte 0x{:X} into a String. Character is not a valid input on the keyboard.", byte));},
    }.to_string())
}

/// A Keyboard is responsible for taking in input from the user, and also providing mechanisms to test itself.
pub struct Keyboard {
    /// An optional pipe that a user of this struct can use to bypass stdin.
    /// If present, we will check this instead of stdin for characters.
    debug_rx: Option<mpsc::Receiver<String>>,
}

impl Keyboard {
    /// Constructs a new Keyboard instance.
    ///
    /// If `debug_rx` is present, we use it instead of stdin.
    pub fn new(debug_rx: Option<mpsc::Receiver<String>>) -> Self {
        Keyboard {
            debug_rx: debug_rx,
        }
    }

    /// Returns true if the given key is currently depressed on the keyboard.
    pub fn check_keyboard_for_key(&self, k: Key) -> bool {
        let input = match &self.debug_rx {
            // We have a debug pipe, so use that instead of the normal keyboard input
            Some(rx) => match rx.recv() {
                Err(_) => panic!("Never received anything from the test interface over the debug rx pipe to the keyboard."),

                // Match on the string we got over the debug interface
                Ok(s) => s,
            },

            // We have no debug pipe, so use the normal keyboard input
            None => {
                let mut buffer = String::new();
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                match handle.read_to_string(&mut buffer) {
                    Err(msg) => {
                        println!("Got invald input over keyboard interface: {:?}", msg);
                        return false;
                    },

                    // Match on the string we built from the keyboard input
                    Ok(_nbytes) => buffer,
                }
            },
        };

        input.contains(k.as_str())
    }
}
