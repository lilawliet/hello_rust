fn validate_input(input: &str) -> Result<String, String>  {
   if input.is_empty() {
    return Err("Input is empty".to_string());
   }

   Ok(input.to_string())
}

fn process_data(data: &str) -> Result<String, String>  {
    if data.len() < 3 {
        return Err("Data is too short".to_string());
    }

    Ok(data.to_uppercase())
}

fn save_data(data: &str) -> Result<(), String> {
    if data.contains("error" ) {
        return Err("Error found in data".to_string());
    }

    println!("Data saved successfully");

    Ok(())
}

fn process_pipeline(input: &str) -> Result<(), String> {
    let validated = validate_input(input)?;
    let processed = process_data(&validated)?;
    save_data(&processed)?;

    Ok(())
}

fn main() {
    match process_pipeline("Hello") {
        Ok(_) => println!("Pipeline completed successfully"),
        Err(e) => println!("Error: {}", e),
    }

    match process_pipeline("") {
        Ok(_) => println!("Pipeline completed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}