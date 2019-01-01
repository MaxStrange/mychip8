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
        match rx.recv_timeout(time::Duration::new(8, 0)) {
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

    /// Asserts that the contents of the given register are equal to the given contents.
    fn assert_register(regidx: u8, regval: u8, tx: &mpsc::Sender<EmulatorCommand>, rx: &mpsc::Receiver<EmulatorResponse>) {
        match send_and_receive(EmulatorCommand::PeekReg(regidx), tx, rx) {
            EmulatorResponse::Reg(received_regval) => assert_eq!(received_regval, regval),
            response => panic!("Response {:?} makes no sense...", response),
        }
    }

    /// Asserts that the contents of register I are equal to the given `regval`.
    fn assert_iregister(regval: u16, tx: &mpsc::Sender<EmulatorCommand>, rx: &mpsc::Receiver<EmulatorResponse>) {
        match send_and_receive(EmulatorCommand::PeekI, tx, rx) {
            EmulatorResponse::I(received_regval) => assert_eq!(received_regval, regval),
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

    /// Test that the SEVxByte instruction works by loading a value into a register, then comparing a byte with that register
    /// and seeing if we break at the appropriate place.
    #[test]
    fn test_sevxbyte() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/SEVxByte/sevxbytetest.bin"));

        // Check that the PC is at the correct location
        assert_pc(0x020C, &tx, &rx);

        // Check that register V3 has the expected value
        assert_register(3, 0x23, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test that the SNEVxByte instruction works by loading a value into a register, then comparing a byte with that register
    /// and seeing if we break at the appropriate place.
    #[test]
    fn test_snevxbyte() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/SNEVxByte/snevxbytetest.bin"));

        // Check that the PC is at the correct location
        assert_pc(0x020C, &tx, &rx);

        // Check that register V3 has the expected value
        assert_register(3, 0x25, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test that the SEVxVy instruction works by loading a value into two different registers and comparing them
    /// and then checking if we break at the right place.
    #[test]
    fn test_sevxvy() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/SEVxVy/sevxvytest.bin"));

        // Check that the PC is at the correct location
        assert_pc(0x020C, &tx, &rx);

        // Check that register V3 has the expected value
        assert_register(3, 0x25, &tx, &rx);

        // Check that register V4 has the expected value
        assert_register(4, 0x25, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test LDVxByte instruction by loading each general purpose register with a known value and checking them.
    #[test]
    fn test_ldvxybyte() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/LDVxByte/ldvxbytetest.bin"));

        // Check that the PC is where we expect
        assert_pc(0x021E, &tx, &rx);

        /* Now check all the registers */
        assert_register(0, 0x25, &tx, &rx);
        assert_register(1, 0x0A, &tx, &rx);
        assert_register(2, 0xCC, &tx, &rx);
        assert_register(3, 0xFF, &tx, &rx);
        assert_register(4, 0x10, &tx, &rx);
        assert_register(5, 0x11, &tx, &rx);
        assert_register(6, 0x22, &tx, &rx);
        assert_register(7, 0x23, &tx, &rx);
        assert_register(8, 0x85, &tx, &rx);
        assert_register(9, 0x09, &tx, &rx);
        assert_register(10, 0xAE, &tx, &rx);
        assert_register(11, 0x0E, &tx, &rx);
        assert_register(12, 0x44, &tx, &rx);
        assert_register(13, 0x35, &tx, &rx);
        assert_register(14, 0x15, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test ADDVxByte instruction.
    #[test]
    fn test_addvxbyte() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/ADDVxByte/addvxbytetest.bin"));

        // Check that the register is what we expect it should be
        assert_register(10, 0x67, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test LDVxVy instruction.
    #[test]
    fn test_ldvxvy() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/LDVxVy/ldvxvytest.bin"));

        // Check register VA
        assert_register(10, 0x02, &tx, &rx);

        // Check register VD
        assert_register(10, 0x02, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test the ORVxVy instruction.
    #[test]
    fn test_orvxvy() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/ORVxVy/orvxvytest.bin"));

        // Check register VA
        assert_register(10, 0x0E | 0x03, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test the ANDVxVy instruction.
    #[test]
    fn test_andvxvy() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/ANDVxVy/andvxvytest.bin"));

        // Check register VA
        assert_register(10, 0x0E & 0x03, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test the XORVxVy instruction.
    #[test]
    fn test_xorvxvy() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/XORVxVy/xorvxvytest.bin"));

        // Check register VA
        assert_register(10, 0x0E ^ 0x03, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test ADDVxVy with carry bit and without.
    #[test]
    fn test_addvxvy() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/ADDVxVy/addvxvytest.bin"));

        // Check register VA
        assert_register(10, 0x11, &tx, &rx);

        // Check no carry in VF
        assert_register(15, 0x00, &tx, &rx);

        // Continue to next break point
        tx.send(EmulatorCommand::ResumeExecution).expect("Could not send");

        // Check register VB
        assert_register(11, 0xE7, &tx, &rx);

        // Check carry in VF
        assert_register(15, 0x01, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test SUBVxVy with borrow/no-borrow.
    #[test]
    fn test_subvxvy() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/SUBVxVy/subvxvytest.bin"));

        // Check register VA
        assert_register(10, 0x0B, &tx, &rx);

        // Check no borrow in VF
        assert_register(15, 0x01, &tx, &rx);

        // Continue to next break point
        tx.send(EmulatorCommand::ResumeExecution).expect("Could not send");

        // Check register VB
        assert_register(11, 0xDD, &tx, &rx);

        // Check borrow in VF
        assert_register(15, 0x00, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test SHRVx with LSB/no LSB
    #[test]
    fn test_shrvx() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/SHRVx/shrvxtest.bin"));

        // Check register VA
        assert_register(10, 0x07, &tx, &rx);

        // Check VF
        assert_register(15, 0x00, &tx, &rx);

        // Continue to next break point
        tx.send(EmulatorCommand::ResumeExecution).expect("Could not send");

        // Check register VB
        assert_register(11, 0x7E, &tx, &rx);

        // Check VF
        assert_register(15, 0x01, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test SUBVNxVy with borrow/no-borrow.
    #[test]
    fn test_subnvxvy() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/SUBNVxVy/subnvxvytest.bin"));

        // Check register VA
        assert_register(10, 0x0B, &tx, &rx);

        // Check no borrow in VF
        assert_register(15, 0x00, &tx, &rx);

        // Continue to next break point
        tx.send(EmulatorCommand::ResumeExecution).expect("Could not send");

        // Check register VB
        assert_register(11, 0xDD, &tx, &rx);

        // Check borrow in VF
        assert_register(15, 0x01, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test SHLVx with LSB/no LSB
    #[test]
    fn test_shlvx() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/SHLVx/shlvxtest.bin"));

        // Check register VA
        assert_register(10, 0x1C, &tx, &rx);

        // Check VF
        assert_register(15, 0x00, &tx, &rx);

        // Continue to next break point
        tx.send(EmulatorCommand::ResumeExecution).expect("Could not send");

        // Check register VB
        assert_register(11, 0xFA, &tx, &rx);

        // Check VF
        assert_register(15, 0x01, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test that the SNEVxVy instruction works by loading a value into two different registers and comparing them
    /// and then checking if we break at the right place.
    #[test]
    fn test_snevxvy() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/SNEVxVy/snevxvytest.bin"));

        // Check that the PC is at the correct location
        assert_pc(0x020C, &tx, &rx);

        // Check that register V3 has the expected value
        assert_register(3, 0x25, &tx, &rx);

        // Check that register V4 has the expected value
        assert_register(4, 0x26, &tx, &rx);

        exit_and_join(emu, &tx);
    }

    /// Test LDIAddr by loading a byte into I and checking it.
    #[test]
    fn test_ldiaddr() {
        let (emu, tx, rx) = emulate(path::Path::new("testprograms/LDIAddr/ldiaddrtest.bin"));

        // Check that register I has the right value.
        assert_iregister(0x021E, &tx, &rx);

        exit_and_join(emu, &tx);
    }
}
