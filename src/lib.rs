use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);
    let file_content = fs::read_to_string(config.filename)?;
    // println!("With text:\n{}", file_content);
    let results = if config.case_sensitive {
        search(&config.query, &file_content)
    } else {
        search_case_insensitive(&config.query, &file_content)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // $Env:CASE_INSENSITIVE=1;
        // 使用 Result 的 is_err 方法来检查其是否是一个 error（也就是环境变量未被设置的情况）
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

//表明返回的 vector 中应该包含引用参数 contents（而不是参数query） slice 的字符串 slice
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
