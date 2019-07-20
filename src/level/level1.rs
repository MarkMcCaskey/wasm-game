use crate::components::{Color, Geometry, Pos, Vel};
use crate::level::common::*;
use crate::level::*;
use crate::shapes::Shape;
use crate::systems::{Movement, PlayerInput};
use num_traits::FromPrimitive;
use specs::prelude::*;
use std::ffi::c_void;
use std::sync::mpsc::{Receiver, Sender};
use wasmer_runtime::{func, imports, instantiate};
use wasmer_runtime_core::vm::Ctx;

pub struct Level1<'a> {
    world: World,
    dispatcher: Dispatcher<'a, 'a>,
    data_inner: Option<Level1InnerData>,
    mailbox: Option<Receiver<LevelMessage>>,
}

impl<'a> Level1<'a> {
    pub fn new() -> Self {
        let mut world = World::new();
        register_rendering_components(&mut world);
        let mut dispatcher = DispatcherBuilder::new()
            .with(PlayerInput, "player input", &[])
            .with(Movement, "movement", &["player input"])
            .build();

        dispatcher.setup(&mut world);

        let mut level1 = Self {
            world,
            dispatcher,
            data_inner: None,
            mailbox: None,
        };
        level1.reset();

        level1
    }

    pub fn reset(&mut self) {
        self.world.delete_all();
        self.world
            .create_entity()
            .with(Vel(0.025, 0.025))
            .with(Pos(100., 100.))
            .with(Geometry(Shape::Rectangle {
                height: 25,
                width: 25,
            }))
            .with(Color((0, 255, 0, 255)))
            .build();

        self.world
            .create_entity()
            .with(Vel(0., 0.))
            .with(Pos(200., 200.))
            .with(Geometry(Shape::Rectangle {
                height: 25,
                width: 25,
            }))
            .with(Color((0, 0, 0, 255)))
            .build();
    }
}

fn get_level1_inner_data(ctx: &mut Ctx) -> &Level1InnerData {
    unsafe { &*(ctx.data as *const Level1InnerData) }
}

fn apply_force_on_player_in_direction(ctx: &mut Ctx, direction: u32) {
    let dir = match FromPrimitive::from_u32(direction) {
        Some(v) => v,
        None => {
            error!("Error reading direction value: {}", direction);
            return;
        }
    };
    let data = get_level1_inner_data(ctx);

    data.send_message(LevelMessage::PlayerMoveRequest { direction: dir });
}

/// This is the data that lives in the Wasm context that the functions will interact with
#[derive(Debug)]
pub struct Level1InnerData {
    pub sender: Sender<LevelMessage>,
}

impl Level1InnerData {
    pub fn send_message(&self, msg: LevelMessage) {
        trace!("Sending message {:?}", msg);
        if let Err(e) = self.sender.send(msg.clone()) {
            error!("Failed to send message: {:?}: {}", msg, e.to_string())
        }
    }
}

impl<'a> Level for Level1<'a> {
    fn load_wasm(&mut self, wasm_bytes: &[u8]) -> Result<(), String> {
        let common_imports = common_imports();
        let mut imports = imports! {
            "env" => {
                "apply_force_on_player_in_direction" => func!(apply_force_on_player_in_direction),
            },
        };
        imports.extend(common_imports);

        let mut instance = instantiate(&wasm_bytes[..], &imports).map_err(|e| e.to_string())?;
        let (sender, recv) = std::sync::mpsc::channel();
        self.mailbox = Some(recv);
        let data = Level1InnerData { sender };
        self.data_inner = Some(data);
        let data_ptr = &mut self.data_inner as *mut _ as *mut c_void;
        instance.context_mut().data = data_ptr;

        let entry_point = instance.func::<(), ()>("level_entrypoint").unwrap();
        let _ = entry_point.call().expect("failed to execute");
        Ok(())
    }

    fn step_time(&mut self) {
        let messages = self
            .mailbox
            .as_ref()
            .map(|r| {
                let mut msgs = vec![];
                while let Ok(msg) = r.try_recv() {
                    msgs.push(msg);
                }
                LevelMessages(msgs)
            })
            .unwrap_or_default();

        if !messages.0.is_empty() {
            trace!("Sending messages: {:?}", &messages);
        }
        self.world.insert(messages);
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }

    fn borrow_world(&self) -> &World {
        &self.world
    }

    fn reset(&mut self) {
        self.reset();
    }
}
