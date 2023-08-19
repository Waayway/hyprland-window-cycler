mod window;
mod server;
mod client;
mod dbus;
mod current_windows;

use std::env;

#[derive(Debug)]
pub enum Mode {
    Daemon,
    Client,
}

fn main() {
    let mut args = env::args();
    args.next();
    let mode = match args.next() {
        Some(x) => match x.as_str() {
            "-d" => Mode::Daemon,
            "-c" => Mode::Client,
            _ => {
                println!("Invalid mode specified, defaulting to client mode");
                Mode::Client
            }
        },
        None => {
            println!("No mode specified, defaulting to client mode");
            Mode::Client
        },
    };
    println!("Mode: {:?}", mode);
    
    match mode {
        Mode::Daemon => {
            server::main();
        },
        Mode::Client => {
            async_std::task::block_on(client::main(args.collect()));
        },
    };
}