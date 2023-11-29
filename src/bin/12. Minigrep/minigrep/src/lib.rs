use glob::glob;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum Flags {
    Help,
    Version,
    IgnoreCase,
    Debug,
}

#[derive(Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub flags: Vec<Flags>,
}

impl Config<'_> {
    pub fn build<'b>(args: &'b [String]) -> Result<Config<'b>, &'static str> {
        if args.len() < 2 {
            return Err("Expected at least 1 argument");
        }

        let mut flags = Vec::new();
        let mut query = None;
        let mut file_path = None;

        for arg in &args[1..] {
            match arg.as_str() {
                "-h" | "--help" => flags.push(Flags::Help),
                "-v" | "--version" => flags.push(Flags::Version),
                "-i" | "--ignore-case" => flags.push(Flags::IgnoreCase),
                "-d" | "--debug" => flags.push(Flags::Debug),
                _ => {
                    if query.is_none() {
                        query = Some(arg);
                    } else if file_path.is_none() {
                        file_path = Some(arg);
                    } else {
                        return Err("Unexpected argument");
                    }
                }
            }
        }

        if query.is_none() || file_path.is_none() {
            if flags.is_empty() {
                return Err("Missing required arguments");
            } else {
                return Ok(Config {
                    query: "",
                    file_path: "",
                    flags,
                });
            }
        }

        return Ok(Config {
            query: query.unwrap(),
            file_path: file_path.unwrap(),
            flags,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut case_sensitive = true;
    let mut print_help = false;
    let mut print_version = false;

    for flag in config.flags {
        match flag {
            Flags::Help => print_help = true,
            Flags::Version => print_version = true,
            Flags::IgnoreCase => case_sensitive = false,
            _ => (),
        }
    }

    if print_help {
        println!("Usage: minigrep <query> <file>");
        println!();
        println!("Options:");
        println!("  -h, --help         Prints help information");
        println!("  -v, --version      Prints version information");
        println!("  -i, --ignore-case  Case insensitive search");
        println!("  -d, --debug        Debug mode");
        return Ok(());
    }

    if print_version {
        println!("minigrep {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let file_path = discover_file(config.file_path)?;

    let contents = fs::read_to_string(file_path)?;

    println!("🔎 Searching for \"{}\"", config.query);
    println!("File: {}", config.file_path);

    for line in search(&config.query, &contents, case_sensitive) {
        println!("{line}");
    }

    Ok(())
}

fn discover_file(file_path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let file_path_pattern = format!("{}/**/{}", env::current_dir()?.display(), file_path);
    let mut file_paths = Vec::new();

    for entry in glob(&file_path_pattern)? {
        match entry {
            Ok(path) => file_paths.push(path),
            Err(e) => return Err(Box::new(e)),
        }
    }

    if file_paths.is_empty() {
        return Err(format!("File not found: {}", file_path).into());
    }

    Ok(file_paths[0].clone())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if case_sensitive {
            if line.contains(&query) {
                results.push(line);
            }
        } else {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(line);
            }
        }
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
    }
}
