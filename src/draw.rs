use crate::components::{self, *};
use crate::shapes::Shape;
use sdl2::pixels::Color;
use specs::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum SdlEvent {
    QuitRequested,
    WasmUpload(String),
}

pub struct SdlRender {
    context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl SdlRender {
    pub fn new() -> Result<SdlRender, String> {
        let context = sdl2::init()?;
        let v_subsystem = context.video()?;
        let window = v_subsystem
            .window("Wasm game", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(SdlRender { context, canvas })
    }

    pub fn get_events(&mut self) -> Vec<SdlEvent> {
        use sdl2::event::Event;

        let mut events = vec![];
        for event in self.context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => events.push(SdlEvent::QuitRequested),
                Event::DropFile { filename, .. } => events.push(SdlEvent::WasmUpload(filename)),
                _ => {}
            }
        }

        events
    }

    pub fn draw(&mut self, world: &World) {
        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        self.canvas.clear();

        let poss = world.read_storage::<Pos>();
        let geoms = world.read_storage::<Geometry>();
        let colors = world.read_storage::<components::Color>();

        for (pos, geom, color) in (&poss, &geoms, &colors).join() {
            match geom.0 {
                Shape::Rectangle { width, height } => {
                    let sdl_rect = sdl2::rect::Rect::new(pos.1 as i32, pos.0 as i32, width, height);
                    self.canvas.set_draw_color(color.0);
                    self.canvas.fill_rect(sdl_rect).unwrap();
                }
                Shape::Circle { .. } => {}
            }
        }

        self.canvas.present();
    }
}
