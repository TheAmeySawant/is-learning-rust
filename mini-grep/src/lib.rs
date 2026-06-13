use std::{fs, process};

use crate::CaseSensitivity::{InSensitive, Sensitive};

#[derive(Debug)]
enum CaseSensitivity {
    Sensitive,
    InSensitive,
}

#[derive(Debug)]
pub struct Config {
    file_path: String,
    query: String,
    sensitivity: CaseSensitivity,
}

impl Config {
    //My version 1 of build was using if-else ladder
    //But now we just take ownership of args iterator and avoiding heap heavy tasks like creating Vec<String>, cloning the String reference
    //This small change makes the V1 much more memory and performance efficient than V2.
    //Actually i should call this V3, right, but i am lazy :D

    pub fn build_v1(mut input_args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        input_args.next();

        if let Some(query) = input_args.next() {
            if let Some(file_path) = input_args.next() {
                match input_args.next() {
                    Some(s) => {
                        if input_args.next().is_none() && s == "-s" {
                            return Ok(Config {
                                file_path,
                                query,
                                sensitivity: Sensitive,
                            });
                        }
                    }
                    None => {
                        return Ok(Config {
                            file_path,
                            query,
                            sensitivity: InSensitive,
                        });
                    }
                }
            }
        }

        Err(
            "\n\n\n-------------------------------------------------------------------
Error: Invalid Command!

Follow these Command Format for, 

Case In-Sensitive Search (By Default): cargo run -- query file_path
Case Sensitive Search : cargo run -- query file_path -s
-------------------------------------------------------------------\n\n\n",
        )
    }

    //Then i found out how to use match on slice of Strings, would be idiomatic, that's how i got v2
    pub fn build_v2(input_args: &[String]) -> Result<Self, &'static str> {
        match input_args {
            [_, query, file_path] => Ok(Config {
                file_path: file_path.clone(),
                query: query.clone(),
                sensitivity: CaseSensitivity::InSensitive,
            }),
            [_, query, file_path, sensitvity] if sensitvity == "-s" => Ok(Config {
                file_path: input_args[2].clone(),
                query: input_args[1].clone(),
                sensitivity: CaseSensitivity::Sensitive,
            }),

            _ => Err(
                "\n\n\n-------------------------------------------------------------------
Error: Invalid Command!

Follow these Command Format for, 

Case In-Sensitive Search (By Default): cargo run -- query file_path
Case Sensitive Search : cargo run -- query file_path -s
-------------------------------------------------------------------\n\n\n",
            ),
        }
    }

    pub fn run(&self) {
        println!("Searching for file: {}", self.file_path);

        let contents = fs::read_to_string(&self.file_path).unwrap_or_else(|err| {
            println!("Error occurred in run() of Config :{err}");
            process::exit(1);
        });

        let search_result;

        match self.sensitivity {
            CaseSensitivity::InSensitive => {
                search_result = self.search_case_insensitive_using_iterator(&contents)
            }
            CaseSensitivity::Sensitive => search_result = self.search_case_sensitive_using_iterator(&contents),
        }

        println!(
            "\nResult of Case {:?} Search:\n\nLines of {} file, containing '{}' :\n{:?}\n",
            self.sensitivity, self.file_path, self.query, search_result
        );
    }

    pub fn search_case_insensitive<'a>(&self, contents: &'a String) -> Vec<&'a str> {
        let mut search_result = Vec::<&str>::new();
        let query_small: String = self.query.to_lowercase();

        let contents_small = contents.to_lowercase();
        let contents_small = contents_small.lines().enumerate();

        let contents: Vec<&str> = contents.lines().collect();

        for (idx, line) in contents_small {
            if line.contains(&query_small) {
                search_result.push(contents[idx]);
            }
        }

        search_result
    }


    //performance wise both are same, but this is compact & idiomatic.
    pub fn search_case_insensitive_using_iterator<'a>(&self, contents: &'a String) -> Vec<&'a str> {
        let query_small: String = self.query.to_lowercase();

        let contents_small = contents.to_lowercase();

        let contents: Vec<&str> = contents.lines().collect();

        //1. break into lines
        //2. enumerate with index
        //3. filter lines which have query
        //4. get og content using index
        //5. return collection
        contents_small
            .lines()
            .enumerate()
            .filter(|tup: &(usize, &str)| tup.1.contains(&query_small))
            .map(|tup: (usize, &str)| contents[tup.0])
            .collect()

    }

    pub fn search_case_sensitive<'a>(&self, contents: &'a String) -> Vec<&'a str> {
        let mut search_result = Vec::<&str>::new();

        for line in contents.lines() {
            if line.contains(&self.query) {
                search_result.push(line);
            }
        }

        search_result
    }

    //This one is compact & idiomatic
    pub fn search_case_sensitive_using_iterator<'a>(&self, contents: &'a String) -> Vec<&'a str> {
        contents
        .lines()
        .filter(|line| line.contains(&self.query))
        .collect()
    }

    pub fn get_file_path(&self) -> &String {
        &self.file_path
    }

    pub fn get_query(&self) -> &String {
        &self.query
    }

    // pub fn dummy() -> Self {
    //     Config { file_path: "".to_string(), query: "".to_string() }
    // }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "rUsT".to_string();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me."
            .to_string();

        let config = Config {
            file_path: contents,
            query,
            sensitivity: CaseSensitivity::InSensitive,
        };

        assert_eq!(
            vec!["Rust:", "Trust me."],
            config.search_case_insensitive(config.get_file_path())
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "rUsT".to_string();
        // this \ in string literal is used to escape the next line, that is the starts from Rust:... not from a 'new line and then Rust:...'
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me."
            .to_string();

        let config = Config {
            file_path: contents,
            query,
            sensitivity: CaseSensitivity::Sensitive,
        };

        assert_eq!(
            Vec::<&str>::new(),
            config.search_case_sensitive(config.get_file_path())
        );
    }
}
