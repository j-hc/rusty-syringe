use std::{env, process};
use std::path::Path;
use rusty_syringe::inject_dll;

static RED: &str = "\x1b[91m";
static RESET: &str = "\x1b[0m";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("{} Usage: {} <dll> <pid> {}", RED, args[0], RESET);
        process::exit(0);
    }

    let dll_path = Path::new(&args[1]);
    let pid: u32 = match args[2].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("{} <pid> parsing error {}", RED, RESET);
            process::exit(0);
        }
    };

    if !dll_path.exists() {
        eprintln!("{} No <dll> in the given path is found {}", RED, RESET);
        process::exit(0);
    }

    let canon_path = dll_path.canonicalize().unwrap();
    let dll_path = canon_path.into_os_string().into_string().unwrap();

    if let Err(_e) = inject_dll(pid, &dll_path) {
        eprintln!("Error");
        process::exit(1);
    }
}