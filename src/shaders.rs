use std::{fs::File, io::Read};

pub fn read_shader_from_file(filename: String) -> String {
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
#[derive(Debug)]
pub enum Shader {
    Basic,
    Grayscale,
    Inverted,
    Text,
}

impl Shader {
    pub fn source_code(&self) -> (String, String) {
        match *self {
            Shader::Basic => {
                let source_code_vert =
                    include_str!("shaders/basic_projection_view_model.vert").to_string();
                let source_code_frag =
                    include_str!("shaders/basic_projection_view_model.frag").to_string();
                (source_code_vert, source_code_frag)
            }

            Shader::Grayscale => {
                let source_code_vert =
                    include_str!("shaders/basic_projection_view_model.vert").to_string();
                let source_code_frag = include_str!("shaders/grayscale.frag").to_string();
                (source_code_vert, source_code_frag)
            }

            Shader::Inverted => {
                let source_code_vert =
                    include_str!("shaders/basic_projection_view_model.vert").to_string();
                let source_code_frag = include_str!("shaders/inverted.frag").to_string();
                (source_code_vert, source_code_frag)
            }
            Shader::Text => {
                let source_code_vert = include_str!("shaders/text_rendering.vert").to_string();
                let source_code_frag = include_str!("shaders/text_rendering.frag").to_string();
                (source_code_vert, source_code_frag)
            }
        }
    }
}
