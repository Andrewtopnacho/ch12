use ch12::Config;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = match Config::from_env() {
        Ok(new_config) => new_config,
        Err(config_error) =>
            return Err(format!("Failed to read configuration from command line arguments: {}", config_error).into()),
    };
    
    match config.run() {
        Ok(()) => println!("Exiting"),
        Err(error) => 
            return Err(format!("Error running config: {}", error).into()), 
    };

    Ok(())
}


