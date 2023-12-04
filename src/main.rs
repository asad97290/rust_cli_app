
use std::env;
use rust_cli_app::Config;
// use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let args = Config::new(&args);
   
    let args:Config = match args {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{:?}",err);
            return;
        }
    };
    let _ = rust_cli_app::run(args);
}

