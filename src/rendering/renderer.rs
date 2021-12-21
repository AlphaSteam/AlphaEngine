extern crate glium;
use epi::NativeTexture;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::game_objects::game_object::{GmObj, GameObject};
use crate::game_objects::implementations::card::Card;
pub use crate::rendering::vertex::Vertex;
use crate::sys::{system::System};
use crate::text::render_text::Text;
pub use crate::window::Window;
use egui::{Frame, Color32, TextStyle, TextureId, Sense};
use egui::epaint::ClippedShape;
use egui_glium::EguiGlium;
use glium::{uniform, BackfaceCullingMode, Blend, Surface};
use glium::backend::glutin::Display;
use itertools::Itertools;


pub struct Renderer {
    last_fps: [f32; 20],
    fps_index: usize,
    ui_textures: HashMap<String,TextureId>,
    card_textures: HashMap<String,TextureId>
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            last_fps: [0.0; 20],
            fps_index: 0,
            ui_textures: HashMap::new(),
            card_textures: HashMap::new(),
        }
    }
    pub fn get_card_ui_textures(&mut self, display : &Display, egui: &mut EguiGlium, card: &Card, identifier: String){
        let default_texture = card.base_properties().animations().current_animation();

        let image = card.base_properties().animations().textures()[default_texture].texture();

        let has_ui_texture = self.ui_textures.get(&identifier);
        match has_ui_texture{
            Some(_)=>(),
            None=>{
                let image_dimensions = image.dimensions();
                let image_raw = glium::texture::RawImage2d::from_raw_rgba( image.clone().into_raw(),
                image_dimensions);
                let texture = glium::texture::SrgbTexture2d::new(display, image_raw).unwrap();

                let glium_texture = std::rc::Rc::new(texture);
                // Allocate egui's texture id for GL texture
                let texture_id = egui.painter_mut().register_native_texture(glium_texture);
                self.card_textures.insert(identifier,texture_id);
            }
        };
    }
    pub fn get_buffers(&mut self, system: &mut System, display : &Display, egui: &mut EguiGlium, object_id: &String, object: &mut Box<dyn GameObject>){
      
            let current_animation = object.as_ref().base_properties().animations().current_animation().clone();


            let has_vertex_buffer = system.vertex_buffers().get(object_id);
            match has_vertex_buffer{
                Some(_)=>(),
                None=>{
                    let shape = object.as_ref().base_properties().meshes()[&current_animation.clone()].vertices();           
                    let vertex_buffer = glium::VertexBuffer::dynamic(display, &shape).unwrap();
                    system.add_vertex_buffer(object_id.clone(), vertex_buffer);
                }
            };
            let has_index_buffer = system.index_buffers().get(object_id);
            match has_index_buffer{
                Some(_)=>(),
                None=>{
                    let indices = object.base_properties().meshes()[&current_animation.clone()].indices();
                    let index_buffer = glium::IndexBuffer::dynamic(
                        display,
                        glium::index::PrimitiveType::TrianglesList,
                        &indices,
                    )
                    .unwrap();
                    system.add_index_buffer(object_id.clone(), index_buffer);
                }
            };
           
           
                let image = object.base_properties().animations().textures()[&current_animation.clone()].texture();
           

                
                if object.base_properties().ui() {
                    let has_ui_texture = self.ui_textures.get(object_id);
                    match has_ui_texture{
                        Some(_)=>(),
                        None=>{
                            let image_dimensions = image.dimensions();
                            let image_raw = glium::texture::RawImage2d::from_raw_rgba( image.clone().into_raw(),
                            image_dimensions);
                            let texture = glium::texture::SrgbTexture2d::new(display, image_raw).unwrap();
            
                            let glium_texture = std::rc::Rc::new(texture);
                            // Allocate egui's texture id for GL texture
                            let texture_id = egui.painter_mut().register_native_texture(glium_texture);
                            self.ui_textures.insert(object_id.to_string(),texture_id);
                        }
                    };
           
                    

                    
                }
                else{
                    let has_texture = system.textures().get(object_id);
                    match has_texture{
                        Some(_)=>(),
                        None=>{
                            let image_dimensions = image.dimensions();
                            let image_raw = glium::texture::RawImage2d::from_raw_rgba_reversed(
                                &image.clone().into_raw(),
                                image_dimensions,
                            );
                            let texture = glium::texture::SrgbTexture2d::new(display, image_raw).unwrap();
                            system.add_texture(object_id.clone(), texture);
                        }
                    };


                }

          
    }
    pub fn start(&mut self, display: &Display,egui: &mut EguiGlium, system: &mut System) {
        let mut game_objects = system.game_objects_mut().game_objects_mut().clone();
        for (object_id, object) in game_objects.iter_mut() {

            self.get_buffers(system, display,egui,object_id, object);

        }

        for (deck_name, deck) in system.decks().iter(){

            for (index,card) in deck.deck.iter().enumerate(){
                let identifier = format!("{}-{}", deck_name, index.to_string());
               self.get_card_ui_textures(display,egui,card, identifier)

            }
            
        }
      
    }
    pub fn render_gui(
        system: &mut System,
        display: &Display,
        egui: &mut EguiGlium,
        fps: f32,
        frame_time: Duration,
        texts: HashMap<String, Text>,
        ui_textures: HashMap<String, TextureId>,
        card_textures: HashMap<String, TextureId>

    ) -> (bool, Vec<ClippedShape>) {
        egui.begin_frame(display);
        
        egui::Window::new("Debug").show(egui.ctx(), |ui| {
            ui.label(format!("Fps: {}", fps));
            ui.label(format!("Frame time: {:?}", frame_time))
        });
        

        for (game_object_id,texture_id) in ui_textures.iter() {
            let game_object =  system.game_objects_mut().get_game_object_mut(game_object_id.clone());
            if game_object.base_properties().should_render(){
                egui::Area::new(game_object_id ).show(egui.ctx(), |ui| {
                    let local_scale = game_object.base_properties().transform().local_scale();
                    let size = [local_scale[0], local_scale[1]];
                    ui.add(egui::Image::new(*texture_id, size).sense(Sense::click_and_drag()));
                    
                });
            }
         

        }
        // Render cards in hand
        for (card_id,texture_id) in card_textures.iter() {
            let identifiers: Vec<&str> = card_id.split("-").collect();
            let deck_id = identifiers[0];
            let card_id: usize = identifiers[1].parse::<usize>().unwrap() as usize;
            let card =  system.decks().get(deck_id).unwrap().deck.get(card_id as usize).unwrap();
            if card.base_properties().should_render(){
                egui::Area::new(format!("{} {}", card_id, "text") ).show(egui.ctx(), |ui| {
                    let local_scale = card.base_properties().transform().local_scale();
                    let size = [local_scale[0], local_scale[1]];
                  
                   ui.add(egui::Label::new(format!("{}",card.clone().cost())) );
                   ui.add(egui::Label::new(format!("{}",card.clone().name())));
                   ui.add(egui::Image::new(*texture_id, size));
                   ui.add(egui::Label::new(format!("{}",card.clone().description())));
                
                    
                });
            }
         

        }
        let mut transparent_frame = Frame::default();
        transparent_frame.fill = Color32::TRANSPARENT;
        for (i, txt) in texts.iter() {
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
        let mut game_objects = system.game_objects_mut().game_objects_mut().clone();
        for (object_id, object) in game_objects.iter_mut() {

            self.get_buffers(system, display,egui,object_id, object);

        }
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

            if game_object.base_properties().should_render() && !game_object.base_properties().ui(){
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
        let ui_textures = self.ui_textures.clone();
        let card_textures = self.card_textures.clone();
        let (_needs_repaint, shapes) = Self::render_gui(system, display, egui, fps_mean, time_since_render,texts, ui_textures, card_textures );
        egui.paint(&display, &mut target, shapes);
        
        target.finish().unwrap();
    }
    pub fn stop(&self) {}
}
