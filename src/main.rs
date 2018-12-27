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

/// Creates an emulator thread and returns it along with the pipe to and from it.
pub fn emulate(progpath: &path::Path) -> (thread::JoinHandle<()>, mpsc::Sender<chip8::EmulatorCommand>, mpsc::Receiver<chip8::EmulatorResponse>) {
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
    let (mytx, yourrx): (mpsc::Sender<chip8::EmulatorCommand>, mpsc::Receiver<chip8::EmulatorCommand>) = mpsc::channel();
    let (yourtx, myrx): (mpsc::Sender<chip8::EmulatorResponse>, mpsc::Receiver<chip8::EmulatorResponse>) = mpsc::channel();

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

    (emuthread, mytx, myrx)
}

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

    // _mytx and _myrx are used in testing, not in main
    let (emuthread, _mytx, _myrx) = emulate(&progpath);

    emuthread.join().expect("Did not join emu thread correctly.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::chip8::EmulatorCommand;
    use super::chip8::EmulatorResponse;
    use std::time;

    /// Handles getting the response from the RX pipe, dealing with timeouts and errors as appropriate.
    fn get_response(rx: &mpsc::Receiver<EmulatorResponse>) -> EmulatorResponse {
        match rx.recv_timeout(time::Duration::new(4, 0)) {
            Err(_) => panic!("Could not receive anything from the emulator. Probably it never reached a BRK."),
            Ok(response) => response,
        }
    }

    /// Sends the given `msg`, then waits to hear back and returns the response.
    fn send_and_receive(msg: EmulatorCommand, tx: &mpsc::Sender<EmulatorCommand>, rx: &mpsc::Receiver<EmulatorResponse>) -> EmulatorResponse {
        tx.send(msg).expect("Could not send.");
        get_response(rx)
    }

    /// Sends the exit command and then joins with the emulator thread.
    fn exit_and_join(emu: thread::JoinHandle<()>, tx: &mpsc::Sender<EmulatorCommand>) {
        tx.send(EmulatorCommand::Exit).expect("Could not send exit signal.");
        emu.join().unwrap_or(());
    }

    /// Asserts that the PC is at the given location.
    fn assert_pc(pc: u16, tx: &mpsc::Sender<EmulatorCommand>, rx: &mpsc::Receiver<EmulatorResponse>) {
        match send_and_receive(EmulatorCommand::PeekPC, tx, rx) {
            EmulatorResponse::PC(received_pc) => assert_eq!(received_pc, pc),
            response => panic!("Response {:?} makes no sense...", response),
        }
    }

    /// Asserts that the stack item at `stackidx` is equal to `stackitem`.
    fn assert_stack_item(stackidx: usize, stackitem: u16, tx: &mpsc::Sender<EmulatorCommand>, rx: &mpsc::Receiver<EmulatorResponse>) {
        match send_and_receive(EmulatorCommand::PeekStack, tx, rx) {
            EmulatorResponse::Stack(stack) => assert_eq!(stack[stackidx], stackitem),
            response => panic!("Response {:?} makes no sense...", response),
        }
    }

    /// Asserts that the stack pointer is at the given location.
    fn assert_sp(sp: u8, tx: &mpsc::Sender<EmulatorCommand>, rx: &mpsc::Receiver<EmulatorResponse>) {
        match send_and_receive(EmulatorCommand::PeekSP, tx, rx) {
            EmulatorResponse::SP(received_sp) => assert_eq!(received_sp, sp),
            response => panic!("Response {:?} makes no sense...", response),
        }
    }

    /// SYS is a NOP, so really just test that nothing breaks.
    #[test]
    fn test_sys() {
        let (emu, tx, _rx) = emulate(path::Path::new("testprograms/SYS/systest.bin"));
        exit_and_join(emu, &tx);
    }

    /// CLS is not really testable from this test harness - requires manual oversight. Included here to make sure it doesn't break things.
    #[test]
    fn test_cls() {
        let (emu, tx, _rx) = emulate(path::Path::new("testprograms/CLS/clstest.bin"));
        exit_and_join(emu, &tx);
    }

    /// RET test. Go to a subroutine then return from it and make sure we break at the right place.
    #[test]
    fn test_ret() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/RET/rettest.bin"));

        // Check that PC is at correct location
        assert_pc(0x0202, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// JP test. Jump to a specific address and break. Check PC.
    #[test]
    fn test_jp() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/JP/jptest.bin"));

        // Check that PC is at correct location
        assert_pc(0x020A, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// CALL test. Jump to an address and break. Check PC and stack.
    #[test]
    fn test_call() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/CALL/calltest.bin"));

        // Check that PC is at correct location
        assert_pc(0x020A, &tx, &rx);

        // Check that the first item in the stack is correct.
        assert_stack_item(0, 0x0204, &tx, &rx);

        // Check that the stack pointer is correct
        assert_sp(1, &tx, &rx);

        exit_and_join(emu, &tx);
    }
}
