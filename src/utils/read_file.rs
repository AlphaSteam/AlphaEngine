use std::{fs::File, io::Read};

pub fn read_file(filename: String) -> String {
    match File::open(filename.clone()) {
        // The file is open (no error).
        Ok(mut file) => {
            let mut content = String::new();

            // Read all the file content into a variable (ignoring the result of the operation).
            file.read_to_string(&mut content).unwrap();

            content
            // The file is automatically closed when is goes out of scope.
        }
        // Error handling.
        Err(error) => {
            panic!("Error reading shader {}: {}", filename, error)
        }
    }
}
