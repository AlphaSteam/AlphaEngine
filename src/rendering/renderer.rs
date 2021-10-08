extern crate glium;
pub use crate::rendering::vertex::Vertex;
pub use crate::window::Window;
use glium::{uniform, Display, Surface};

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }
    pub fn start(&self, display: &Display){

    }
    pub fn render(&self, display: &Display) {
        let mut target = display.draw();
        let vertex1 = Vertex {
            position: [-0.5, -0.5, 0.0],
        };
        let vertex2 = Vertex {
            position: [0.0, 0.5, 0.0],
        };
        let vertex3 = Vertex {
            position: [0.5, -0.25, 0.0],
        };
        let shape = vec![vertex1, vertex2, vertex3];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = r#"
        #version 140

        in vec3 position;
    
        void main() {
            gl_Position = vec4(position, 1.0);
        }
"#;
        let fragment_shader_src = r#"
        #version 140

        out vec4 color;
    
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
"#;
        let program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        /* target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap(); */
        target.finish().unwrap();
    }
    pub fn stop(&self) {}
}
