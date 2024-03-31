use std::{env, error::Error, fs, process};


fn main() {
    let args : Vec<String> = env::args().collect();


    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    
    println!("searching for {}", config.word_to_find);
    println!("in file: {}", config.file_path);

    if let Err(e) = run(config) {
        println!("An error occurred: {e}");
        process::exit(1);
        
    }
}


struct Config {
    word_to_find : String,
    file_path : String,
}



impl Config {
    fn build(args : &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let word_to_find = args[1].clone();
        let file_path = args[2].clone();
        

        Ok(Config { word_to_find, file_path })
    }
}

fn run(config : Config) -> Result<() , Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    println!("contents are: {contents}");
    Ok(())

}

