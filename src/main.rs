use std::env;
use std::process;
use rsplayground::{Config,run};

fn main(){
    let args:Vec<String> = env::args().collect();
    // let config = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}",config.query);
    println!("In file{}",config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error:{e}");
        process::exit(1);
    }
}

// fn run(config:Config){
// let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
//     println!("With text:\n{contents}");
// }


// fn parse_config(args:&[String])->Config{
//     let query = args[1].clone();
//     let file_path =args[2].clone();
//     Config {query, file_path}
// }

