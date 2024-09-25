mod byteutil;
mod disassemble;
mod fileutil;

fn print_help() {
    println!("CHIP-RS");
    println!("A fun project to learn Rust with by @hotcocoaNcode");
    println!("\nFLAGS");
    println!("chip-rs --help\nShow help menu\n");
    println!("chip-rs -d <file>\nDisassemble a CHIP-8 Program\n");
    println!("chip-rs <file>\nchip-rs -r <file>\nExecute a CHIP-8 Program\n");
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 => {
            println!("For some reason, 0 arguments were passed. \nThis should not happen. \nPlease create a PR on GitHub so this can be fixed.");
        }

        1 => {
            println!("Not enough arguments provided! For help, please use --help.");
        }

        2 => {
            if args[1] == "--help" {
                print_help();
            } else {
                //TODO: Run CHIP-8 files

            }
        }

        3 => {
            if args[1] == "-d" {
                disassemble::print_disassembly(fileutil::load_bytes(args[2].clone()));
            } else if args[1] == "-r" {
                //TODO: Run CHIP-8 files

            } else {
                println!("Unrecognized flag {}! For help, please use --help.", args[1]);
            }
        }

        _ => {
            println!("Too many arguments! For help, please use --help.");
        }
    }
}