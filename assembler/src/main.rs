//! The Chip8 assembler takes one or more assembly files and converts them into object files, which can then be linked.

extern crate clap;

use std::path;
use std::process;

fn main() {
    let matches = clap::App::new("Chip 8 Assembler")
                            .version("0.1.0")
                            .author("Max Strange")
                            .about("Assembles valid assembly files into Chip 8 machine code.")
                            .arg(clap::Arg::with_name("sources")
                                    .required(true)
                                    .multiple(true)
                                    .takes_value(true)
                                    .help("The ASM file(s) to assemble."))
                            .get_matches();

    let source_paths_strings: Vec<_> = matches.values_of("sources").unwrap().collect();
    let mut source_paths = Vec::<&path::Path>::new();
    for fpath in source_paths_strings {
        let p = path::Path::new(fpath);
        source_paths.push(&p);
    }

    // Make sure each path is valid
    for fpath in source_paths.iter() {
        if !fpath.exists() {
            println!("{:?} does not exist.", fpath);
            process::exit(1);
        }
    }

    for fpath in source_paths.iter() {
        match assemble_file(fpath) {
            Ok(()) => (),
            Err(msg) => println!("Could not assemble file {:?}: {:?}", fpath, msg),
        }
    }
}

/// Assemble the file found at the given path and return an error message if it doesn't work.
fn assemble_file(_fpath: &path::Path) -> Result<(), String> {
    // TODO
    // Lex the file into tokens
    // Preprocess the tokens using the preprocessor
    // Parse the token stream into an AST
    // Apply recursive descent to the AST to visit each node and generate machine code in an object file output
    Ok(())
}
