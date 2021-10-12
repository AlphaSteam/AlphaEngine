extern crate glium;
pub use crate::rendering::vertex::Vertex;
pub use crate::window::Window;
use crate::{
    shaders::Shader,
    sys::{
        cam::{projection::Projection, projection_ortho::ProjectionOrtho},
        system::System,
    },
};
use glium::{uniform, BackfaceCullingMode, Blend, Display, Surface, VertexBuffer};
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
        let mut text_buffers = Vec::<(VertexBuffer<Vertex>, char)>::new();

        let texts = system.text_mut().clone();
        for txt in texts {
            let text = &txt.text;
            let x = &txt.position[0];
            let y = &txt.position[1];

            for c in text.chars() {
                let font = &system.fonts()[&txt.font];
                let char = &font.characters()[&c];

                let xpos = x + char.bearing[0] * txt.scale[0];
                let ypos = y - (char.size.1 as f32 - char.bearing[1]) * txt.scale[1];

                let w = char.size.0 as f32 * txt.scale[0];
                let h = char.size.1 as f32 * txt.scale[1];

                let vertices = vec![
                    Vertex {
                        position: [xpos, ypos + h, 0.0],
                        tex_coords: [0.0, 0.0],
                    },
                    Vertex {
                        position: [xpos, ypos, 0.0],
                        tex_coords: [0.0, 1.0],
                    },
                    Vertex {
                        position: [xpos + w, ypos, 0.0],
                        tex_coords: [1.0, 1.0],
                    },
                    Vertex {
                        position: [xpos + w, ypos + h, 0.0],
                        tex_coords: [1.0, 0.0],
                    },
                ];

                let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
                text_buffers.push((vertex_buffer, c));
            }
        }
        for text_buffer in text_buffers {
            system.add_text_buffer(text_buffer.0, text_buffer.1);
        }
    }
    pub fn render(&self, display: &Display, system: &mut System) {
        let mut target = display.draw();

        let program = glium::Program::from_source(
            display,
            system.current_shader().source_code().0.as_str(),
            system.current_shader().source_code().1.as_str(),
            None,
        )
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
            backface_culling: BackfaceCullingMode::CullClockwise,
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

        // Draw text
        let program = glium::Program::from_source(
            display,
            Shader::Text.source_code().0.as_str(),
            Shader::Text.source_code().1.as_str(),
            None,
        )
        .unwrap();
        let window_resolution = system.get_window_resolution();
        let projection = *ProjectionOrtho::new(
            0.0,
            window_resolution[0],
            0.0,
            window_resolution[1],
            -1.0,
            1.0,
        )
        .get_projection()
        .as_ref();
        for n in 0..system.text().len() {
            let vertex_buffer = &system.text_buffers()[n].0;

            let index_buffer = &glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &vec![0_u32, 1, 2, 3, 0, 2],
            )
            .unwrap();
            let text = &system.text()[n];
            let c = &system.text_buffers()[n].1;
            let font = &system.fonts()[&text.font];
            let char = &font.characters()[&c];

            let texture = &char.texture;

            target
                .draw(
                    vertex_buffer,
                    index_buffer,
                    &program,
                    &uniform! { projection: projection, text: texture, text_colors: text.color},
                    &params,
                )
                .unwrap();
        }
        target.finish().unwrap();
    }
    pub fn stop(&self) {}
}
