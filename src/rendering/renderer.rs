extern crate glium;

use std::time::{Duration, Instant};

pub use crate::rendering::vertex::Vertex;
use crate::sys::{system::System};
use crate::text::render_text::Text;
pub use crate::window::Window;
use egui::{Frame, Color32, FontDefinitions, TextStyle, FontFamily, Style};
use egui::epaint::ClippedShape;
use egui_glium::EguiGlium;
use glium::{uniform, BackfaceCullingMode, Blend, Surface};
use glium::backend::glutin::Display;
use itertools::Itertools;



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
        let mut game_objects = system.game_objects_mut().clone();
        let game_objects = game_objects.game_objects_mut();
        for (game_object_id, game_object) in game_objects.iter() {
            let default_texture = game_object.as_ref().base_properties().animations().current_animation();
            let shape = game_object.as_ref().base_properties().meshes()[default_texture].vertices();           
            let vertex_buffer = glium::VertexBuffer::dynamic(display, &shape).unwrap();
            system.add_vertex_buffer(game_object_id.clone(), vertex_buffer);

            let indices = game_object.base_properties().meshes()[default_texture].indices();
            let index_buffer = glium::IndexBuffer::dynamic(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &indices,
            )
            .unwrap();
            system.add_index_buffer(game_object_id.clone(), index_buffer);

            let image = game_object.base_properties().animations().textures()[default_texture].texture();
            let image_dimensions = image.dimensions();
            let image_raw = glium::texture::RawImage2d::from_raw_rgba_reversed(
                &image.clone().into_raw(),
                image_dimensions,
            );

            let texture = glium::texture::SrgbTexture2d::new(display, image_raw).unwrap();
            system.add_texture(game_object_id.clone(), texture);
        }
  
      
    }
    pub fn render_gui(
        display: &Display,
        egui: &mut EguiGlium,
        fps: f32,
        frame_time: Duration,
        texts: Vec<Text>
    ) -> (bool, Vec<ClippedShape>) {
        egui.begin_frame(display);
        
        egui::Window::new("Debug").show(egui.ctx(), |ui| {
            ui.label(format!("Fps: {}", fps));
            ui.label(format!("Frame time: {:?}", frame_time))
        });

        let mut transparent_frame = Frame::default();
        transparent_frame.fill = Color32::TRANSPARENT;
        for (i, txt) in texts.iter().enumerate() {
            let pos_x = txt.position[0];
            let pos_y = txt.position[1];
            let r = txt.color[0];
            let g = txt.color[1];
            let b = txt.color[2];

        

       
            egui::Area::new(format!("Text {}", i)).fixed_pos(egui::pos2(pos_x, pos_y)).show(egui.ctx(), |ui| {
              
                ui.add(egui::Label::new(format!("{}", txt.text)).text_style(TextStyle::Heading).text_color(Color32::from_rgb(r,g,b)));
            });
        }
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
        let delta_time = 1.0 / fps_raw;
        for (_game_object_id, game_object) in system.game_objects_mut().game_objects_mut().iter_mut() {
            game_object
                .base_properties_mut()
                .transform_mut()
                .delta_time = delta_time;
        }
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
        let mut game_objects = system.game_objects_mut().clone();
        let game_objects = game_objects.game_objects_mut();
        for (_, game_object) in system.game_objects_mut().game_objects_mut().iter_mut(){
            game_object.base_properties_mut().animations_mut().run(delta_time);

        }
        for (game_object_id, game_object) in game_objects.iter_mut().sorted_by(|a,b|
            {
            let first_z_index = &a.1.base_properties()
            .transform().z_index();

            let second_z_index = &b.1.base_properties()
            .transform().z_index();
            Ord::cmp(first_z_index,second_z_index)
        }
       
    ) {  

            if game_object.base_properties().should_render(){
                let model = *game_object
                .base_properties()
                .transform()
                .get_model_matrix()
                .as_ref();

            let vertex_buffer = &system.vertex_buffers()[game_object_id];
            let shape = game_object.as_ref().base_properties().animations().get_current_frame();
            vertex_buffer.write(&shape);
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
           
        }
        let texts = system.text_mut().clone();

        let (_needs_repaint, shapes) = Self::render_gui(display, egui, fps_mean, time_since_render,texts);
        egui.paint(&display, &mut target, shapes);
        
        target.finish().unwrap();
    }
    pub fn stop(&self) {}
}
