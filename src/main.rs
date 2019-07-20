#[macro_use]
extern crate log;

mod components;
mod draw;
mod level;
mod logging;
mod shapes;
mod systems;
use draw::{SdlEvent, SdlRender};
use level::*;

fn main() {
    logging::set_up_logging().unwrap();
    let mut render_ctx = match SdlRender::new() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error creating SDL window: {}", e);
            std::process::exit(-1);
        }
    };

    info!("Renderer set up");
    let mut current_level: Box<dyn Level> = Box::new(level1::Level1::new());

    'mainloop: loop {
        let events = render_ctx.get_events();
        for event in &events {
            match event {
                SdlEvent::QuitRequested => {
                    info!("Quitting game");
                    break 'mainloop;
                }
                SdlEvent::WasmUpload(filename) => {
                    info!("loading Wasm from file {}", filename);
                    // TODO: make this async
                    use std::io::Read;
                    let mut bytes = vec![];
                    let mut file = std::fs::File::open(&filename).unwrap();
                    file.read_to_end(&mut bytes).unwrap();
                    current_level.load_wasm(&bytes[..]).unwrap();
                }
            }
        }

        current_level.step_time();
        let world = current_level.borrow_world();
        render_ctx.draw(world);
    }
}
