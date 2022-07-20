use std::env;
fn main() {
    let args: Vec<String> = env::args().collect(); //collect 可以被用来创建很多类型的集合，需要显示注明类型
    let bin_name = &args[0];
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
