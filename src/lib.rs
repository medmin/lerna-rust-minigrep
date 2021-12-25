use std::{fs, error::Error, env};


pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }
    Ok(())
}


pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        args.next();

        let query =  match args.next() {
            Some(x) => x,
            None => return Err("Did not get a query string"),
        };
        let filename =  match args.next() {
            Some(x) => x,
            None => return Err("Did not get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename, 
            case_sensitive
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.        
Duct tape.
";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.        
Trust.
";

        assert_eq!(vec!["Rust:", "Trust."], search_case_insensitive(query, contents));
    }
}