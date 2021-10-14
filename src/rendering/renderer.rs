extern crate glium;
use std::usize::MAX;

pub use crate::rendering::vertex::Vertex;
pub use crate::window::Window;
use crate::{
    shaders::Shader,
    sys::{
        cam::{projection::Projection, projection_ortho::ProjectionOrtho},
        system::System,
    },
    window,
};
use ab_glyph::{point, FontRef, Rect};
use glium::texture::{buffer_texture::BufferTextureType, Texture1d, Texture1dDataSink};
use glium::{
    texture::buffer_texture::BufferTexture, uniform, BackfaceCullingMode, Blend, Display, Surface,
    VertexBuffer,
};
use glyph_brush::{
    ab_glyph::FontArc, BrushAction, BrushError, GlyphBrush, GlyphBrushBuilder, Section, Text,
};
use image::GenericImageView;

/// The texture used to cache drawn glyphs

pub struct Renderer {
    texture: BufferTexture<u8>,
    vertex_buffer: VertexBuffer<Vertex>,
    glyph_brush: GlyphBrush<Vec<Vertex>>,
}

impl Renderer {
    pub fn new(display: Display) -> Renderer {
        let arial = FontArc::try_from_slice(include_bytes!("/home/alphasteam/Escritorio/Dev/Motor de videojuegos/alpha_engine/src/fonts/ArialCE.ttf")).unwrap();
        let mut glyph_brush: GlyphBrush<Vec<Vertex>> = GlyphBrushBuilder::using_font(arial).build();
        let mut texture =
            BufferTexture::empty_dynamic(&display, 4352, BufferTextureType::Float).unwrap();
        let mut vertex_buffer = VertexBuffer::empty_dynamic(&display, 4352).unwrap();
        Renderer {
            texture,
            vertex_buffer,
            glyph_brush,
        }
    }
    pub fn start(&mut self, display: &Display, system: &mut System) {
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
        let display_clone = display.clone();
        let gl_window = display_clone.gl_window();
        let window = gl_window.window();
        let dimensions = window.inner_size();
        let width = dimensions.width as f32;
        let height = dimensions.height as f32;
        let font_size = 18.0;
        let scale = (font_size * window.scale_factor() as f32).round();
        let base_text = Text::new(&"test").with_scale(scale);
        // Queue up all sections of text to be drawn
        let mut glyph_brush = &mut self.glyph_brush;
        glyph_brush.queue(
            Section::default()
                .add_text(base_text.with_color([0.9, 0.3, 0.3, 1.0]))
                .with_bounds((width / 3.15, height)),
        );
    }
    pub fn render(&mut self, display: &Display, system: &mut System) {
        let mut target = display.draw();
        let mut glyph_brush = &mut self.glyph_brush;
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
        let arial = FontRef::try_from_slice(include_bytes!("/home/alphasteam/Escritorio/Dev/Motor de videojuegos/alpha_engine/src/fonts/ArialCE.ttf")).unwrap();
        let mut brush_action;
        let max_image_dimension = {
            let mut value = 0;
            value = MAX;
            value as u32
        };
        let mut texture = &self.texture;
        loop {
            brush_action = glyph_brush.process_queued(
                |rect, tex_data| {
                    println!("{}", tex_data.len());
                    texture.write(tex_data);
                },
                Self::to_vertex,
            );
            match brush_action {
                Ok(_) => break,
                Err(BrushError::TextureTooSmall { suggested, .. }) => {
                    let (new_width, new_height) = if (suggested.0 > max_image_dimension
                        || suggested.1 > max_image_dimension)
                        && (glyph_brush.texture_dimensions().0 < max_image_dimension
                            || glyph_brush.texture_dimensions().1 < max_image_dimension)
                    {
                        (max_image_dimension, max_image_dimension)
                    } else {
                        suggested
                    };
                    eprint!("\r                            \r");
                    eprintln!("Resizing glyph texture -> {}x{}", new_width, new_height);

                    // Recreate texture as a larger size to fit more
                    let texture: BufferTexture<u8> =
                        BufferTexture::empty_dynamic(display, 4352, BufferTextureType::Float)
                            .unwrap();

                    glyph_brush.resize_texture(new_width, new_height);
                }
            }
        }
        match brush_action.unwrap() {
            BrushAction::Draw(vertices) => {
                let vertices = &*vertices.clone()[0];
                self.vertex_buffer.write(&vertices);
            }
            BrushAction::ReDraw => {}
        }
        let index_buffer = &glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &vec![0_u32, 1, 2, 3, 0, 2],
        )
        .unwrap();
        // Draw the text to the screen

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
        target
            .draw(
                &self.vertex_buffer,
                index_buffer,
                &program,
                &uniform! { projection: projection, text:texture , text_colors: [1.0,1.0,1.0]},
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    }
    #[inline]
    pub fn to_vertex(
        glyph_brush::GlyphVertex {
            mut tex_coords,
            pixel_coords,
            bounds,
            extra,
        }: glyph_brush::GlyphVertex,
    ) -> Vec<Vertex> {
        let gl_bounds = bounds;

        let mut gl_rect = Rect {
            min: point(pixel_coords.min.x as f32, pixel_coords.min.y as f32),
            max: point(pixel_coords.max.x as f32, pixel_coords.max.y as f32),
        };

        // handle overlapping bounds, modify uv_rect to preserve texture aspect
        if gl_rect.max.x > gl_bounds.max.x {
            let old_width = gl_rect.width();
            gl_rect.max.x = gl_bounds.max.x;
            tex_coords.max.x = tex_coords.min.x + tex_coords.width() * gl_rect.width() / old_width;
        }
        if gl_rect.min.x < gl_bounds.min.x {
            let old_width = gl_rect.width();
            gl_rect.min.x = gl_bounds.min.x;
            tex_coords.min.x = tex_coords.max.x - tex_coords.width() * gl_rect.width() / old_width;
        }
        if gl_rect.max.y > gl_bounds.max.y {
            let old_height = gl_rect.height();
            gl_rect.max.y = gl_bounds.max.y;
            tex_coords.max.y =
                tex_coords.min.y + tex_coords.height() * gl_rect.height() / old_height;
        }
        if gl_rect.min.y < gl_bounds.min.y {
            let old_height = gl_rect.height();
            gl_rect.min.y = gl_bounds.min.y;
            tex_coords.min.y =
                tex_coords.max.y - tex_coords.height() * gl_rect.height() / old_height;
        }
        vec![
            Vertex {
                position: [gl_rect.min.x, gl_rect.min.y, 0.0],
                tex_coords: [tex_coords.min.x, tex_coords.min.y],
            },
            Vertex {
                position: [gl_rect.min.x, gl_rect.max.y, 0.0],
                tex_coords: [tex_coords.min.x, tex_coords.max.y],
            },
            Vertex {
                position: [gl_rect.max.x, gl_rect.max.y, 0.0],
                tex_coords: [tex_coords.max.x, tex_coords.max.y],
            },
            Vertex {
                position: [gl_rect.max.x, gl_rect.min.y, 0.0],
                tex_coords: [tex_coords.max.x, tex_coords.min.y],
            },
        ]
    }
    pub fn stop(&self) {}
}
