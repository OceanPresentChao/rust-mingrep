use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); //collect 可以被用来创建很多类型的集合，需要显示注明类型
    let bin_name = &args[0];
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); //打印至标准错误流
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
