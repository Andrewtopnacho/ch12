enum ConfigError {
    MissingQuery,
    MissingFilePath,
}
type Error = Box<dyn std::error::Error>;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub file_data: Option<String>,
}

impl Config {
    // Creates a collection of environment arguments 
    pub fn from_env() -> Result<Config, Error> {
        let args = std::env::args().collect::<Vec<String>>();
        
        let query = args.get(1)
            .ok_or("Missing first argument. Usage: ch12 <query> <file path>")?;
        
        let file_path = args.get(2)
            .ok_or("Missing second argument. Usage: ch12 <query> <file path>")?;
        
        Ok(Config::new(query, file_path))
    }

    // Creates config from a slice of string
    pub fn new(query: &str, file_path: &str) -> Config {

        Config {
            query: query.to_owned(),
            file_path: file_path.to_owned(),
            file_data: None,
        }
    }

    pub fn search_for_query(&self) -> Option<Vec<&str>> {
        return match &self.file_data {
            Some(file_data) => Some(search(&self.query, &file_data)),
            None => None,
        }
    }

    pub fn read_file(&mut self) -> Result<(), Error> {
        self.file_data = Some(std::fs::read_to_string(&self.file_path)?);
        Ok(())
    }

    // Runs a configuration (of self)
    pub fn run(&mut self) -> Result<(), Error> {
        self.read_file()?; // Reads contents of self

        let search_result = match self.search_for_query() {
            Some(matching_lines) => matching_lines, // Store matching lines
            None => return Err(format!("{} was not found in {}", self.query, self.file_path).into()) // Returns error with context 
        };

        for line in search_result {
            println!("{}", line);
        }

        Ok(())
    }


}
// Creates vector of lines containing query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }

    matches
}

#[cfg(test)]
mod test {
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
}

