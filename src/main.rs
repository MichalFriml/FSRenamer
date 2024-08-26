use std::{env, process};
use file_system_name_refactor::{Config, run};


fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let config = Config::build(&args).unwrap_or_else(|err| {
        let mut silent: bool = true;
        for arg in args {
            if arg.contains("-s") {silent = true}
        }
        if !silent {println!("Problem parsing arguments: {err}");}
        process::exit(1);
    });

    println!("{:?}", config);
    if let Err(e) = run(&config) {
        if !config.silent {println!("Application error: {e}")}
        process::exit(1);
    }
}


