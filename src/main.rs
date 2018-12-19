/* Externs */
extern crate clap;

/* Mods */
mod display;
mod emulator;

/* Uses */
use self::emulator::chip8;
use std::fs;
use std::io::Read;
use std::path;
use std::process;

fn main() {
    // Check args for a valid file
    let matches = clap::App::new("Chip 8 Emulator")
                            .version("0.1.0")
                            .author("Max Strange")
                            .about("Emulates Chip 8")
                            .arg(clap::Arg::with_name("programfile")
                                    .short("p")
                                    .long("programfile")
                                    .value_name("FILE")
                                    .help("Path to the Chip 8 Program binary to run")
                                    .takes_value(true)
                                    .required(true))
                            .get_matches();
    let progpath = path::Path::new(matches.value_of("programfile").unwrap());

    // Make sure file is a game file and is valid
    if !progpath.exists() {
        println!("{} does not exist. You must supply a path to a binary that exists", progpath.to_str().unwrap());
        process::exit(1);
    }

    // Load the contents from the file
    let mut contents = match fs::File::open(progpath) {
        Ok(b) => b,
        Err(e) => {
            println!("Problem opening file at location {}: {:?}", progpath.to_str().unwrap(), e);
            process::exit(2);
        },
    };

    // Read the contents into a bufer of bytes
    let mut binary = Vec::<u8>::new();
    match contents.read_to_end(&mut binary) {
        Ok(_nbytes) => (),
        Err(e) => {
            println!("Could not read the contents of the file into a vector: {:?}", e);
        },
    }

    // Create and initialize a Chip 8 instance
    let mut emu = chip8::Chip8::new();

    // Load the program into memory
    match emu.load(&binary) {
        Ok(()) => (),
        Err(s) => {
            println!("Could not load binary: {}", s);
            process::exit(3);
        },
    }

    // Hand over control to the emulator
    emu.run();
}
