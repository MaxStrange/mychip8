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

fn inverse_map(mut k: Key) -> Result<u8, String> {
    k.make_ascii_uppercase();
    Ok(match k.as_str() {
        "X" => 0x00,
        "1" => 0x01,
        "2" => 0x02,
        "3" => 0x03,
        "Q" => 0x04,
        "W" => 0x05,
        "E" => 0x06,
        "A" => 0x07,
        "S" => 0x08,
        "D" => 0x09,
        "Z" => 0x0A,
        "C" => 0x0B,
        "4" => 0x0C,
        "R" => 0x0D,
        "F" => 0x0E,
        "V" => 0x0F,
        _ => {return Err(format!("Cannot convert Key {} to a byte. Character is not a valid input on the keyboard.", k));},
    })
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
        let mut lowercase_key = k;
        lowercase_key.make_ascii_lowercase();

        let mut input = match &self.debug_rx {
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
                    Ok(_nbytes) => match buffer.chars().last() {
                        None => "".to_string(),
                        Some(c) => c.to_string(),
                    },
                }
            },
        };

        input.make_ascii_lowercase();
        input.contains(lowercase_key.as_str())
    }

    /// Waits until the user presses a valid key, then returns the correct byte.
    pub fn wait_for_keypress(&self) -> u8 {
        if let Some(rx) = &self.debug_rx {
            // We are in debug mode, so wait around until we get a valid key
            loop {
                match rx.recv() {
                    Err(e) => panic!("Error on the debug interface keyboard: {:?}", e),
                    Ok(mut s) => {
                        s.make_ascii_lowercase();
                        let c = s.chars().last().expect(format!("Got {}, which does not contain any characters.", s).as_str());

                        if let Ok(byte) = inverse_map(c.to_string()) {
                            return byte;
                        } else {
                            panic!("Got an invalid key on the debug interface: {}", c);
                        }
                    }
                }
            }
        } else {
            // We are in non-debug mode

            let stdin = io::stdin();
            let mut handle = stdin.lock();

            // Sit around reading from stdin until we get something from the user
            loop {
                let mut buffer = String::new();
                match handle.read_to_string(&mut buffer) {
                    Err(msg) => {
                        println!("Got invalid input over keyboard interface: {:?}", msg);
                    },

                    // Check if what we got from the user is a valid key input and return it if so.
                    Ok(_nbytes) => match buffer.chars().last() {
                        None => (),
                        Some(c) => {
                            if let Ok(byte) = inverse_map(c.to_string()) {
                                return byte;
                            }
                        },
                    },
                }
            }
        }
    }
}
