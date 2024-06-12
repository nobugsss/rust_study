use std::env;
use std::process;
use unit12::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // let query = &args[1];
    // let file_path = &args[2];
    // println!("query: {}", query);
    // println!("file_path: {}", file_path);

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    // 重构以上代码
    // let args: Vec<String> = env::args().collect();
    // //     let config = parse_config(&args);
    // // let config = Config::new(&args);
    // // println!("config: {:?}", config);
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });
    // println!("config: {:?}", config);

    // if let Err(e) = run(&config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }

    // 最终改造
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        // eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = unit12::run(&config) {
        println!("Application error: {e}");
        // eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let file_path = &args[2];

//     (query, file_path)
// }

// fn改进
// #[derive(Debug)]
// struct Config {
//     query: String,
//     file_path: String,
// }
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Config { query, file_path }
//     }
// }

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Config { query, file_path }
//     }
// }

// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }
// }

// fn run(config: &Config) {
//     let contents =
//         fs::read_to_string(&config.file_path).expect("Should have been able to read the file");
//     println!("With text:\n{contents}");
// }

// 改造run
// fn run(config: &Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(&config.file_path)?;

//     println!("With text:\n{contents}");

//     Ok(())
// }
