use std::cell::Cell;
use wasmer_runtime::{func, imports};
use wasmer_runtime_core::{
    import::ImportObject,
    memory::ptr::{Array, WasmPtr},
    vm::Ctx,
};

fn log_message(ctx: &mut Ctx, str_ptr: WasmPtr<u8, Array>, str_len: u32) {
    let memory = ctx.memory(0);
    let slice = match str_ptr.deref(memory, 0, str_len) {
        Some(v) => unsafe { &*(v as *const [Cell<u8>] as *const [u8]) },
        None => {
            eprintln!("Error dereferencing pointer from Wasm memory");
            return;
        }
    };
    let wasm_str = match std::str::from_utf8(slice) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error reading string from Wasm memory: {}", e);
            return;
        }
    };

    println!("[user_script]: \"{}\"", wasm_str);
}

pub fn common_imports() -> ImportObject {
    imports! {
        "env" => {
            "log_message" => func!(log_message),
        },
    }
}
