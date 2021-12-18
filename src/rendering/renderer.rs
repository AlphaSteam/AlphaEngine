extern crate glium;

use std::time::{Duration, Instant};

pub use crate::rendering::vertex::Vertex;
use crate::sys::{system::System};
pub use crate::window::Window;
use egui::epaint::ClippedShape;
use egui_glium::EguiGlium;
use glium::{uniform, BackfaceCullingMode, Blend, Display, Surface, VertexBuffer};
use image::GenericImageView;

pub struct Renderer {
    last_fps: [f32; 20],
    fps_index: usize,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            last_fps: [0.0; 20],
            fps_index: 0,
        }
    }
    pub fn start(&self, display: &Display, system: &mut System) {
        let game_object_mutex = system.game_objects_mut().clone();
        let mut game_objects = game_object_mutex;
        let game_objects = game_objects.game_objects_mut();
        for (game_object_id, game_object) in game_objects.iter() {
            let shape = game_object.as_ref().get_base_properties().mesh().vertices();
            let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
            system.add_vertex_buffer(game_object_id.clone(), vertex_buffer);

            let indices = game_object.get_base_properties().mesh().indices();
            let index_buffer = glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &indices,
            )
            .unwrap();
            system.add_index_buffer(game_object_id.clone(), index_buffer);

            let image = game_object.get_base_properties().texture();
            let image_dimensions = image.dimensions();
            let image_raw = glium::texture::RawImage2d::from_raw_rgba_reversed(
                &image.clone().into_bytes(),
                image_dimensions,
            );

            let texture = glium::texture::SrgbTexture2d::new(display, image_raw).unwrap();
            system.add_texture(game_object_id.clone(), texture);
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
    pub fn render_gui(
        display: &Display,
        egui: &mut EguiGlium,
        fps: f32,
        frame_time: Duration,
    ) -> (bool, Vec<ClippedShape>) {
        egui.begin_frame(&display);

        egui::Window::new("Debug").show(egui.ctx(), |ui| {
            ui.label(format!("Fps: {}", fps));
            ui.label(format!("Frame time: {:?}", frame_time))
        });

        let (needs_repaint, shapes) = egui.end_frame(&display);
        return (needs_repaint, shapes);
    }
    pub fn render(
        &mut self,
        display: &Display,
        egui: &mut EguiGlium,
        system: &mut System,
        old_render: &mut Instant,
    ) {
        let time_since_render = Instant::now().duration_since(*old_render);
        self.last_fps[self.fps_index] = time_since_render.as_nanos() as f32;
        if self.fps_index < 19 {
            self.fps_index += 1;
        } else {
            self.fps_index = 0;
        }
        let fps_mean = self.last_fps.iter().sum::<f32>() / 20.0;
        let fps_mean = (1.0 / (fps_mean / 1_000_000_000.0)).round();

        let fps_raw = (1.0 / (time_since_render.as_nanos() as f32 / 1_000_000_000.0)).round();
        for (_game_object_id, game_object) in system.game_objects_mut().game_objects_mut().iter_mut() {
            game_object
                .get_base_properties_mut()
                .transform_mut()
                .delta_time = 1.0 / fps_raw;
        }
        let (_needs_repaint, shapes) = Self::render_gui(display, egui, fps_mean, time_since_render);
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
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::Overwrite,
                write: true,
                ..Default::default()
            },
            blend: Blend::alpha_blending(),
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        // Draw game_objects
        let projection = *system.camera().projection().get_projection().as_ref();

        let view = *system.camera().transform().get_view_matrix().as_ref();
        let game_object_mutex = system.game_objects_mut().clone();
        let mut game_objects = game_object_mutex;

        let game_objects = game_objects.game_objects_mut();
        for (game_object_id, game_object) in game_objects.iter() {
            let model = *game_object
                .get_base_properties()
                .transform()
                .get_model_matrix()
                .as_ref();

            let vertex_buffer = &system.vertex_buffers()[game_object_id];
            let index_buffer = &system.index_buffers()[game_object_id];
            let diffuse_texture = &system.textures()[game_object_id];

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
        egui.paint(&display, &mut target, shapes);
        // Draw text
        /* let program = glium::Program::from_source(
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
        } */
        target.finish().unwrap();
    }
    pub fn stop(&self) {}
}
