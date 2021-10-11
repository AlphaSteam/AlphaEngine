extern crate glium;
pub use crate::rendering::vertex::Vertex;
use crate::sys::system::System;
pub use crate::window::Window;
use glium::{uniform, BackfaceCullingMode, Blend, Display, Surface};
use image::GenericImageView;
pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }
    pub fn start(&self, display: &Display, system: &mut System) {
        let game_objects = system.game_objects_mut().clone();
        for game_object in game_objects {
            let shape = game_object.mesh().vertices();
            let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
            system.add_vertex_buffer(vertex_buffer);

            let indices = game_object.mesh().indices();
            let index_buffer = glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &indices,
            )
            .unwrap();
            system.add_index_buffer(index_buffer);

            let image = game_object.texture();
            let image_dimensions = image.dimensions();

            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
                &image.to_bytes(),
                image_dimensions,
            );
            let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
            system.add_texture(texture);
        }
    }
    pub fn render(&self, display: &Display, system: &mut System) {
        let mut target = display.draw();
        let vertex_shader_src = r#"
    
         #version 330

         in vec3 position;
         in vec2 tex_coords;
  
         out vec2 v_tex_coords;

         uniform mat4 projection;
         uniform mat4 view;
         uniform mat4 model;

         void main() {
             mat4 view_model = view * model;
             v_tex_coords = tex_coords;
             gl_Position = projection  * view * model * vec4(position, 1.0) ;
         }

"#;
        let fragment_shader_src = r#"

         #version 330

         in vec2 v_tex_coords;

         out vec4 color;

        uniform sampler2D diffuse_tex;

         void main() {
            vec4 diffuse_color = texture(diffuse_tex, v_tex_coords) ;
            if(diffuse_color.a < 0.1)
                discard;
            color = vec4(diffuse_color);
         }
"#;

        let program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        let params = glium::DrawParameters {
            // GO BACK TO THIS
            /*depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::Overwrite,
                write: true,
                ..Default::default()
            },*/
            blend: Blend::alpha_blending(),
            //backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        // Draw game_objects
        let projection = *system.camera().projection().get_projection().as_ref();

        let view = *system.camera().transform().get_view_matrix().as_ref();
        for n in 0..system.game_objects().len() {
            let game_object = &system.game_objects()[n];
            let model = *game_object.transform().get_model_matrix().as_ref();

            let vertex_buffer = &system.vertex_buffers()[n];
            let index_buffer = &system.index_buffers()[n];
            let diffuse_texture = &system.textures()[n];

            target
                .draw(
                    vertex_buffer,
                    index_buffer,
                    &program,
                    &uniform! { model: model, view: view, projection: projection, diffuse_tex: diffuse_texture },
                    &params,
                )
                .unwrap();
        }

        target.finish().unwrap();
    }
    pub fn stop(&self) {}
}
