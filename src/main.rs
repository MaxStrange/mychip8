/* Externs */
extern crate clap;

/* Mods */
mod display;
mod emulator;

/* Uses */
use self::emulator::chip8;
use std::fs;
use std::io::Read;
use std::sync::mpsc;
use std::path;
use std::process;
use std::thread;

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

    // Make some pipes. Use these for debugging and in the test rig.
    let (_mytx, yourrx): (mpsc::Sender<chip8::EmulatorCommand>, mpsc::Receiver<chip8::EmulatorCommand>) = mpsc::channel();
    let (yourtx, _myrx): (mpsc::Sender<chip8::EmulatorResponse>, mpsc::Receiver<chip8::EmulatorResponse>) = mpsc::channel();

    // Spawn an emulator. We can send it commands while it is running. Useful for debugging.
    let emuthread = thread::spawn(move || {
        // Create and initialize a Chip 8 instance
        let mut emu = chip8::Chip8::new(yourtx, yourrx);

        // Load the program into memory
        match emu.load(&binary) {
            Ok(()) => (),
            Err(s) => {
                println!("Could not load binary: {}", s);
                process::exit(3);
            },
        }

        emu.run();
    });

    emuthread.join().expect("Did not join emu thread correctly.");
}
