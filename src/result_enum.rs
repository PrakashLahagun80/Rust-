use std::fs::read_to_string;

pub fn read_from_file_a(file_path: String) -> Result<String,String> {
    let result = read_to_string(file_path);
    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error reading file: {}", err)),
    }
}